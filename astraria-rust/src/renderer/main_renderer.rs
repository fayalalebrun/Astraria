use glam::{Mat4, Vec3};
use std::sync::Arc;
use wgpu::util::DeviceExt;
/// Main Renderer that manages all shader types
/// Equivalent to the Java Renderer class that owns all shader instances
use wgpu::{Device, Queue, RenderPass};

use crate::{
    assets::{AssetManager, CubemapAsset, TextureAsset},
    graphics::Mesh,
    renderer::{
        camera::Camera,
        core::{MeshType, RenderCommand, *},
        shaders::{
            BillboardShader, BlackHoleShader, DefaultShader, LensGlowShader, LineShader,
            PlanetAtmoShader, PointShader, SkyboxShader, SunShader,
        },
    },
    AstrariaError, AstrariaResult,
};

/// Main rendering coordinator that manages all specialized shaders
/// Based on the Java Renderer.java architecture
pub struct MainRenderer {
    // Core wgpu resources
    device: Device,
    queue: Queue,

    // Camera system
    pub camera: Camera,

    // Asset management
    pub asset_manager: AssetManager,

    // All shader instances (like Java Renderer class)
    pub default_shader: DefaultShader,
    pub planet_atmo_shader: PlanetAtmoShader,
    pub sun_shader: SunShader,
    pub skybox_shader: SkyboxShader,
    pub billboard_shader: BillboardShader,
    pub lens_glow_shader: LensGlowShader,
    pub black_hole_shader: BlackHoleShader,
    pub line_shader: LineShader,
    pub point_shader: PointShader,

    // Loaded textures for testing
    pub earth_day_texture: Arc<TextureAsset>,
    pub earth_night_texture: Arc<TextureAsset>,
    pub sun_texture: Arc<TextureAsset>,
    pub skybox_cubemap: Arc<CubemapAsset>,
    pub atmo_gradient_texture: Arc<TextureAsset>,
    pub star_glow_texture: Arc<TextureAsset>,
    pub star_spectrum_texture: Arc<TextureAsset>,

    // Pre-created bind groups to avoid lifetime issues
    pub sun_texture_bind_group: wgpu::BindGroup,
    pub skybox_texture_bind_group: wgpu::BindGroup,
    pub black_hole_texture_bind_group: wgpu::BindGroup,
    pub black_hole_uniform_bind_group: wgpu::BindGroup,
    pub lens_glow_texture_bind_group: wgpu::BindGroup,
    pub transform_bind_group: wgpu::BindGroup,
    transform_buffer: wgpu::Buffer,

    // Geometry meshes for testing
    cube_mesh: Mesh,
    sphere_mesh: Mesh,
    quad_mesh: Mesh,
    line_mesh: Mesh,
    point_mesh: Mesh,

    // Rendering constants (like Java Renderer)
    pub max_view_distance: f32,
    pub log_depth_constant: f32,
}

impl MainRenderer {
    pub async fn new() -> AstrariaResult<Self> {
        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| AstrariaError::Graphics("Failed to find adapter".to_string()))?;

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .map_err(|e| AstrariaError::Graphics(format!("Failed to create device: {}", e)))?;

