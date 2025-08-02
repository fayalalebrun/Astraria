use glam::{Mat4, Vec2, Vec3, Vec4};
use std::sync::Arc;
use wgpu::util::DeviceExt;
/// Main Renderer that manages all shader types
/// Equivalent to the Java Renderer class that owns all shader instances
use wgpu::{CommandEncoder, Device, Queue, RenderPass, Surface, SurfaceConfiguration};

use crate::{
    assets::{AssetManager, CubemapAsset, ModelAsset, TextureAsset},
    renderer::{
        core::*,
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
    pub camera_bind_group: wgpu::BindGroup,

    // Geometry buffers for testing
    cube_vertex_buffer: wgpu::Buffer,
    cube_index_buffer: wgpu::Buffer,
    cube_num_indices: u32,

    sphere_vertex_buffer: wgpu::Buffer,
    sphere_index_buffer: wgpu::Buffer,
    sphere_num_indices: u32,

    quad_vertex_buffer: wgpu::Buffer,
    quad_index_buffer: wgpu::Buffer,
    quad_num_indices: u32,

    line_vertex_buffer: wgpu::Buffer,
    line_index_buffer: wgpu::Buffer,
    line_num_indices: u32,

    point_vertex_buffer: wgpu::Buffer,
    point_index_buffer: wgpu::Buffer,
    point_num_indices: u32,

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

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| AstrariaError::Graphics("Failed to find GPU adapter".to_string()))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Main Renderer Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| AstrariaError::Graphics(format!("Failed to create device: {}", e)))?;

        // Create test geometry
        let (cube_vertices, cube_indices) = create_cube_geometry();
        let (sphere_vertices, sphere_indices) = create_sphere_geometry(1.0, 32, 64);
        let (quad_vertices, quad_indices) = create_quad_geometry();
        let (line_vertices, line_indices) = create_line_geometry();
        let (point_vertices, point_indices) = create_point_geometry();

        let cube_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(&cube_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let cube_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(&cube_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let sphere_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Vertex Buffer"),
            contents: bytemuck::cast_slice(&sphere_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let sphere_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Index Buffer"),
            contents: bytemuck::cast_slice(&sphere_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Vertex Buffer"),
            contents: bytemuck::cast_slice(&quad_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let quad_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Index Buffer"),
            contents: bytemuck::cast_slice(&quad_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let line_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Vertex Buffer"),
            contents: bytemuck::cast_slice(&line_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let line_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Index Buffer"),
            contents: bytemuck::cast_slice(&line_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let point_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Point Vertex Buffer"),
            contents: bytemuck::cast_slice(&point_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let point_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Point Index Buffer"),
            contents: bytemuck::cast_slice(&point_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

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

        // Create camera bind group layout that all shaders will use
        let camera_bind_group_layout = create_camera_bind_group_layout(&device);

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

        // Create camera and transform data structures
        let camera_position = Vec3::new(0.0, 0.0, 3.0);
        let camera_direction = Vec3::new(0.0, 0.0, -1.0);
        let view_matrix = Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y);
        let projection_matrix =
            Mat4::perspective_rh(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0);
        let view_projection_matrix = projection_matrix * view_matrix;

        // Create transform uniform buffer with full structure expected by shaders
        let model_matrix = Mat4::IDENTITY;
        let model_view_matrix = view_matrix * model_matrix;
        let normal_matrix = model_view_matrix.inverse().transpose();

        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        struct FullTransformUniform {
            model_matrix: [[f32; 4]; 4],
            model_view_matrix: [[f32; 4]; 4],
            normal_matrix: [[f32; 4]; 3], // mat4x3 stored as 3 vec4 for alignment (192 bytes total)
            _padding: [f32; 4],
        }

        let full_transform_uniform = FullTransformUniform {
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
            contents: bytemuck::cast_slice(&[full_transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM,
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

        // Create camera uniform buffer with full structure expected by shaders

        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        struct FullCameraUniform {
            view_matrix: [[f32; 4]; 4],
            projection_matrix: [[f32; 4]; 4],
            view_projection_matrix: [[f32; 4]; 4],
            camera_position: [f32; 3],
            _padding1: f32,
            camera_direction: [f32; 3],
            _padding2: f32,
            log_depth_constant: f32,
            far_plane_distance: f32,
            near_plane_distance: f32,
            _padding3: f32,
        }

        let full_camera_uniform = FullCameraUniform {
            view_matrix: view_matrix.to_cols_array_2d(),
            projection_matrix: projection_matrix.to_cols_array_2d(),
            view_projection_matrix: view_projection_matrix.to_cols_array_2d(),
            camera_position: camera_position.to_array(),
            _padding1: 0.0,
            camera_direction: camera_direction.to_array(),
            _padding2: 0.0,
            log_depth_constant: 1.0,
            far_plane_distance: 100.0,
            near_plane_distance: 0.1,
            _padding3: 0.0,
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[full_camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

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

        // Create black hole uniforms bind group (camera, transform, black hole data)
        let black_hole_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Black Hole Uniform Bind Group"),
            layout: &black_hole_shader.uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: transform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: black_hole_shader.uniform_buffer.as_entire_binding(),
                },
            ],
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        Ok(Self {
            device,
            queue,
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
            camera_bind_group,
            cube_vertex_buffer,
            cube_index_buffer,
            cube_num_indices: cube_indices.len() as u32,
            sphere_vertex_buffer,
            sphere_index_buffer,
            sphere_num_indices: sphere_indices.len() as u32,
            quad_vertex_buffer,
            quad_index_buffer,
            quad_num_indices: quad_indices.len() as u32,
            line_vertex_buffer,
            line_index_buffer,
            line_num_indices: line_indices.len() as u32,
            point_vertex_buffer,
            point_index_buffer,
            point_num_indices: point_indices.len() as u32,
            max_view_distance: 100000000000.0, // Like Java MAXVIEWDISTANCE
            log_depth_constant: 1.0,           // Like Java LOGDEPTHCONSTANT
        })
    }

    /// Render a cube using default shader
    pub fn render_default_cube<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Update uniforms (in real usage, these would be provided as parameters)
        self.default_shader.update_uniforms(
            &self.queue,
            Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y),
            Mat4::perspective_rh(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0),
            Mat4::IDENTITY,
            Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(1.0, 1.0, 1.0),
            self.max_view_distance,
            self.log_depth_constant,
        );

        self.default_shader.render_geometry(
            render_pass,
            &self.cube_vertex_buffer,
            &self.cube_index_buffer,
            self.cube_num_indices,
        );
    }

    /// Render a sphere using default shader
    pub fn render_default_sphere<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Update uniforms
        self.default_shader.update_uniforms(
            &self.queue,
            Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y),
            Mat4::perspective_rh(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0),
            Mat4::IDENTITY,
            Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(1.0, 1.0, 1.0),
            self.max_view_distance,
            self.log_depth_constant,
        );

        self.default_shader.render_geometry(
            render_pass,
            &self.sphere_vertex_buffer,
            &self.sphere_index_buffer,
            self.sphere_num_indices,
        );
    }

    /// Render a planet with atmosphere using Earth textures
    pub fn render_atmospheric_planet<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Calculate positions relative to camera (as done in Java implementation)
        let view_matrix = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
        let projection_matrix =
            Mat4::perspective_rh(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0);
        let model_matrix = Mat4::IDENTITY;

        let camera_position = Vec3::new(0.0, 0.0, 3.0);
        let star_world_position = Vec3::new(5.0, 5.0, 5.0); // Star position in world space
        let planet_world_position = Vec3::ZERO; // Planet at origin

        // Transform to camera space (relative positions) as done in Java
        let star_relative = star_world_position - camera_position;
        let planet_relative = planet_world_position - camera_position;

        // Update atmosphere-specific uniforms
        self.planet_atmo_shader.update_uniforms(
            &self.queue,
            view_matrix,
            projection_matrix,
            model_matrix,
            star_relative,                 // Light position (relative to camera)
            Vec3::new(1.0, 1.0, 1.0),      // Light color (white)
            star_relative,                 // Star position (same as light)
            planet_relative,               // Planet position (relative to camera)
            Vec4::new(0.4, 0.6, 1.0, 1.0), // Earth-like atmosphere color (blue)
            0.1,                           // Overglow (standard value from Java)
            true,                          // Use Earth day/night textures
        );

        // Use the existing planet atmosphere shader's render method
        self.planet_atmo_shader.render_geometry(
            render_pass,
            &self.sphere_vertex_buffer,
            &self.sphere_index_buffer,
            self.sphere_num_indices,
        );
    }

    /// Get device reference for external use
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Create a standard sampler for texture rendering
    fn create_sampler(&self) -> wgpu::Sampler {
        self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        })
    }

    /// Get queue reference for external use
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Create texture bind groups for various shader types (DRY helper)
    fn create_texture_bind_groups(
        &self,
    ) -> AstrariaResult<(
        wgpu::BindGroup,
        wgpu::BindGroup,
        wgpu::BindGroup,
        wgpu::BindGroup,
    )> {
        // Create a default sampler for all textures
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Default Texture Sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });

        // Planet atmosphere bind group (day + night + atmospheric gradient)
        let planet_texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Planet Atmosphere Texture Bind Group"),
            layout: &self.planet_atmo_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.earth_day_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.earth_night_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&self.atmo_gradient_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Skybox bind group (cubemap)
        let skybox_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Skybox Texture Bind Group"),
            layout: &self.skybox_shader.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.skybox_cubemap.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Lens glow bind group (glow + spectrum textures)
        let lens_glow_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Lens Glow Texture Bind Group"),
            layout: &self.lens_glow_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.star_glow_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.star_spectrum_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Sun texture bind group (single texture)
        let sun_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Sun Texture Bind Group"),
            layout: &self.sun_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.sun_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Ok((
            planet_texture_bind_group,
            skybox_bind_group,
            lens_glow_bind_group,
            sun_bind_group,
        ))
    }

    /// Render sun with stellar surface texture
    pub fn render_sun<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Update sun shader uniforms
        self.sun_shader.update_uniforms(
            &self.queue,
            5778.0,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 3.0),
        );

        // Set all required bind groups in correct order
        render_pass.set_pipeline(&self.sun_shader.pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]); // Camera at index 0
        render_pass.set_bind_group(1, &self.transform_bind_group, &[]); // Transform at index 1
        render_pass.set_bind_group(2, &self.sun_shader.bind_group, &[]); // Sun uniforms at index 2
        render_pass.set_bind_group(3, &self.sun_texture_bind_group, &[]); // Texture at index 3
        render_pass.set_vertex_buffer(0, self.sphere_vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            self.sphere_index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..self.sphere_num_indices, 0, 0..1);
    }

    /// Render skybox with Milky Way cubemap
    pub fn render_skybox<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Set all required bind groups in correct order
        render_pass.set_pipeline(&self.skybox_shader.pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]); // Camera at index 0
        render_pass.set_bind_group(1, &self.skybox_texture_bind_group, &[]); // Texture at index 1
        render_pass.set_vertex_buffer(0, self.cube_vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.cube_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.cube_num_indices, 0, 0..1);
    }

    /// Render billboard sprite
    pub fn render_billboard<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Update billboard shader uniforms for test mode
        self.billboard_shader.update_simple_uniforms(
            &self.queue,
            Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y),
            Mat4::perspective_rh(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0),
        );

        // Use the billboard shader with proper quad geometry
        render_pass.set_pipeline(&self.billboard_shader.pipeline);
        render_pass.set_bind_group(0, &self.billboard_shader.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.quad_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.quad_num_indices, 0, 0..1);
    }

    /// Render lens glow effect with temperature-based colors
    pub fn render_lens_glow<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Update lens glow shader uniforms
        let view_matrix = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
        let projection_matrix =
            Mat4::perspective_rh(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0);
        self.lens_glow_shader
            .update_simple_uniforms(&self.queue, view_matrix, projection_matrix);

        // Use the lens glow shader with flare effects
        render_pass.set_pipeline(&self.lens_glow_shader.pipeline);
        render_pass.set_bind_group(0, &self.lens_glow_shader.bind_group, &[]); // Uniforms at index 0
        render_pass.set_bind_group(1, &self.lens_glow_texture_bind_group, &[]); // Textures at index 1
        render_pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.quad_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.quad_num_indices, 0, 0..1);
    }

    /// Render black hole with gravitational lensing effect
    pub fn render_black_hole<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Update black hole shader uniforms (position etc.)
        self.black_hole_shader.update_simple_uniforms(&self.queue);

        // Use the black hole shader with gravitational lensing
        render_pass.set_pipeline(&self.black_hole_shader.pipeline);
        render_pass.set_bind_group(0, &self.black_hole_uniform_bind_group, &[]); // Uniforms at index 0
        render_pass.set_bind_group(1, &self.black_hole_texture_bind_group, &[]); // Skybox texture at index 1
        render_pass.set_vertex_buffer(0, self.sphere_vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            self.sphere_index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..self.sphere_num_indices, 0, 0..1);
    }

    /// Render orbital lines
    pub fn render_line<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Update line shader uniforms for test mode
        self.line_shader.update_simple_uniforms(
            &self.queue,
            Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y),
            Mat4::perspective_rh(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0),
        );

        // Use the line shader with line geometry for orbital paths
        render_pass.set_pipeline(&self.line_shader.pipeline);
        render_pass.set_bind_group(0, &self.line_shader.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.line_vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.line_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.line_num_indices, 0, 0..1);
    }

    /// Render distant point objects
    pub fn render_point<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Update point shader uniforms for test mode
        self.point_shader.update_simple_uniforms(
            &self.queue,
            Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y),
            Mat4::perspective_rh(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0),
        );

        // Use point geometry for distant object rendering
        render_pass.set_pipeline(&self.point_shader.pipeline);
        render_pass.set_bind_group(0, &self.point_shader.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.point_vertex_buffer.slice(..));
        render_pass.draw(0..self.point_num_indices, 0..1);
    }
}
