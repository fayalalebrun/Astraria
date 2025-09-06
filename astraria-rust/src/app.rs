use anyhow::Result;
/// Main application structure that coordinates all subsystems
/// Replaces the LibGDX Game class from the original implementation
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use crate::{
    AstrariaResult, assets::AssetManager, input::InputHandler, physics::PhysicsSimulation,
    renderer::Renderer, scenario::BodyType, ui::UserInterface,
};

pub struct AstrariaApp {
    window: Option<Window>,
    renderer: Option<Renderer>,
    physics: Option<PhysicsSimulation>,
    ui: Option<UserInterface>,
    input_handler: Option<InputHandler>,
    asset_manager: Option<AssetManager>,
    last_frame_time: std::time::Instant,
    scenario_file: String,
    focus_body_index: usize,
}

impl AstrariaApp {
    pub fn new() -> Result<Self> {
        Self::new_with_scenario("Solar_System_2K.txt".to_string())
    }

    pub fn new_with_scenario(scenario_file: String) -> Result<Self> {
        Self::new_with_scenario_and_focus(scenario_file, 0)
    }

    pub fn new_with_scenario_and_focus(
        scenario_file: String,
        focus_body_index: usize,
    ) -> Result<Self> {
        Ok(Self {
            window: None,
            renderer: None,
            physics: None,
            ui: None,
            input_handler: None,
            asset_manager: None,
            last_frame_time: std::time::Instant::now(),
            scenario_file,
            focus_body_index,
        })
    }

