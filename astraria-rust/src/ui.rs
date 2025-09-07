/// User interface system using egui
/// Replaces the VisUI system from the original implementation
use egui_winit::winit;
use winit::dpi::PhysicalSize;

use crate::{AstrariaResult, physics::PhysicsSimulation, renderer::Renderer};
use glam::DVec3;

/// Actions that the UI can request from the application
#[derive(Debug, Clone)]
pub enum UiAction {
    FocusCameraOnObject {
        object_index: usize,
        position: DVec3,
        radius: f64,
    },
    ClearCameraFocus,
}

pub struct UserInterface {
    egui_ctx: egui::Context,
    egui_winit: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,

    // UI state
    show_controls: bool,
    show_info: bool,
    show_stats: bool,
    show_object_list: bool,
    simulation_speed: f32,
    selected_object_index: Option<usize>,
    pending_actions: Vec<UiAction>,
    ui_visible: bool,
}

impl UserInterface {
    pub fn new(window: &winit::window::Window, renderer: &Renderer) -> AstrariaResult<Self> {
        let egui_ctx = egui::Context::default();
        let viewport_id = egui_winit::egui::ViewportId::ROOT;
        let egui_winit = egui_winit::State::new(
            egui_ctx.clone(),
            viewport_id,
            window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        let egui_renderer = egui_wgpu::Renderer::new(
            renderer.device(),
            wgpu::TextureFormat::Bgra8UnormSrgb, // Adjust format as needed
            None,
            1,
            false,
        );

        Ok(Self {
            egui_ctx,
            egui_winit,
            egui_renderer,
            show_controls: true,
            show_info: true,
            show_stats: false,
            show_object_list: true,
            simulation_speed: 1.0,
            selected_object_index: None,
            pending_actions: Vec::new(),
            ui_visible: true,
        })
    }

    pub fn handle_event(
        &mut self,
        event: &winit::event::WindowEvent,
        window: &winit::window::Window,
    ) -> AstrariaResult<bool> {
        // Handle H key to toggle UI visibility
        if let winit::event::WindowEvent::KeyboardInput {
            event:
                winit::event::KeyEvent {
                    physical_key: winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyH),
                    state: winit::event::ElementState::Pressed,
                    ..
                },
            ..
        } = event
        {
            self.ui_visible = !self.ui_visible;
            log::info!("UI visibility toggled to: {}", self.ui_visible);
            return Ok(true); // Consume the H key event
        }

        let response = self.egui_winit.on_window_event(window, event);
        Ok(response.consumed)
    }

    pub fn update(
        &mut self,
        _delta_time: f32,
        physics: Option<&PhysicsSimulation>,
        _renderer: &mut Option<Renderer>,
    ) -> AstrariaResult<()> {
        // Update simulation speed if changed
        if let Some(physics) = physics {
            if let Ok(current_speed) = physics.get_simulation_speed() {
                if (current_speed - self.simulation_speed).abs() > 1e-6 {
                    let _ = physics.set_simulation_speed(self.simulation_speed);
                }
            }
        }

        Ok(())
    }

    /// Get pending UI actions and clear the queue
    pub fn take_actions(&mut self) -> Vec<UiAction> {
        std::mem::take(&mut self.pending_actions)
    }

    pub fn prepare(
        &mut self,
        renderer: &mut Renderer,
        window: &winit::window::Window,
        physics: Option<&PhysicsSimulation>,
    ) -> AstrariaResult<(egui_wgpu::ScreenDescriptor, Vec<egui::ClippedPrimitive>)> {
        // Begin egui frame
        let raw_input = self.egui_winit.take_egui_input(window);
        let mut show_controls = self.show_controls;
        let mut show_info = self.show_info;
        let mut show_stats = self.show_stats;
        let mut show_object_list = self.show_object_list;
        let mut simulation_speed = self.simulation_speed;
        let mut selected_object_index = self.selected_object_index;
        let mut pending_actions = Vec::new();
        let ui_visible = self.ui_visible;

        // Get physics data for object list
        let bodies = if let Some(physics) = physics {
            physics.get_bodies().unwrap_or_default()
        } else {
            Vec::new()
        };

        let full_output = self.egui_ctx.run(raw_input, |ctx| {
            Self::render_ui_static(
                ctx,
                &mut show_controls,
                &mut show_info,
                &mut show_stats,
                &mut show_object_list,
                &mut simulation_speed,
                &mut selected_object_index,
                &bodies,
                &mut pending_actions,
                ui_visible,
            );
        });

        // Update state after rendering
        self.show_controls = show_controls;
        self.show_info = show_info;
        self.show_stats = show_stats;
        self.show_object_list = show_object_list;
        self.simulation_speed = simulation_speed;
        self.selected_object_index = selected_object_index;

        // Store pending actions
        self.pending_actions.extend(pending_actions);

        // Handle egui output (cursor, copy/paste, etc.)
        self.egui_winit
            .handle_platform_output(window, full_output.platform_output);

        // Prepare egui rendering
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [window.inner_size().width, window.inner_size().height],
            pixels_per_point: window.scale_factor() as f32,
        };

