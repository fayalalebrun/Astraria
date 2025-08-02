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

use crate::{
    assets::{AssetManager, ModelAsset},
    graphics::{test_geometry, Mesh},
    physics::PhysicsSimulation,
    AstrariaError, AstrariaResult,
};
use wgpu::util::DeviceExt;

pub use buffers::BufferManager;
pub use camera::Camera;
pub use core::*;
pub use lighting::LightManager;
pub use main_renderer::MainRenderer;
pub use pipeline::PipelineManager;
pub use shaders::ShaderManager;

pub struct Renderer {
    // Core wgpu resources
    device: Device,
    queue: Queue,
    surface: Surface,
    surface_config: SurfaceConfiguration,

    // Rendering managers
    camera: Camera,
    shaders: ShaderManager,
    pub pipelines: PipelineManager,
    pub buffers: BufferManager,
    lights: LightManager,

    // Rendering state
    pub current_frame: Option<wgpu::SurfaceTexture>,
    depth_texture: wgpu::Texture,
    pub depth_view: wgpu::TextureView,

    // Test geometry for validation
    pub test_triangle: Option<Mesh>,
    pub test_cube: Option<Mesh>,
    test_sphere_model: Option<String>, // Path to loaded sphere model

    // Skybox rendering
    skybox_vertex_buffer: Option<wgpu::Buffer>,
    skybox_index_buffer: Option<wgpu::Buffer>,
    skybox_num_indices: u32,
    skybox_bind_group: Option<wgpu::BindGroup>,

    // Demo shader bind groups (temporary)
    demo_star_bind_group: Option<wgpu::BindGroup>,
    demo_star_texture_bind_group: Option<wgpu::BindGroup>,

