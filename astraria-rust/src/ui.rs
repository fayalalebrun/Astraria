/// User interface system using egui
/// Replaces the VisUI system from the original implementation
use egui_winit::winit;
use winit::dpi::PhysicalSize;

use crate::{physics::PhysicsSimulation, renderer::Renderer, AstrariaResult};

pub struct UserInterface {
    egui_ctx: egui::Context,
    egui_winit: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,

    // UI state
    show_controls: bool,
    show_info: bool,
    show_stats: bool,
    simulation_speed: f32,
}

impl UserInterface {
    pub fn new(window: &winit::window::Window, renderer: &Renderer) -> AstrariaResult<Self> {
        let egui_ctx = egui::Context::default();
        let egui_winit = egui_winit::State::new(window);

        let egui_renderer = egui_wgpu::Renderer::new(
            renderer.device(),
            wgpu::TextureFormat::Bgra8UnormSrgb, // Adjust format as needed
            None,
            1,
        );

        Ok(Self {
            egui_ctx,
            egui_winit,
            egui_renderer,
            show_controls: true,
            show_info: true,
            show_stats: false,
            simulation_speed: 1.0,
        })
    }

    pub fn handle_event(&mut self, event: &winit::event::WindowEvent) -> AstrariaResult<bool> {
        let response = self.egui_winit.on_event(&self.egui_ctx, event);
        Ok(response.consumed)
    }

    pub fn update(
        &mut self,
        _delta_time: f32,
        physics: Option<&PhysicsSimulation>,
        renderer: &mut Option<Renderer>,
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

    pub fn render(
        &mut self,
        renderer: &mut Renderer,
        window: &winit::window::Window,
    ) -> AstrariaResult<()> {
        // Begin egui frame
        let raw_input = self.egui_winit.take_egui_input(window);
        let mut show_controls = self.show_controls;
        let mut show_info = self.show_info;
        let mut show_stats = self.show_stats;
        let mut simulation_speed = self.simulation_speed;

        let _full_output = self.egui_ctx.run(raw_input, |ctx| {
            Self::render_ui_static(
                ctx,
                &mut show_controls,
                &mut show_info,
                &mut show_stats,
                &mut simulation_speed,
            );
        });

        // Update state after rendering
        self.show_controls = show_controls;
        self.show_info = show_info;
        self.show_stats = show_stats;
        self.simulation_speed = simulation_speed;

        // Handle egui output
        // Note: egui 0.23 API is different from newer versions
        // self.egui_winit.handle_platform_output(...);

        // Render egui
        // TODO: Integrate egui rendering with wgpu render pass

        Ok(())
    }

    fn render_ui_static(
        ctx: &egui::Context,
        show_controls: &mut bool,
        show_info: &mut bool,
        show_stats: &mut bool,
        simulation_speed: &mut f32,
    ) {
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

                    // TODO: Display actual simulation statistics
                    ui.label(format!("Bodies: {}", 0));
                    ui.label(format!("FPS: {:.1}", 60.0));
                    ui.label(format!("Physics Steps/s: {}", 0));
                });
        }

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
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
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About...").clicked() {
                        // TODO: Show about dialog
                    }
                });
            });
        });
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) -> AstrariaResult<()> {
        // egui handles resize automatically
        Ok(())
    }
}