    pub fn run(mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut self)?;
        Ok(())
    }

    async fn initialize(&mut self, window: &Window) -> AstrariaResult<()> {
        log::info!("Initializing Astraria application...");

        // Initialize asset manager first
        self.asset_manager = Some(AssetManager::new().await?);

        // Initialize renderer
        let renderer = Renderer::new(window, self.asset_manager.as_mut().unwrap()).await?;
        self.renderer = Some(renderer);

        // Load atmospheric assets required by shaders after renderer is ready
        if let (Some(asset_manager), Some(renderer)) = (&mut self.asset_manager, &self.renderer) {
            asset_manager
                .load_atmospheric_assets(renderer.device(), renderer.queue())
                .await?;
        }

        // Initialize physics simulation
        self.physics = Some(PhysicsSimulation::new());

        // Initialize input handler
        self.input_handler = Some(InputHandler::new());

        // Initialize UI
        let ui = UserInterface::new(window, self.renderer.as_ref().unwrap())?;
        self.ui = Some(ui);

        // Load default scenario if available
        self.load_default_scenario().await?;

        // Position camera to focus on the specified body
        self.position_camera_on_focus_body().await?;

        log::info!("Astraria initialization complete!");
        Ok(())
    }

    async fn load_default_scenario(&mut self) -> AstrariaResult<()> {
        // Try to load the specified scenario file
        log::info!(
            "App: Attempting to load scenario file: {}",
            self.scenario_file
        );
        if let Some(asset_manager) = &mut self.asset_manager {
            if let Ok(scenario_data) = asset_manager.load_scenario(&self.scenario_file).await {
                log::info!("App: Successfully loaded scenario data, parsing...");

                // Parse the scenario
                let scenario = crate::scenario::ScenarioParser::parse(&scenario_data)?;
                log::info!("App: Parsed scenario with {} bodies", scenario.bodies.len());

                // Load textures for all bodies in the scenario
                if let Some(renderer) = &mut self.renderer {
                    log::info!("App: Loading scenario textures...");
                    let (textures_before, models_before, cubemaps_before) =
                        asset_manager.cache_stats();
                    log::info!(
                        "App: AssetManager cache before scenario textures: textures={}, models={}, cubemaps={}",
                        textures_before,
                        models_before,
                        cubemaps_before
                    );

                    renderer
                        .main_renderer()
                        .load_scenario_textures(asset_manager, &scenario)
                        .await?;

                    let (textures_after, models_after, cubemaps_after) =
                        asset_manager.cache_stats();
                    log::info!(
                        "App: AssetManager cache after scenario textures: textures={}, models={}, cubemaps={}",
                        textures_after,
                        models_after,
                        cubemaps_after
                    );
                    log::info!(
                        "App: Scenario textures loaded successfully - added {} textures",
                        textures_after - textures_before
                    );
                } else {
                    log::error!("App: Renderer not initialized when loading scenario textures");
                }

                // Pass the raw scenario data to physics (it will re-parse, but that's OK for now)
                if let Some(physics) = &mut self.physics {
                    physics.load_scenario(scenario_data)?;
                    log::info!("App: Physics loaded scenario: {}", self.scenario_file);
                } else {
                    log::error!("App: Physics system not initialized when loading scenario");
                }
            } else {
                log::warn!(
                    "App: Could not load scenario '{}', starting with empty simulation",
                    self.scenario_file
                );
            }
        } else {
            log::error!("App: Asset manager not initialized when loading scenario");
        }
        Ok(())
    }

    async fn position_camera_on_focus_body(&mut self) -> AstrariaResult<()> {
        if let (Some(physics), Some(renderer)) = (&self.physics, &mut self.renderer) {
            let bodies = physics.get_bodies()?;
            if let Some(focus_body) = bodies.get(self.focus_body_index) {
                log::info!(
                    "Positioning camera to focus on body '{}' at index {}",
                    focus_body.name,
                    self.focus_body_index
                );

                // Calculate camera distance based on body radius
                let radius = match &focus_body.body_type {
                    BodyType::Planet { radius, .. } => radius,
                    BodyType::Star { radius, .. } => radius,
                    BodyType::PlanetAtmo { radius, .. } => radius,
                    BodyType::BlackHole { radius } => radius,
                };

                // Position camera at 3x radius distance for good view
                let camera_distance = radius * 3.0;
                let body_position = focus_body.position;

                // Use simplified look_at with distance parameter
                renderer.set_camera_look_at(body_position, camera_distance as f64);

                log::info!(
                    "Camera positioned at distance {:.2e} looking at '{}' at ({:.2e}, {:.2e}, {:.2e})",
                    camera_distance,
                    focus_body.name,
                    body_position.x,
                    body_position.y,
                    body_position.z
                );
            } else {
                log::warn!(
                    "Focus body index {} is out of range, using default camera position",
                    self.focus_body_index
                );
            }
        }
        Ok(())
    }

    fn update(&mut self, delta_time: f32) -> AstrariaResult<()> {
        // Update physics simulation
        if let Some(physics) = &mut self.physics {
            physics.update(delta_time)?;
        }

        // Update input handler
        if let Some(input_handler) = &mut self.input_handler {
            input_handler.update(delta_time);

            // Handle camera input and update camera movement
            if let Some(renderer) = &mut self.renderer {
                renderer.handle_camera_input(input_handler, delta_time)?;
                renderer.update_camera(delta_time);
            }
        }

        // Update UI
        if let Some(ui) = &mut self.ui {
            ui.update(delta_time, self.physics.as_ref(), &mut self.renderer)?;

            // Handle UI actions
            let actions = ui.take_actions();
            for action in actions {
                self.handle_ui_action(action)?;
            }
        }

        Ok(())
    }

    fn handle_ui_action(&mut self, action: crate::ui::UiAction) -> AstrariaResult<()> {
        use crate::ui::UiAction;

        match action {
            UiAction::FocusCameraOnObject {
                object_index,
                position,
                radius,
            } => {
                log::info!(
                    "Focusing camera on object {} at position ({:.2e}, {:.2e}, {:.2e}) with radius {:.2e}",
                    object_index,
                    position.x,
                    position.y,
                    position.z,
                    radius
                );

                if let Some(renderer) = &mut self.renderer {
                    // Position camera at 3x radius distance for good view (matching Java behavior)
                    let camera_distance = (radius * 3.0).max(1000.0); // Minimum 1000m distance
                    renderer.set_camera_look_at(position, camera_distance);

                    log::info!(
                        "Camera positioned at distance {:.2e} meters looking at object",
                        camera_distance
                    );
                }
            }
            UiAction::ClearCameraFocus => {
                log::info!("Clearing camera focus - camera now in free mode");
                // Camera focus is cleared - user can now move freely
                // No specific action needed as the camera will respond to user input
            }
        }

        Ok(())
    }

    fn render(&mut self) -> AstrariaResult<()> {
        if let (Some(renderer), Some(physics), Some(asset_manager), Some(ui), Some(window)) = (
            &mut self.renderer,
            &self.physics,
            &self.asset_manager,
            &mut self.ui,
            &self.window,
        ) {
            renderer.begin_frame()?;

            // Render 3D scene
            renderer.render_scene(physics, asset_manager)?;

            // Prepare and render UI overlay
            let (screen_descriptor, clipped_primitives) =
                ui.prepare(renderer, window, Some(physics))?;
            renderer.render_ui_overlay(ui, &clipped_primitives, &screen_descriptor)?;

            renderer.end_frame()?;
        }

        Ok(())
    }

    fn handle_window_event(
        &mut self,
        event: &WindowEvent,
        window: &Window,
    ) -> AstrariaResult<bool> {
        // Let input handler process camera-related events first
        if let Some(input_handler) = &mut self.input_handler {
            // Give input handler priority for mouse events (needed for camera controls)
            match event {
                WindowEvent::MouseInput { .. }
                | WindowEvent::CursorMoved { .. }
                | WindowEvent::MouseWheel { .. } => {
                    if input_handler.handle_event(event)? {
                        return Ok(true); // Event consumed by input handler
                    }
                }
                _ => {}
            }
        }

        // Let UI handle the event next (but only if input handler didn't consume it)
        if let Some(ui) = &mut self.ui {
            if ui.handle_event(event, window)? {
                return Ok(true); // Event consumed by UI
            }
        }

        // Finally let input handler process keyboard events
        if let Some(input_handler) = &mut self.input_handler {
            if input_handler.handle_event(event)? {
                return Ok(true); // Event consumed by input handler
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                log::info!("Close requested, shutting down...");
                return Ok(false);
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(*physical_size)?;
                }
                if let Some(ui) = &mut self.ui {
                    ui.resize(*physical_size)?;
                }
            }
            _ => {}
        }

        Ok(true)
    }
}