    // Logarithmic depth buffer constants
    log_depth_constant: f32,
    far_plane_distance: f32,
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

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Astraria Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| AstrariaError::Graphics(format!("Failed to create device: {}", e)))?;

        // Configure surface
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

        surface.configure(&device, &surface_config);

        // Create depth texture
        let (depth_texture, depth_view) = Self::create_depth_texture(&device, &surface_config);

        // Initialize subsystems
        let mut camera = Camera::new(size.width as f32 / size.height as f32);
        camera.initialize_gpu_resources(&device);
        let shaders = ShaderManager::new();
        // Temporarily disabled for new shader architecture
        // let pipelines = PipelineManager::new(&device, &shaders, surface_format)?;
        let buffers = BufferManager::new(&device, asset_manager, &queue)?;
        let lights = LightManager::new(&device)?;

        // Create test geometry for validation
        let (triangle_vertices, triangle_indices) = test_geometry::create_test_triangle();
        let test_triangle = Mesh::new(&device, &triangle_vertices, &triangle_indices);

        let (cube_vertices, cube_indices) = test_geometry::create_test_cube();
        let test_cube = Mesh::new(&device, &cube_vertices, &cube_indices);

        // Load test sphere model
        let sphere_path = "assets/models/sphere.obj";
        let test_sphere_model = match asset_manager.load_model(&device, sphere_path).await {
            Ok(_model) => {
                log::debug!("Successfully loaded test model: {}", sphere_path);
                Some(sphere_path.to_string())
            }
            Err(e) => {
                log::warn!("Failed to load test model {}: {}", sphere_path, e);
                None
            }
        };

        // Logarithmic depth buffer constants (from original implementation)
        let log_depth_constant = 1.0;
        let far_plane_distance = 1e11; // 100 billion units for astronomical scale

        // Load and create skybox
        let (skybox_vertex_buffer, skybox_index_buffer, skybox_num_indices, skybox_bind_group) =
            Self::create_skybox(&device, &queue, asset_manager).await?;

        // Create demo shader bind groups
        let (demo_star_bind_group, demo_star_texture_bind_group) =
            Self::create_demo_star_bind_groups(&device, &queue, asset_manager).await?;

        log::info!("Renderer initialization complete");

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            camera,
            shaders,
            pipelines: PipelineManager::new_empty(),
            buffers,
            lights,
            current_frame: None,
            depth_texture,
            depth_view,
            test_triangle: Some(test_triangle),
            test_cube: Some(test_cube),
            test_sphere_model,
            skybox_vertex_buffer: Some(skybox_vertex_buffer),
            skybox_index_buffer: Some(skybox_index_buffer),
            skybox_num_indices,
            skybox_bind_group: Some(skybox_bind_group),
            demo_star_bind_group: Some(demo_star_bind_group),
            demo_star_texture_bind_group: Some(demo_star_texture_bind_group),
            log_depth_constant,
            far_plane_distance,
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
            self.surface.configure(&self.device, &self.surface_config);

            // Recreate depth texture
            let (depth_texture, depth_view) =
                Self::create_depth_texture(&self.device, &self.surface_config);
            self.depth_texture = depth_texture;
            self.depth_view = depth_view;

            // Update camera aspect ratio
            self.camera
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

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Update camera movement and matrices
        self.camera.update_movement(0.016); // Assume ~60fps for now
        self.camera.update(&self.queue);
        self.lights.update(&self.queue, physics)?;

        // Update buffers with current camera data
        let view_matrix = glam::Mat4::look_at_rh(
            self.camera.position().as_vec3(),
            self.camera.position().as_vec3() + self.camera.direction(),
            glam::Vec3::Y,
        );
        let proj_matrix = glam::Mat4::perspective_rh(
            45.0_f32.to_radians(),
            self.surface_config.width as f32 / self.surface_config.height as f32,
            0.1,
            100.0,
        );
        let view_proj = proj_matrix * view_matrix;

        self.buffers.update_camera(
            &self.queue,
            view_matrix,
            proj_matrix,
            view_proj,
            self.camera.position().as_vec3(),
            self.camera.direction(),
        );

        let model = glam::Mat4::IDENTITY;
        self.buffers.update_transform(&self.queue, model);

        let default_light = crate::renderer::buffers::PointLight {
            position: [5.0, 5.0, 5.0],
            _padding1: 0.0,
            ambient: [0.1, 0.1, 0.1],
            _padding2: 0.0,
            diffuse: [1.0, 1.0, 1.0],
            _padding3: 0.0,
            specular: [1.0, 1.0, 1.0],
            _padding4: 0.0,
        };
        self.buffers.update_lighting(&self.queue, &[default_light]);

        // Begin render pass
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

            // Render skybox first (if available)
            self.render_skybox(&mut render_pass)?;

            // Render simulation objects
            self.render_simulation_objects(&mut render_pass, physics, asset_manager)?;

            // Render UI elements (orbital paths, etc.)
            self.render_ui_elements(&mut render_pass, physics)?;
        }

        // Submit the command buffer
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    fn render_skybox<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) -> AstrariaResult<()> {
        if let (Some(pipeline), Some(vertex_buffer), Some(index_buffer), Some(bind_group)) = (
            self.pipelines
                .get_pipeline(crate::renderer::shaders::ShaderType::Skybox),
            &self.skybox_vertex_buffer,
            &self.skybox_index_buffer,
            &self.skybox_bind_group,
        ) {
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, &self.buffers.camera_bind_group, &[]);
            render_pass.set_bind_group(1, bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.skybox_num_indices, 0, 0..1);
        }
        Ok(())
    }

    fn render_simulation_objects<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        _physics: &PhysicsSimulation,
        asset_manager: &'a AssetManager,
    ) -> AstrariaResult<()> {
        // Render test geometry for now
        self.render_test_geometry(render_pass, asset_manager)?;

        // Render demo objects using the new shaders
        self.render_demo_planet_with_atmosphere(render_pass, asset_manager)?;
        self.render_demo_star(render_pass, asset_manager)?;
        self.render_demo_billboard(render_pass, asset_manager)?;

        Ok(())
    }

    pub fn render_test_geometry<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        asset_manager: &'a AssetManager,
    ) -> AstrariaResult<()> {
        // Set the default pipeline
        if let Some(pipeline) = self
            .pipelines
            .get_pipeline(crate::renderer::shaders::ShaderType::Default)
        {
            render_pass.set_pipeline(pipeline);

            // Set bind groups for uniforms
            render_pass.set_bind_group(0, &self.buffers.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.buffers.transform_bind_group, &[]);
            render_pass.set_bind_group(2, &self.buffers.lighting_bind_group, &[]);
            render_pass.set_bind_group(3, &self.buffers.default_texture_bind_group, &[]);

            // Render triangle first
            if let Some(triangle) = &self.test_triangle {
                // Position triangle slightly to the left
                let triangle_model = glam::Mat4::from_translation(glam::Vec3::new(-1.2, 0.0, 0.0));
                self.buffers
                    .update_triangle_transform(&self.queue, triangle_model);

                // Use triangle-specific transform bind group
                render_pass.set_bind_group(1, &self.buffers.triangle_transform_bind_group, &[]);

                render_pass.set_vertex_buffer(0, triangle.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(triangle.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..triangle.num_indices, 0, 0..1);
            }

            // Render cube second
            if let Some(cube) = &self.test_cube {
                // Position cube slightly to the right and rotate it
                let rotation = glam::Mat4::from_rotation_y(std::f32::consts::PI * 0.25);
                let translation = glam::Mat4::from_translation(glam::Vec3::new(1.2, 0.0, 0.0));
                let cube_model = translation * rotation;
                self.buffers.update_cube_transform(&self.queue, cube_model);

                // Use cube-specific transform bind group
                render_pass.set_bind_group(1, &self.buffers.cube_transform_bind_group, &[]);

                render_pass.set_vertex_buffer(0, cube.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(cube.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..cube.num_indices, 0, 0..1);
            }

            // Render loaded model (if available)
            if let Some(model_path) = &self.test_sphere_model {
                if let Some(model) = asset_manager.get_model(model_path) {
                    // Position model above the other objects
                    let translation = glam::Mat4::from_translation(glam::Vec3::new(0.0, 2.0, 0.0));
                    let scale = glam::Mat4::from_scale(glam::Vec3::splat(0.8));
                    let model_transform = translation * scale;
                    self.buffers.update_transform(&self.queue, model_transform);
                    render_pass.set_bind_group(1, &self.buffers.transform_bind_group, &[]);

                    render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
                    render_pass
                        .set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..model.num_indices, 0, 0..1);
                }
            }
        }
        Ok(())
    }

    fn render_ui_elements(
        &self,
        _render_pass: &mut wgpu::RenderPass,
        _physics: &PhysicsSimulation,
    ) -> AstrariaResult<()> {
        // TODO: Implement UI element rendering (orbital paths, etc.)
        Ok(())
    }

    pub fn end_frame(&mut self) -> AstrariaResult<()> {
        if let Some(frame) = self.current_frame.take() {
            frame.present();
        }
        Ok(())
    }

    /// Handle camera input
    pub fn handle_camera_input(
        &mut self,
        input: &mut crate::input::InputHandler,
    ) -> AstrariaResult<()> {
        // Handle WASD movement
        if input.is_key_pressed(&winit::event::VirtualKeyCode::W) {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Forward, true);
        } else {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Forward, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::S) {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Backward, true);
        } else {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Backward, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::A) {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Left, true);
        } else {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Left, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::D) {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Right, true);
        } else {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Right, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::Space) {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Up, true);
        } else {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Up, false);
        }

        if input.is_key_pressed(&winit::event::VirtualKeyCode::LShift) {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Down, true);
        } else {
            self.camera
                .process_keyboard(crate::renderer::camera::CameraMovement::Down, false);
        }

        // Handle mouse look
        if let Some((delta_x, delta_y)) = input.take_mouse_delta() {
            self.camera.process_mouse_movement(delta_x, delta_y);
        }

        Ok(())
    }

    // Getters for subsystems
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn lights_mut(&mut self) -> &mut LightManager {
        &mut self.lights
    }

    async fn create_skybox(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        asset_manager: &mut AssetManager,
    ) -> AstrariaResult<(wgpu::Buffer, wgpu::Buffer, u32, wgpu::BindGroup)> {
        // Create skybox geometry (using the original face order from Astraria)
        let (vertices, indices) = test_geometry::create_skybox_cube();

        // Create vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Load the MilkyWay cubemap (following original Astraria order)
        let face_paths = [
            "assets/skyboxes/MilkyWayXP.png", // +X (right)
            "assets/skyboxes/MilkyWayXN.png", // -X (left)
            "assets/skyboxes/MilkyWayYP.png", // +Y (top)
            "assets/skyboxes/MilkyWayYN.png", // -Y (bottom)
            "assets/skyboxes/MilkyWayZP.png", // +Z (front)
            "assets/skyboxes/MilkyWayZN.png", // -Z (back)
        ];

        let cubemap = asset_manager
            .load_cubemap(device, queue, "milky_way", &face_paths)
            .await?;

        // Create sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Skybox Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create bind group layout
        let skybox_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Skybox Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::Cube,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Skybox Bind Group"),
            layout: &skybox_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&cubemap.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Ok((
            vertex_buffer,
            index_buffer,
            indices.len() as u32,
            bind_group,
        ))
    }

    async fn create_demo_star_bind_groups(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        asset_manager: &mut AssetManager,
    ) -> AstrariaResult<(wgpu::BindGroup, wgpu::BindGroup)> {
        // Create dummy star uniform data
        #[repr(C)]
        #[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        struct StarUniform {
            temperature: f32,
            _padding1: f32,
            camera_to_sun_direction: [f32; 3],
            _padding2: f32,
            sun_position: [f32; 3],
            _padding3: f32,
        }

        let star_uniform = StarUniform {
            temperature: 5778.0, // Sun's temperature in Kelvin
            _padding1: 0.0,
            camera_to_sun_direction: [1.0, 0.0, 0.0],
            _padding2: 0.0,
            sun_position: [-4.0, 0.0, -3.0], // Match our star position
            _padding3: 0.0,
        };

        // Create uniform buffer
        let star_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Demo Star Uniform Buffer"),
            contents: bytemuck::cast_slice(&[star_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layout for star uniform
        let star_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Demo Star Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Create star uniform bind group
        let star_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Demo Star Bind Group"),
            layout: &star_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: star_uniform_buffer.as_entire_binding(),
            }],
        });

        // Create a default texture for the demo
        let default_texture = asset_manager.create_default_texture(device, queue)?;

        // Create sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Demo Star Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create texture bind group layout
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Demo Star Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        // Create texture bind group (using default texture for both diffuse and gradient)
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Demo Star Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&default_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&default_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Ok((star_bind_group, texture_bind_group))
    }

    // Demo rendering methods to showcase additional objects using different geometries
    fn render_demo_planet_with_atmosphere<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        _asset_manager: &'a AssetManager,
    ) -> AstrariaResult<()> {
        // Use cube as a demo planet for now
        if let (Some(pipeline), Some(cube)) = (
            self.pipelines
                .get_pipeline(crate::renderer::shaders::ShaderType::Default),
            &self.test_cube,
        ) {
            render_pass.set_pipeline(pipeline);

            // Position planet cube to the far right
            let planet_transform = glam::Mat4::from_translation(glam::Vec3::new(4.0, 0.0, -3.0))
                * glam::Mat4::from_rotation_y(std::f32::consts::PI * 0.5)
                * glam::Mat4::from_scale(glam::Vec3::splat(1.5));
            self.buffers
                .update_cube_transform(&self.queue, planet_transform);

            render_pass.set_bind_group(0, &self.buffers.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.buffers.cube_transform_bind_group, &[]);
            render_pass.set_bind_group(2, &self.buffers.lighting_bind_group, &[]);
            render_pass.set_bind_group(3, &self.buffers.default_texture_bind_group, &[]);

            render_pass.set_vertex_buffer(0, cube.vertex_buffer.slice(..));
            render_pass.set_index_buffer(cube.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..cube.num_indices, 0, 0..1);
        }
        Ok(())
    }

    fn render_demo_star<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        _asset_manager: &'a AssetManager,
    ) -> AstrariaResult<()> {
        // Use default shader for now - star shader needs uniform buffer compatibility fixes
        if let (Some(pipeline), Some(triangle)) = (
            self.pipelines
                .get_pipeline(crate::renderer::shaders::ShaderType::Default),
            &self.test_triangle,
        ) {
            render_pass.set_pipeline(pipeline);

            // Position star triangle to the far left
            let star_transform = glam::Mat4::from_translation(glam::Vec3::new(-4.0, 0.0, -3.0))
                * glam::Mat4::from_rotation_z(std::f32::consts::PI * 0.3)
                * glam::Mat4::from_scale(glam::Vec3::splat(2.0));
            self.buffers
                .update_triangle_transform(&self.queue, star_transform);

            render_pass.set_bind_group(0, &self.buffers.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.buffers.triangle_transform_bind_group, &[]);
            render_pass.set_bind_group(2, &self.buffers.lighting_bind_group, &[]);
            render_pass.set_bind_group(3, &self.buffers.default_texture_bind_group, &[]);

            render_pass.set_vertex_buffer(0, triangle.vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(triangle.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..triangle.num_indices, 0, 0..1);
        }
        Ok(())
    }

    fn render_demo_billboard<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        _asset_manager: &'a AssetManager,
    ) -> AstrariaResult<()> {
        // Use cube as a demo billboard object positioned high up
        if let (Some(pipeline), Some(cube)) = (
            self.pipelines
                .get_pipeline(crate::renderer::shaders::ShaderType::Default),
            &self.test_cube,
        ) {
            render_pass.set_pipeline(pipeline);

            // Position billboard cube high above the scene
            let billboard_transform = glam::Mat4::from_translation(glam::Vec3::new(0.0, 4.0, -2.0))
                * glam::Mat4::from_rotation_x(std::f32::consts::PI * 0.25)
                * glam::Mat4::from_scale(glam::Vec3::splat(0.8));
            self.buffers
                .update_transform(&self.queue, billboard_transform);

            render_pass.set_bind_group(0, &self.buffers.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.buffers.transform_bind_group, &[]);
            render_pass.set_bind_group(2, &self.buffers.lighting_bind_group, &[]);
            render_pass.set_bind_group(3, &self.buffers.default_texture_bind_group, &[]);

            render_pass.set_vertex_buffer(0, cube.vertex_buffer.slice(..));
            render_pass.set_index_buffer(cube.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..cube.num_indices, 0, 0..1);
        }
        Ok(())
    }
}