        Self::with_device(device, queue).await
    }

    pub async fn with_surface(
        surface: wgpu::Surface,
        adapter: &wgpu::Adapter,
        surface_config: wgpu::SurfaceConfiguration,
    ) -> AstrariaResult<(Self, wgpu::Surface)> {
        // Request device and queue with the same adapter as the surface
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .map_err(|e| AstrariaError::Graphics(format!("Failed to create device: {}", e)))?;

        // Configure surface with the matching device
        surface.configure(&device, &surface_config);

        let main_renderer = Self::with_device(device, queue).await?;
        Ok((main_renderer, surface))
    }

    pub async fn with_device(device: wgpu::Device, queue: wgpu::Queue) -> AstrariaResult<Self> {
        // Create test geometry meshes
        let (cube_vertices, cube_indices) = create_cube_geometry();
        let (sphere_vertices, sphere_indices) = create_sphere_geometry(1.0, 32, 64);
        let (quad_vertices, quad_indices) = create_quad_geometry();
        let (line_vertices, line_indices) = create_line_geometry();
        let (point_vertices, point_indices) = create_point_geometry();

        let cube_mesh = Mesh::new(&device, &cube_vertices, &cube_indices);
        let sphere_mesh = Mesh::new(&device, &sphere_vertices, &sphere_indices);
        let quad_mesh = Mesh::new(&device, &quad_vertices, &quad_indices);
        let line_mesh = Mesh::new(&device, &line_vertices, &line_indices);
        let point_mesh = Mesh::new(&device, &point_vertices, &point_indices);

        // Load all required textures FIRST
        let mut asset_manager = AssetManager::new().await?;

        // Load Earth textures for atmospheric planet
        let earth_day_texture = asset_manager
            .load_texture(
                &device,
                &queue,
                "assets/Planet Textures/8k_earth_with_clouds.jpg",
            )
            .await?;
        let earth_night_texture = asset_manager
            .load_texture(
                &device,
                &queue,
                "assets/Planet Textures/8k_earth_nightmap.jpg",
            )
            .await?;

        // Load Sun texture
        let sun_texture = asset_manager
            .load_texture(&device, &queue, "assets/Planet Textures/8k_sun.jpg")
            .await?;

        // Load atmospheric gradient
        let atmo_gradient_texture = asset_manager
            .load_texture(&device, &queue, "assets/atmoGradient.png")
            .await?;

        // Load lens glow textures
        let star_glow_texture = asset_manager
            .load_texture(&device, &queue, "assets/star_glow.png")
            .await?;
        let star_spectrum_texture = asset_manager
            .load_texture(&device, &queue, "assets/star_spectrum_1D.png")
            .await?;

        // Load skybox cubemap
        let skybox_cubemap = asset_manager
            .load_cubemap(
                &device,
                &queue,
                "milky_way",
                &[
                    "assets/skybox/MilkyWayXP.png", // +X
                    "assets/skybox/MilkyWayXN.png", // -X
                    "assets/skybox/MilkyWayYP.png", // +Y
                    "assets/skybox/MilkyWayYN.png", // -Y
                    "assets/skybox/MilkyWayZP.png", // +Z
                    "assets/skybox/MilkyWayZN.png", // -Z
                ],
            )
            .await?;

        // Create camera system first since some shaders need the bind group layout
        let mut camera = Camera::new(800.0 / 600.0); // Aspect ratio
        let camera_bind_group_layout = camera.initialize_gpu_resources(&device);

        // Create all shader instances (textures are now loaded)
        let default_shader = DefaultShader::new(&device)?;
        let planet_atmo_shader = PlanetAtmoShader::new(
            &device,
            &queue,
            &earth_day_texture.texture,
            &earth_night_texture.texture,
            &atmo_gradient_texture.texture,
        )?;
        let sun_shader = SunShader::new(&device, &camera_bind_group_layout)?;
        let skybox_shader = SkyboxShader::new(&device, &camera_bind_group_layout)?;
        let billboard_shader = BillboardShader::new(&device, &queue)?;
        let lens_glow_shader = LensGlowShader::new(&device, &queue)?;
        let black_hole_shader = BlackHoleShader::new(&device, &queue)?;
        let line_shader = LineShader::new(&device, &queue)?;
        let point_shader = PointShader::new(&device, &queue)?;

        // Create pre-made bind groups to avoid lifetime issues
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // Create sun texture bind group with proper texture mappings
        let sun_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Sun Texture Bind Group"),
            layout: &sun_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&sun_texture.view), // Diffuse texture (sun surface)
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&star_spectrum_texture.view), // Temperature gradient
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Create skybox texture bind group
        let skybox_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Skybox Texture Bind Group"),
            layout: &skybox_shader.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&skybox_cubemap.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Update camera uniforms first so we can use its matrices
        camera.update(&queue);

        let model_matrix = Mat4::IDENTITY;
        let view_matrix = camera.view_projection_matrix(); // Get from camera
        let model_view_matrix = view_matrix * model_matrix;
        let normal_matrix = model_view_matrix.inverse().transpose();

        let transform_uniform = TransformUniform {
            model_matrix: model_matrix.to_cols_array_2d(),
            model_view_matrix: model_view_matrix.to_cols_array_2d(),
            normal_matrix: [
                [
                    normal_matrix.x_axis.x,
                    normal_matrix.x_axis.y,
                    normal_matrix.x_axis.z,
                    0.0,
                ],
                [
                    normal_matrix.y_axis.x,
                    normal_matrix.y_axis.y,
                    normal_matrix.y_axis.z,
                    0.0,
                ],
                [
                    normal_matrix.z_axis.x,
                    normal_matrix.z_axis.y,
                    normal_matrix.z_axis.z,
                    0.0,
                ],
            ],
            _padding: [0.0; 4],
        };

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let transform_bind_group_layout =
            crate::renderer::core::create_transform_bind_group_layout(&device);
        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transform Bind Group"),
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        // Global camera and transform bind groups are set separately

        // Create lens glow texture bind group
        let lens_glow_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Lens Glow Texture Bind Group"),
            layout: &lens_glow_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&star_glow_texture.view), // Glow texture
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&star_spectrum_texture.view), // Spectrum texture
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Create black hole texture bind group (uses skybox cubemap for lensing background)
        let black_hole_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Black Hole Texture Bind Group"),
            layout: &black_hole_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&skybox_cubemap.view), // Skybox for lensing
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Create black hole uniform buffer
        use crate::renderer::shaders::black_hole_shader::BlackHoleUniform;
        let black_hole_uniform = BlackHoleUniform {
            hole_position: [0.0, 0.0, 0.0],
            _padding: 0.0,
        };
        let black_hole_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Black Hole Uniform Buffer"),
                contents: bytemuck::cast_slice(&[black_hole_uniform]),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        // Create black hole specific uniforms bind group (transform and black hole data only)
        // Note: Camera is now handled by the shared camera bind group (group 0)
        let black_hole_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Black Hole Uniform Bind Group"),
            layout: &black_hole_shader.uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: black_hole_uniform_buffer.as_entire_binding(),
                },
            ],
        });

        Ok(Self {
            device,
            queue,
            camera,
            asset_manager,
            default_shader,
            planet_atmo_shader,
            sun_shader,
            skybox_shader,
            billboard_shader,
            lens_glow_shader,
            black_hole_shader,
            line_shader,
            point_shader,
            earth_day_texture,
            earth_night_texture,
            sun_texture,
            skybox_cubemap,
            atmo_gradient_texture,
            star_glow_texture,
            star_spectrum_texture,
            sun_texture_bind_group,
            skybox_texture_bind_group,
            black_hole_texture_bind_group,
            black_hole_uniform_bind_group,
            lens_glow_texture_bind_group,
            transform_bind_group,
            transform_buffer,
            cube_mesh,
            sphere_mesh,
            quad_mesh,
            line_mesh,
            point_mesh,
            max_view_distance: 100000000000.0, // Like Java MAXVIEWDISTANCE
            log_depth_constant: 1.0,           // Like Java LOGDEPTHCONSTANT
        })
    }

    /// Get device reference for external use
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Get queue reference for external use
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Update camera with movement and GPU uniforms
    pub fn update_camera(&mut self) {
        self.camera.update_movement(0.016); // Assume ~60fps for now
        self.camera.update(&self.queue);
    }

    /// Unified render method that takes a render command and transform
    /// This eliminates code duplication by providing a single interface for all shader types
    pub fn render<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        command: &RenderCommand,
        transform: Mat4,
    ) {
        // Update transform buffer with the provided transform matrix
        let view_matrix = self.camera.view_matrix();
        let model_view_matrix = view_matrix * transform;
        let normal_matrix = transform.inverse().transpose();

        let transform_uniform = TransformUniform {
            model_matrix: transform.to_cols_array_2d(),
            model_view_matrix: model_view_matrix.to_cols_array_2d(),
            normal_matrix: [
                [
                    normal_matrix.x_axis.x,
                    normal_matrix.x_axis.y,
                    normal_matrix.x_axis.z,
                    0.0,
                ],
                [
                    normal_matrix.y_axis.x,
                    normal_matrix.y_axis.y,
                    normal_matrix.y_axis.z,
                    0.0,
                ],
                [
                    normal_matrix.z_axis.x,
                    normal_matrix.z_axis.y,
                    normal_matrix.z_axis.z,
                    0.0,
                ],
            ],
            _padding: [0.0; 4],
        };
        self.queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );

        // Set shared bind groups once for all shaders
        // Group 0: Camera (shared by all shaders)
        render_pass.set_bind_group(0, self.camera.bind_group().unwrap(), &[]);
        // Group 1: Transform (used by shaders that need it, ignored by others)
        render_pass.set_bind_group(1, &self.transform_bind_group, &[]);
        match command {
            RenderCommand::Default {
                mesh_type,
                light_position: _,
                light_color: _,
            } => {
                // Update DefaultShader uniforms with the passed transform
                self.default_shader.update_uniforms(
                    &self.queue,
                    self.camera.view_matrix(),
                    self.camera.projection_matrix(),
                    transform,
                );

                let mesh = self.get_mesh(mesh_type);
                self.default_shader.render_mesh(render_pass, mesh);
            }

            RenderCommand::AtmosphericPlanet {
                star_position,
                planet_position,
                atmosphere_color,
                overglow,
                use_ambient_texture,
            } => {
                self.planet_atmo_shader.update_uniforms(
                    &self.queue,
                    self.camera.view_matrix(),
                    self.camera.projection_matrix(),
                    transform,
                    Vec3::new(2.0, 2.0, 2.0), // light_position - could be parameterized
                    Vec3::new(1.0, 1.0, 1.0), // light_color - could be parameterized
                    *star_position,
                    *planet_position,
                    *atmosphere_color,
                    *overglow,
                    *use_ambient_texture,
                );

                self.planet_atmo_shader
                    .render_mesh(render_pass, &self.sphere_mesh);
            }

            RenderCommand::Sun {
                temperature,
                star_position,
                camera_position,
            } => {
                self.sun_shader.update_uniforms(
                    &self.queue,
                    *temperature,
                    *star_position,
                    *camera_position,
                );

                // Sun shader: groups 0 and 1 already set by main renderer
                render_pass.set_pipeline(&self.sun_shader.pipeline);
                render_pass.set_bind_group(2, &self.sun_shader.bind_group, &[]);
                render_pass.set_bind_group(3, &self.sun_texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.sphere_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.sphere_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.sphere_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::Skybox => {
                render_pass.set_pipeline(&self.skybox_shader.pipeline);
                render_pass.set_bind_group(1, &self.skybox_texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.cube_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.cube_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.cube_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::Billboard => {
                render_pass.set_pipeline(&self.billboard_shader.pipeline);
                render_pass.set_vertex_buffer(0, self.quad_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.quad_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.quad_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::LensGlow => {
                render_pass.set_pipeline(&self.lens_glow_shader.pipeline);
                render_pass.set_bind_group(1, &self.lens_glow_shader.uniform_bind_group, &[]);
                render_pass.set_bind_group(2, &self.lens_glow_texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.quad_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.quad_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.quad_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::BlackHole => {
                render_pass.set_pipeline(&self.black_hole_shader.pipeline);
                render_pass.set_bind_group(1, &self.black_hole_uniform_bind_group, &[]);
                render_pass.set_bind_group(2, &self.black_hole_texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.sphere_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.sphere_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.sphere_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::Line { color: _ } => {
                // Update line shader color uniform if needed
                // For now, use existing line shader setup
                render_pass.set_pipeline(&self.line_shader.pipeline);
                render_pass.set_bind_group(1, &self.line_shader.line_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.line_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.line_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.line_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::Point => {
                render_pass.set_pipeline(&self.point_shader.pipeline);
                render_pass.set_vertex_buffer(0, self.point_mesh.vertex_buffer.slice(..));
                render_pass.draw(0..self.point_mesh.num_indices, 0..1);
            }
        }
    }

    /// Helper method to get the appropriate mesh for a given mesh type
    fn get_mesh(&self, mesh_type: &MeshType) -> &Mesh {
        match mesh_type {
            MeshType::Cube => &self.cube_mesh,
            MeshType::Sphere => &self.sphere_mesh,
            MeshType::Quad => &self.quad_mesh,
            MeshType::Line => &self.line_mesh,
            MeshType::Point => &self.point_mesh,
        }
    }
}
