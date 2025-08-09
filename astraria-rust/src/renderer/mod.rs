pub mod buffers;
/// Graphics rendering system using wgpu
/// Replaces the LibGDX rendering pipeline with modern GPU-driven approach
pub mod camera;
pub mod core;
pub mod lighting;
pub mod main_renderer;
pub mod occlusion;
pub mod pipeline;
pub mod precision_math;
pub mod shader_utils;
pub mod shaders;
pub mod uniforms;

use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{AstrariaError, AstrariaResult, assets::AssetManager, physics::PhysicsSimulation};

/// Physics constants for lens glow size calculation (from Java LensGlow.java)
const SOLAR_DIAMETER_KM: f64 = 1392684.0; // Solar diameter in kilometers
const SOLAR_TEMPERATURE_K: f64 = 5778.0; // Solar temperature in Kelvin

pub use buffers::BufferManager;
pub use camera::Camera;
pub use core::*;
pub use lighting::LightManager;
pub use main_renderer::MainRenderer;
pub use pipeline::PipelineManager;
pub use shaders::ShaderManager;

pub struct Renderer {
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    _buffers: BufferManager,
    lights: LightManager,

    // Rendering state
    pub current_frame: Option<wgpu::SurfaceTexture>,
    depth_texture: wgpu::Texture,
    pub depth_view: wgpu::TextureView,

    // Use MainRenderer for shader management and device access
    main_renderer: MainRenderer,
}

/// Calculate lens glow size using exact Java LensGlow.calculateGlowSize() formula
/// Matches the Java implementation precisely for consistent results
pub fn calculate_lens_glow_size(diameter_km: f64, temperature_k: f64, distance_m: f64) -> f64 {
    // Java constants - exact values from LensGlow.calculateGlowSize()
    const DSUN: f64 = 1392684.0; // Sun diameter in km
    const TSUN: f64 = 5778.0; // Sun temperature in K

    // Convert distance from meters to kilometers (Java formula expects km)
    let d = distance_m / 1000.0;

    // Java formula step 1: D = diameter * DSUN
    let D = diameter_km * DSUN;

    // Java formula step 2: L = (D * D) * (temperature / TSUN)^4.0
    let L = (D * D) * (temperature_k / TSUN).powf(4.0);

    // Java formula step 3: size = 0.016 * L^0.25 / d^0.3
    let size = 0.016 * L.powf(0.25) / d.powf(0.3);

    size
}

/// Calculate distance modifier for lens glow based on camera proximity
pub fn get_distance_modifier(distance_m: f64) -> f32 {
    // Add proximity-based scaling (closer = larger glow)
    let base_distance = 1e9; // 1 million km reference distance
    let factor = (base_distance / distance_m).min(5.0).max(0.1); // Clamp between 0.1x and 5x
    factor as f32
}

impl Renderer {
    pub async fn new(window: &Window, asset_manager: &mut AssetManager) -> AstrariaResult<Self> {
        log::info!("Initializing wgpu renderer...");

        // Create wgpu instance with all available backends for better compatibility
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface with transmuted lifetime - safe because window lives for entire app
        let surface = instance
            .create_surface(window)
            .map_err(|e| AstrariaError::Graphics(format!("Failed to create surface: {}", e)))?;
        let surface: Surface<'static> = unsafe { std::mem::transmute(surface) };

        // Request adapter with fallback support
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await;

        // If no adapter found, try with fallback
        let adapter = match adapter {
            Ok(adapter) => adapter,
            Err(_) => {
                log::warn!("No primary adapter found, trying fallback");
                instance
                    .request_adapter(&wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::default(),
                        compatible_surface: Some(&surface),
                        force_fallback_adapter: true,
                    })
                    .await
                    .map_err(|e| {
                        AstrariaError::Graphics(format!(
                            "Failed to find any suitable GPU adapter: {}",
                            e
                        ))
                    })?
            }
        };

        log::info!("Using GPU: {}", adapter.get_info().name);

        // Configure surface first
        let surface_caps = surface.get_capabilities(&adapter);
        // Use Bgra8UnormSrgb which is supported on this system
        let surface_format = wgpu::TextureFormat::Bgra8UnormSrgb;