impl ApplicationHandler for AstrariaApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Astraria - 3D Orbital Mechanics Simulator")
                .with_inner_size(PhysicalSize::new(1280, 720))
                .with_min_inner_size(PhysicalSize::new(800, 600));

            match event_loop.create_window(window_attributes) {
                Ok(window) => {
                    if let Err(e) = pollster::block_on(self.initialize(&window)) {
                        log::error!("Failed to initialize application: {e}");
                        event_loop.exit();
                        return;
                    }
                    self.window = Some(window);
                    self.last_frame_time = std::time::Instant::now();
                }
                Err(e) => {
                    log::error!("Failed to create window: {e}");
                    event_loop.exit();
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        _event: winit::event::DeviceEvent,
    ) {
        // Handle device events if needed
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = &self.window {
            if window.id() != window_id {
                return;
            }
        }

        if matches!(event, WindowEvent::RedrawRequested) {
            let current_time = std::time::Instant::now();
            let delta_time = current_time
                .duration_since(self.last_frame_time)
                .as_secs_f32();
            self.last_frame_time = current_time;

            if let Err(e) = self.update(delta_time) {
                log::error!("Update error: {e}");
            }

            if let Err(e) = self.render() {
                log::error!("Render error: {e}");
            }
            return;
        }

        // Handle the window event
        // Extract window temporarily to avoid borrow conflicts
        if let Some(window) = self.window.take() {
            let result = self.handle_window_event(&event, &window);
            self.window = Some(window); // Put it back

            match result {
                Ok(true) => {
                    if let Some(ref window) = self.window {
                        window.request_redraw();
                    }
                }
                Ok(false) => {
                    event_loop.exit();
                }
                Err(e) => {
                    log::error!("Error handling window event: {e}");
                    event_loop.exit();
                }
            }
        }
    }
}

impl Drop for AstrariaApp {
    fn drop(&mut self) {
        log::info!("Shutting down Astraria application");

        // Stop physics simulation first
        if let Some(mut physics) = self.physics.take() {
            physics.shutdown();
        }

        // Clean up other resources
        // (Rust's drop semantics will handle the rest)
    }
}
