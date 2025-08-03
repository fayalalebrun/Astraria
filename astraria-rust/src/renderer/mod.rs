pub mod buffers;
/// Graphics rendering system using wgpu
/// Replaces the LibGDX rendering pipeline with modern GPU-driven approach
pub mod camera;
pub mod core;
pub mod lighting;
pub mod main_renderer;
pub mod pipeline;
pub mod shaders;

use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{assets::AssetManager, physics::PhysicsSimulation, AstrariaError, AstrariaResult};

pub use buffers::BufferManager;
pub use camera::Camera;
pub use core::*;
pub use lighting::LightManager;
pub use main_renderer::MainRenderer;
pub use pipeline::PipelineManager;
pub use shaders::ShaderManager;

pub struct Renderer {
    surface: Surface,
    surface_config: SurfaceConfiguration,
    buffers: BufferManager,
    lights: LightManager,

    // Rendering state
    pub current_frame: Option<wgpu::SurfaceTexture>,
    depth_texture: wgpu::Texture,
    pub depth_view: wgpu::TextureView,

    // Use MainRenderer for shader management and device access
    main_renderer: MainRenderer,
}

impl Renderer {
    pub async fn new(window: &Window, asset_manager: &mut AssetManager) -> AstrariaResult<Self> {
        log::info!("Initializing wgpu renderer...");

        // Create wgpu instance with all available backends for better compatibility
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface
        let surface = unsafe { instance.create_surface(window) }
            .map_err(|e| AstrariaError::Graphics(format!("Failed to create surface: {}", e)))?;

        // Request adapter with fallback support
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await;

        // If no adapter found, try with fallback
        let adapter = if adapter.is_none() {
            log::warn!("No primary adapter found, trying fallback");
            instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: true,
                })
                .await
        } else {
            adapter
        };

        let adapter = adapter.ok_or_else(|| {
            AstrariaError::Graphics("Failed to find any suitable GPU adapter".to_string())
        })?;

        log::info!("Using GPU: {}", adapter.get_info().name);

        // Configure surface first
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();
        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        // Create MainRenderer with the surface and adapter to ensure device compatibility
        let (main_renderer, surface) =
            MainRenderer::with_surface(surface, &adapter, surface_config.clone()).await?;

        // Get device and queue from MainRenderer
        let device = main_renderer.device();
        let queue = main_renderer.queue();

        // Create depth texture
        let (depth_texture, depth_view) = Self::create_depth_texture(device, &surface_config);

        // Initialize subsystems
        let buffers = BufferManager::new(device, asset_manager, queue)?;
        let lights = LightManager::new(device)?;

        log::info!("Renderer initialization complete");

        Ok(Self {
            surface,
            surface_config,
            main_renderer,
            buffers,
            lights,
            current_frame: None,
            depth_texture,
            depth_view,
        })
    }

    fn create_depth_texture(
        device: &Device,
        config: &SurfaceConfiguration,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        (depth_texture, depth_view)
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) -> AstrariaResult<()> {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface
                .configure(self.main_renderer.device(), &self.surface_config);

            // Recreate depth texture
            let (depth_texture, depth_view) =
                Self::create_depth_texture(self.main_renderer.device(), &self.surface_config);
            self.depth_texture = depth_texture;
            self.depth_view = depth_view;

            // Update camera aspect ratio
            self.main_renderer
                .camera
                .set_aspect_ratio(new_size.width as f32 / new_size.height as f32);

            log::debug!("Renderer resized to {}x{}", new_size.width, new_size.height);
        }
        Ok(())
    }

    pub fn begin_frame(&mut self) -> AstrariaResult<()> {
        // Get the next frame
        let frame = self.surface.get_current_texture().map_err(|e| {
            AstrariaError::Graphics(format!("Failed to get surface texture: {}", e))
        })?;

        self.current_frame = Some(frame);
        Ok(())
    }

    pub fn render_scene(
        &mut self,
        physics: &PhysicsSimulation,
        asset_manager: &AssetManager,
    ) -> AstrariaResult<()> {
        let frame = self
            .current_frame
            .as_ref()
            .ok_or_else(|| AstrariaError::Graphics("No current frame available".to_string()))?;

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.main_renderer
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // Update camera movement and matrices
        self.main_renderer.update_camera();

        self.lights.update(self.main_renderer.queue(), physics)?;

        // Generate render commands from physics simulation
        let render_commands = self.generate_physics_render_commands(physics)?;

        // Create render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            // First render skybox for background
            let skybox_command = crate::renderer::core::RenderCommand::Skybox;
            self.main_renderer
                .render(&mut render_pass, &skybox_command, glam::Mat4::IDENTITY);

            // Render all physics bodies using MainRenderer
            for (command, transform) in &render_commands {
                self.main_renderer
                    .render(&mut render_pass, command, *transform);
            }
        }

        // Submit the command buffer
        self.main_renderer
            .queue()
            .submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    fn generate_physics_render_commands(
        &self,
        physics: &PhysicsSimulation,
    ) -> AstrariaResult<Vec<(crate::renderer::core::RenderCommand, glam::Mat4)>> {
        use crate::renderer::core::{MeshType, RenderCommand};
        use crate::scenario::BodyType;
        use glam::{Mat4, Vec3, Vec4};

        let mut commands = Vec::new();

        // Try to get physics bodies
        if let Ok(bodies) = physics.get_bodies() {
            if !bodies.is_empty() {
                log::info!("Generating {} physics body render commands", bodies.len());

                for body in &bodies {
                    // Convert astronomical coordinates to camera-relative f32 coordinates
                    // Scale factor to make Solar System visible (1 AU = ~1.5e11 m -> ~100 units)
                    let scale_factor = 6.67e-10; // Adjusted for visibility
                    let position = Vec3::new(
                        (body.position.x * scale_factor) as f32,
                        (body.position.y * scale_factor) as f32,
                        (body.position.z * scale_factor) as f32,
                    );

                    // Get body radius and choose appropriate render command
                    // Make planets visible but not too large
                    let radius_scale = match &body.body_type {
                        BodyType::Planet { radius, .. } => (*radius * 1e-6).max(0.1), // Minimum visible size
                        BodyType::Star { radius, .. } => (*radius * 5e-8).max(1.0), // Sun should be bigger
                        BodyType::PlanetAtmo { radius, .. } => (*radius * 1e-6).max(0.1),
                        BodyType::BlackHole { radius } => (*radius * 1e-6).max(0.1),
                    };

                    log::debug!(
                        "Body '{}' at position ({:.2}, {:.2}, {:.2}) with radius {:.2}",
                        body.name,
                        position.x,
                        position.y,
                        position.z,
                        radius_scale
                    );

                    let transform = Mat4::from_translation(position)
                        * Mat4::from_scale(Vec3::splat(radius_scale));

                    // Choose render command based on body type
                    let command = match &body.body_type {
                        BodyType::Star { temperature, .. } => RenderCommand::Sun {
                            temperature: *temperature,
                            star_position: position,
                            camera_position: self.main_renderer.camera.position().as_vec3(),
                        },
                        BodyType::PlanetAtmo { atmo_color, .. } => {
                            RenderCommand::AtmosphericPlanet {
                                star_position: Vec3::new(0.0, 0.0, 0.0), // Assume Sun at origin for now
                                planet_position: position,
                                atmosphere_color: Vec4::new(
                                    atmo_color[0],
                                    atmo_color[1],
                                    atmo_color[2],
                                    atmo_color[3],
                                ),
                                overglow: 0.1,
                                use_ambient_texture: false,
                            }
                        }
                        BodyType::Planet { .. } => RenderCommand::Default {
                            mesh_type: MeshType::Sphere,
                            light_position: Vec3::new(0.0, 0.0, 0.0), // Sun position
                            light_color: Vec3::new(1.0, 1.0, 1.0),
                        },
                        BodyType::BlackHole { .. } => RenderCommand::Default {
                            mesh_type: MeshType::Sphere,
                            light_position: Vec3::new(0.0, 0.0, 0.0),
                            light_color: Vec3::new(0.1, 0.1, 0.1), // Dim for black hole
                        },
                    };

                    commands.push((command, transform));

                    log::debug!("Generated render command for body '{}' at position ({:.2e}, {:.2e}, {:.2e}) with radius {:.2e}", 
                        body.name, position.x, position.y, position.z, radius_scale);
                }
                return Ok(commands);
            }
        }

        // Fallback: render some test objects if no physics bodies
        let default_command = RenderCommand::Default {
            mesh_type: MeshType::Sphere,
            light_position: Vec3::new(2.0, 2.0, 2.0),
            light_color: Vec3::new(1.0, 1.0, 1.0),
        };
        commands.push((default_command, Mat4::IDENTITY));

        Ok(commands)
    }

    pub fn end_frame(&mut self) -> AstrariaResult<()> {
        if let Some(frame) = self.current_frame.take() {
            frame.present();
        }
        Ok(())
    }

    // Getters for subsystems
    pub fn camera(&self) -> &Camera {
        &self.main_renderer.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.main_renderer.camera
    }

    pub fn device(&self) -> &Device {
        self.main_renderer.device()
    }

    pub fn queue(&self) -> &Queue {
        self.main_renderer.queue()
    }

    pub fn lights_mut(&mut self) -> &mut LightManager {
        &mut self.lights
    }

    /// Handle camera input
    pub fn handle_camera_input(
        &mut self,
        input: &mut crate::input::InputHandler,
    ) -> AstrariaResult<()> {
        // Handle WASD movement
        if input.is_key_pressed(&winit::event::VirtualKeyCode::W) {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Forward, true);
        } else {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Forward, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::S) {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Backward, true);
        } else {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Backward, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::A) {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Left, true);
        } else {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Left, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::D) {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Right, true);
        } else {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Right, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::Space) {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Up, true);
        } else {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Up, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::LShift) {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Down, true);
        } else {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Down, false);
        }

        // Handle Q/E roll controls
        if input.is_key_pressed(&winit::event::VirtualKeyCode::Q) {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::RollLeft, true);
        } else {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::RollLeft, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::E) {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::RollRight, true);
        } else {
            self.main_renderer
                .camera
                .process_keyboard(crate::renderer::camera::CameraMovement::RollRight, false);
        }

        // Handle mouse look
        if let Some((delta_x, delta_y)) = input.take_mouse_delta() {
            log::info!(
                "Processing mouse look: delta=({:.2}, {:.2})",
                delta_x,
                delta_y
            );
            self.main_renderer
                .camera
                .process_mouse_movement(delta_x, delta_y);
        }

        // Handle scroll wheel for camera speed adjustment
        if let Some(scroll_delta) = input.take_scroll_delta() {
            self.main_renderer.camera.process_scroll(scroll_delta);
        }

        Ok(())
    }
}
