use anyhow::Result;
/// Main application structure that coordinates all subsystems
/// Replaces the LibGDX Game class from the original implementation
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    assets::AssetManager, input::InputHandler, physics::PhysicsSimulation, renderer::Renderer,
    ui::UserInterface, AstrariaResult,
};

pub struct AstrariaApp {
    renderer: Option<Renderer>,
    physics: Option<PhysicsSimulation>,
    ui: Option<UserInterface>,
    input_handler: Option<InputHandler>,
    asset_manager: Option<AssetManager>,
    last_frame_time: std::time::Instant,
    scenario_file: String,
}

impl AstrariaApp {
    pub fn new() -> Result<Self> {
        Self::new_with_scenario("Solar_System_2K.txt".to_string())
    }

    pub fn new_with_scenario(scenario_file: String) -> Result<Self> {
        Ok(Self {
            renderer: None,
            physics: None,
            ui: None,
            input_handler: None,
            asset_manager: None,
            last_frame_time: std::time::Instant::now(),
            scenario_file,
        })
    }

    pub fn run(mut self) -> Result<()> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Astraria - 3D Orbital Mechanics Simulator")
            .with_inner_size(PhysicalSize::new(1280, 720))
            .with_min_inner_size(PhysicalSize::new(800, 600))
            .build(&event_loop)?;

        // Initialize the application
        pollster::block_on(self.initialize(&window))?;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match self.handle_window_event(event) {
                    Ok(true) => {
                        window.request_redraw();
                    }
                    Ok(false) => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Err(e) => {
                        log::error!("Error handling window event: {}", e);
                        *control_flow = ControlFlow::Exit;
                    }
                },
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    let current_time = std::time::Instant::now();
                    let delta_time = current_time
                        .duration_since(self.last_frame_time)
                        .as_secs_f32();
                    self.last_frame_time = current_time;

                    if let Err(e) = self.update(delta_time) {
                        log::error!("Update error: {}", e);
                    }

                    if let Err(e) = self.render() {
                        log::error!("Render error: {}", e);
                    }
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }

    async fn initialize(&mut self, window: &winit::window::Window) -> AstrariaResult<()> {
        log::info!("Initializing Astraria application...");

        // Initialize asset manager first
        self.asset_manager = Some(AssetManager::new().await?);

        // Initialize renderer
        let renderer = Renderer::new(&window, self.asset_manager.as_mut().unwrap()).await?;
        self.renderer = Some(renderer);

        // Initialize physics simulation
        self.physics = Some(PhysicsSimulation::new());

        // Initialize input handler
        self.input_handler = Some(InputHandler::new());

        // Initialize UI
        let ui = UserInterface::new(&window, self.renderer.as_ref().unwrap())?;
        self.ui = Some(ui);

        // Load default scenario if available
        self.load_default_scenario().await?;

        log::info!("Astraria initialization complete!");
        Ok(())
    }

    async fn load_default_scenario(&mut self) -> AstrariaResult<()> {
        // Try to load the specified scenario file
        if let Some(asset_manager) = &self.asset_manager {
            if let Ok(scenario_data) = asset_manager.load_scenario(&self.scenario_file).await {
                if let Some(physics) = &mut self.physics {
                    physics.load_scenario(scenario_data)?;
                    log::info!("Loaded scenario: {}", self.scenario_file);
                }
            } else {
                log::warn!(
                    "Could not load scenario '{}', starting with empty simulation",
                    self.scenario_file
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

            // Handle camera input
            if let Some(renderer) = &mut self.renderer {
                renderer.handle_camera_input(input_handler)?;
            }
        }

        // Update UI
        if let Some(ui) = &mut self.ui {
            ui.update(delta_time, self.physics.as_ref(), &mut self.renderer)?;
        }

        Ok(())
    }

    fn render(&mut self) -> AstrariaResult<()> {
        if let (Some(renderer), Some(physics), Some(asset_manager)) =
            (&mut self.renderer, &self.physics, &self.asset_manager)
        {
            renderer.begin_frame()?;

            // Render 3D scene
            renderer.render_scene(physics, asset_manager)?;

            // TODO: Render UI overlay - need window reference
            // ui.render(renderer, window)?;

            renderer.end_frame()?;
        }

        Ok(())
    }

    fn handle_window_event(&mut self, event: &WindowEvent) -> AstrariaResult<bool> {
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
            if ui.handle_event(event)? {
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