        log::info!("Surface format chosen: {:?}", surface_format);
        log::info!("Available surface formats: {:?}", surface_caps.formats);

        let size = window.inner_size();
        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        // Create MainRenderer with the surface and adapter to ensure device compatibility
        let (main_renderer, surface) = MainRenderer::with_surface(&instance, surface).await?;

        // Get device and queue from MainRenderer
        let device = main_renderer.device();
        let queue = main_renderer.queue();

        // Create depth texture
        let (depth_texture, depth_view) = Self::create_depth_texture(device, &surface_config);

        // Initialize subsystems
        let _buffers = BufferManager::new(device, asset_manager, queue)?;
        let lights = LightManager::new(device)?;

        log::info!("Renderer initialization complete");

        Ok(Self {
            surface,
            surface_config,
            main_renderer,
            _buffers,
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

    /// Update camera with delta time for movement
    pub fn update_camera(&mut self, delta_time: f32) {
        self.main_renderer.update_camera(delta_time);
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

        self.lights.update(self.main_renderer.queue(), physics)?;

        // Position camera relative to the first body (usually the Sun) if not already positioned
        self.position_camera_if_needed(physics)?;

        // Reset frame MVP data and prepare for new frame
        self.main_renderer.begin_frame();

        // Generate render commands from physics simulation
        let render_commands = self.generate_physics_render_commands(physics)?;
        log::debug!(
            "Generated {} render commands for physics bodies",
            render_commands.len()
        );

        // Separate solid objects from lens glow effects for proper rendering order
        let mut solid_commands = Vec::new();
        let mut lens_glow_commands = Vec::new();

        for (command, transform) in render_commands {
            match command {
                crate::renderer::core::RenderCommand::LensGlow { .. } => {
                    lens_glow_commands.push((command, transform));
                }
                _ => {
                    solid_commands.push((command, transform));
                }
            }
        }

        // Prepare skybox command first
        let skybox_command = crate::renderer::core::RenderCommand::Skybox;
        self.main_renderer
            .prepare_render_command(skybox_command, glam::Mat4::IDENTITY);

        // Prepare all solid object render commands first
        for (command, transform) in &solid_commands {
            self.main_renderer
                .prepare_render_command(command.clone(), *transform);
        }

        // Prepare lens glow commands last (to render on top)
        for (command, transform) in &lens_glow_commands {
            self.main_renderer
                .prepare_render_command(command.clone(), *transform);
        }

        // MVP data is uploaded per-object using generated bind groups (no bulk upload needed)

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
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0), // Clear depth buffer for main scene
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Execute all prepared render commands with dynamic MVP offsets
            // This includes skybox and all physics bodies
            self.main_renderer
                .execute_prepared_commands(&mut render_pass);
        }

        // TEMPORARILY DISABLED: Execute occlusion queries AFTER main scene rendering
        // This is disabled to test if the black screen is caused by occlusion system
        log::debug!("Occlusion queries temporarily disabled for debugging");

        // TEMPORARILY DISABLED: Process occlusion results from previous frames
        log::debug!("Occlusion result processing temporarily disabled for debugging");

        // Submit the command buffer
        self.main_renderer
            .queue()
            .submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    fn generate_physics_render_commands(
        &mut self,
        physics: &PhysicsSimulation,
    ) -> AstrariaResult<Vec<(crate::renderer::core::RenderCommand, glam::Mat4)>> {
        use crate::renderer::core::{MeshType, RenderCommand};
        use crate::scenario::BodyType;
        use glam::{DVec3, Mat4, Vec3, Vec4};

        let mut commands = Vec::new();

        // Try to get physics bodies
        if let Ok(bodies) = physics.get_bodies() {
            if !bodies.is_empty() {
                log::debug!("Generating {} physics body render commands", bodies.len());
                log::debug!(
                    "Camera position: ({:.2e}, {:.2e}, {:.2e})",
                    self.main_renderer.camera.position().x,
                    self.main_renderer.camera.position().y,
                    self.main_renderer.camera.position().z
                );

                // Find the sun position first (needed for lighting calculations)
                let sun_position = bodies
                    .iter()
                    .find(|body| matches!(body.body_type, BodyType::Star { .. }))
                    .map(|sun| sun.position)
                    .unwrap_or(DVec3::ZERO); // Fallback to origin if no sun found

                for (body_index, body) in bodies.iter().enumerate() {
                    // Use TRUE ASTRONOMICAL SCALE - no scaling down allowed!
                    let position = Vec3::new(
                        body.position.x as f32,
                        body.position.y as f32,
                        body.position.z as f32,
                    );

                    // Use TRUE RADIUS - no scaling down allowed!
                    let radius_scale = match &body.body_type {
                        BodyType::Planet { radius, .. } => *radius as f32,
                        BodyType::Star { radius, .. } => *radius as f32,
                        BodyType::PlanetAtmo { radius, .. } => *radius as f32,
                        BodyType::BlackHole { radius } => *radius as f32,
                    };

                    log::debug!(
                        "Body '{}' at position ({:.2e}, {:.2e}, {:.2e}) with radius {:.2e}",
                        body.name,
                        position.x,
                        position.y,
                        position.z,
                        radius_scale
                    );

                    // Calculate distance from camera to body
                    let camera_pos = self.main_renderer.camera.position().as_vec3();
                    let distance_to_camera = (position - camera_pos).length();

                    // Calculate apparent size in pixels (rough estimate)
                    let fov_radians = 45.0_f32.to_radians(); // Camera FOV
                    let screen_height = 720.0; // Assumed screen height
                    let angular_size = 2.0 * (radius_scale / distance_to_camera).atan();
                    let apparent_pixels = (angular_size / fov_radians) * screen_height;

                    log::debug!(
                        "Distance from camera to {}: {:.2e} meters, apparent size: {:.2} pixels",
                        body.name,
                        distance_to_camera,
                        apparent_pixels
                    );

                    let transform = Mat4::from_translation(position)
                        * Mat4::from_scale(Vec3::splat(radius_scale));

                    // Choose render command based on body type
                    let command = match &body.body_type {
                        BodyType::Star { temperature, .. } => RenderCommand::Sun {
                            temperature: *temperature,
                        },
                        BodyType::PlanetAtmo {
                            atmo_color,
                            ambient_texture,
                            texture_path,
                            ..
                        } => RenderCommand::AtmosphericPlanet {
                            atmosphere_color: Vec4::new(
                                atmo_color[0],
                                atmo_color[1],
                                atmo_color[2],
                                atmo_color[3],
                            ),
                            overglow: 0.1,
                            use_ambient_texture: ambient_texture.is_some(),
                            texture_path: texture_path.clone(),
                            ambient_texture_path: ambient_texture.clone(),
                            planet_position: body.position,
                            sun_position,
                        },
                        BodyType::Planet { texture_path, .. } => RenderCommand::Planet {
                            texture_path: texture_path.clone(),
                            planet_position: body.position,
                            sun_position,
                        },
                        BodyType::BlackHole { .. } => RenderCommand::Default {
                            mesh_type: MeshType::Sphere,
                            light_position: Vec3::new(0.0, 0.0, 0.0),
                            light_color: Vec3::new(0.1, 0.1, 0.1), // Dim for black hole
                        },
                    };

                    commands.push((command.clone(), transform));

                    // Add lens glow effect for stars
                    if let BodyType::Star {
                        temperature,
                        radius,
                        ..
                    } = &body.body_type
                    {
                        // Calculate camera distance to star
                        let camera_distance =
                            (body.position - self.main_renderer.camera.position()).length();

                        let star_id = body_index as u32;
                        log::info!(
                            "Setting up lens glow for star '{}' (ID: {}) at distance {:.2e}",
                            body.name,
                            star_id,
                            camera_distance
                        );

                        // Test occlusion for this star using simplified system
                        if let Err(e) = self
                            .main_renderer
                            .test_star_occlusion(star_id, body.position)
                        {
                            log::warn!(
                                "Failed to set up occlusion test for star {}: {}",
                                star_id,
                                e
                            );
                        } else {
                            log::debug!("Successfully queued occlusion test for star {}", star_id);
                        }

                        let lens_glow_command = RenderCommand::LensGlow {
                            star_id, // Use body index as star ID
                            star_position: body.position,
                            star_temperature: *temperature,
                            star_radius: *radius as f64,
                            camera_distance,
                        };

                        // Use physics-based size calculation
                        // Java uses star.getRadius() * 200 as diameter input
                        let diameter_km = ((*radius as f64) / 1000.0) * 200.0; // Convert radius to km, then multiply by 200
                        let physics_size_pixels = calculate_lens_glow_size(
                            diameter_km,
                            *temperature as f64,
                            camera_distance,
                        );

                        // BILLBOARD: Transform only positions the star center, size is handled in shader

                        // Only position the star - no scaling needed for billboard
                        let glow_transform = Mat4::from_translation(position);
                        commands.push((lens_glow_command, glow_transform));
                    }

                    log::debug!(
                        "Generated render command for body '{}' - transform: {:?}",
                        body.name,
                        transform
                    );
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

    /// Position camera relative to the first physics body if not already positioned
    fn position_camera_if_needed(&mut self, physics: &PhysicsSimulation) -> AstrariaResult<()> {
        // Only position camera if it's at origin (initial position)
        if self.main_renderer.camera.position().length() < 1e3 {
            if let Ok(bodies) = physics.get_bodies() {
                if !bodies.is_empty() {
                    let first_body = &bodies[0];
                    let body_radius = match &first_body.body_type {
                        crate::scenario::BodyType::Planet { radius, .. } => *radius,
                        crate::scenario::BodyType::Star { radius, .. } => *radius,
                        crate::scenario::BodyType::PlanetAtmo { radius, .. } => *radius,
                        crate::scenario::BodyType::BlackHole { radius } => *radius,
                    };

                    self.main_renderer.camera.position_relative_to_body(
                        first_body.position,
                        body_radius as f64,
                        3.0, // 3x the body's radius
                    );
                    log::debug!(
                        "Positioned camera relative to first body: {}",
                        first_body.name
                    );
                }
            }
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

    pub fn set_camera_position(&mut self, position: glam::DVec3) {
        self.main_renderer.camera.set_position(position);
    }

    pub fn set_camera_look_at(&mut self, target: glam::DVec3, distance: f64) {
        self.main_renderer.camera.look_at(target, distance);
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

    pub fn main_renderer(&mut self) -> &mut MainRenderer {
        &mut self.main_renderer
    }

    /// Handle camera input
    pub fn handle_camera_input(
        &mut self,
        input: &mut crate::input::InputHandler,
        delta_time: f32,
    ) -> AstrariaResult<()> {
        use crate::renderer::camera::CameraMovement;
        use winit::keyboard::KeyCode;

        // Handle WASD movement - process movement when keys are pressed
        let camera = &mut self.main_renderer.camera;

        if input.is_key_pressed(&KeyCode::KeyW) {
            camera.process_movement(CameraMovement::Forward, delta_time);
        }
        if input.is_key_pressed(&KeyCode::KeyS) {
            camera.process_movement(CameraMovement::Backward, delta_time);
        }
        if input.is_key_pressed(&KeyCode::KeyA) {
            camera.process_movement(CameraMovement::Left, delta_time);
        }
        if input.is_key_pressed(&KeyCode::KeyD) {
            camera.process_movement(CameraMovement::Right, delta_time);
        }
        if input.is_key_pressed(&KeyCode::Space) {
            camera.process_movement(CameraMovement::Up, delta_time);
        }
        if input.is_key_pressed(&KeyCode::ShiftLeft) {
            camera.process_movement(CameraMovement::Down, delta_time);
        }
        if input.is_key_pressed(&KeyCode::KeyQ) {
            camera.process_movement(CameraMovement::RollLeft, delta_time);
        }
        if input.is_key_pressed(&KeyCode::KeyE) {
            camera.process_movement(CameraMovement::RollRight, delta_time);
        }

        // Handle mouse look
        if let Some((delta_x, delta_y)) = input.take_mouse_delta() {
            log::debug!(
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