        let clipped_primitives = self
            .egui_ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        // Update egui texture atlas
        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer.update_texture(
                renderer.device(),
                renderer.queue(),
                *id,
                image_delta,
            );
        }

        // Free egui textures
        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        Ok((screen_descriptor, clipped_primitives))
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        surface_view: &wgpu::TextureView,
        clipped_primitives: &[egui::ClippedPrimitive],
        screen_descriptor: &egui_wgpu::ScreenDescriptor,
    ) -> AstrariaResult<()> {
        // Update buffers first
        self.egui_renderer.update_buffers(
            device,
            queue,
            encoder,
            clipped_primitives,
            screen_descriptor,
        );

        // Begin render pass and immediately render - don't store the pass in a variable
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Load existing scene
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        // Render using the correct lifetime management
        // Safety: The render pass is guaranteed to live for the duration of this call
        unsafe {
            let static_render_pass: &mut wgpu::RenderPass<'static> =
                std::mem::transmute(&mut render_pass);
            self.egui_renderer
                .render(static_render_pass, clipped_primitives, screen_descriptor);
        }
        drop(render_pass);

        Ok(())
    }

    fn render_ui_static(
        ctx: &egui::Context,
        show_controls: &mut bool,
        show_info: &mut bool,
        show_stats: &mut bool,
        show_object_list: &mut bool,
        simulation_speed: &mut f32,
        selected_object_index: &mut Option<usize>,
        bodies: &[crate::math::Body],
        pending_actions: &mut Vec<UiAction>,
        ui_visible: bool,
    ) {
        // If UI is hidden, don't render any windows
        if !ui_visible {
            return;
        }
        // Control panel
        if *show_controls {
            egui::Window::new("Simulation Controls")
                .default_pos([10.0, 10.0])
                .default_size([300.0, 200.0])
                .show(ctx, |ui| {
                    ui.heading("Physics");

                    ui.horizontal(|ui| {
                        ui.label("Speed:");
                        ui.add(
                            egui::Slider::new(simulation_speed, 0.1..=10.0)
                                .logarithmic(true)
                                .text("x"),
                        );
                    });

                    ui.separator();

                    ui.heading("View");
                    ui.checkbox(show_info, "Show Info");
                    ui.checkbox(show_stats, "Show Statistics");
                    if ui.checkbox(show_object_list, "Show Object List").changed() {
                        log::info!("Object List toggled to: {}", *show_object_list);
                    }
                });
        }

        // Info panel
        if *show_info {
            egui::Window::new("Simulation Info")
                .default_pos([10.0, 230.0])
                .default_size([300.0, 150.0])
                .show(ctx, |ui| {
                    ui.label("Astraria - Rust Port");
                    ui.label("3D Orbital Mechanics Simulator");
                    ui.separator();

                    ui.label("Controls:");
                    ui.label("• WASD: Move camera");
                    ui.label("• Mouse: Look around");
                    ui.label("• Scroll: Adjust speed");
                    ui.label("• H: Toggle UI");
                });
        }

        // Statistics panel
        if *show_stats {
            egui::Window::new("Statistics")
                .default_pos([320.0, 10.0])
                .default_size([250.0, 200.0])
                .show(ctx, |ui| {
                    ui.label("System Statistics");
                    ui.separator();

                    // Display actual simulation statistics
                    ui.label(format!("Bodies: {}", bodies.len()));
                    ui.label(format!("FPS: {:.1}", 60.0)); // TODO: Get actual FPS
                    ui.label(format!("Physics Steps/s: {}", 0)); // TODO: Get actual physics rate
                });
        }

        // Object List Window - Based on Java ObjectListWindow.java
        log::debug!("Object List visibility: {}", *show_object_list);
        if *show_object_list {
            log::debug!("Rendering Object List window with {} bodies", bodies.len());
            let window_response = egui::Window::new("Object List")
                .default_pos([320.0, 230.0]) // Move to a more visible position
                .default_size([250.0, 300.0]) // Make it slightly larger
                .resizable(true)
                .collapsible(false) // Don't allow collapsing for now
                .show(ctx, |ui| {
                    ui.label("Simulation Objects");
                    ui.separator();

                    ui.label(format!("Total bodies: {}", bodies.len()));
                    ui.separator();

                    if bodies.is_empty() {
                        ui.label("No objects in simulation");
                        ui.separator();
                        ui.small("Objects will appear here when");
                        ui.small("a scenario is loaded.");
                    } else {
                        // Scrollable list of objects
                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                for (index, body) in bodies.iter().enumerate() {
                                    let is_selected = selected_object_index.map_or(false, |sel| sel == index);

                                    // Create selectable object entry
                                    let response = ui.selectable_label(is_selected, &body.name);

                                    if response.clicked() {
                                        // Handle object selection
                                        *selected_object_index = Some(index);
                                        log::info!("Selected object: {} (index: {})", body.name, index);

                                        // Get radius for camera positioning
                                        let radius = match &body.body_type {
                                            crate::scenario::BodyType::Planet { radius, .. } => *radius as f64,
                                            crate::scenario::BodyType::Star { radius, .. } => *radius as f64,
                                            crate::scenario::BodyType::PlanetAtmo { radius, .. } => *radius as f64,
                                            crate::scenario::BodyType::BlackHole { radius } => *radius as f64,
                                        };

                                        // Queue camera focus action
                                        pending_actions.push(UiAction::FocusCameraOnObject {
                                            object_index: index,
                                            position: body.position,
                                            radius,
                                        });

                                        log::info!("Queued camera focus on '{}' at position ({:.2e}, {:.2e}, {:.2e})",
                                            body.name, body.position.x, body.position.y, body.position.z);
                                    }
                                }
                            });

                        ui.separator();

                        // Show selection info
                        if let Some(selected_idx) = *selected_object_index {
                            if let Some(selected_body) = bodies.get(selected_idx) {
                                ui.label(format!("Selected: {}", selected_body.name));

                                // Show basic info about selected object
                                ui.small(format!("Mass: {:.2e} kg", selected_body.mass));

                                // Show radius based on body type
                                let radius = match &selected_body.body_type {
                                    crate::scenario::BodyType::Planet { radius, .. } => *radius,
                                    crate::scenario::BodyType::Star { radius, .. } => *radius,
                                    crate::scenario::BodyType::PlanetAtmo { radius, .. } => *radius,
                                    crate::scenario::BodyType::BlackHole { radius } => *radius,
                                };
                                ui.small(format!("Radius: {:.2e} m", radius));
                            }
                        } else {
                            ui.label("No object selected");
                        }

                        // Clear selection button
                        if ui.button("Clear Selection").clicked() {
                            *selected_object_index = None;
                            pending_actions.push(UiAction::ClearCameraFocus);
                        }
                    }
                });

            if let Some(_response) = window_response {
                log::debug!("Object List window rendered successfully");
            } else {
                log::warn!("Object List window failed to render");
            }
        }

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load Scenario...").clicked() {
                        // TODO: Open file dialog
                    }
                    if ui.button("Save Scenario...").clicked() {
                        // TODO: Save current state
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        // TODO: Signal application exit
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox(show_controls, "Controls");
                    ui.checkbox(show_info, "Info");
                    ui.checkbox(show_stats, "Statistics");
                    if ui.checkbox(show_object_list, "Object List").changed() {
                        log::info!("Object List toggled via menu to: {}", *show_object_list);
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About...").clicked() {
                        // TODO: Show about dialog
                    }
                });
            });
        });
    }

    pub fn resize(&mut self, _new_size: PhysicalSize<u32>) -> AstrariaResult<()> {
        // egui handles resize automatically
        Ok(())
    }
}
