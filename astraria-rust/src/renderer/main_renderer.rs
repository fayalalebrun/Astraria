use glam::{DMat4, DVec3, Mat4};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// Constants for dynamic MVP uniform buffer management
const MVP_UNIFORM_ALIGNED_SIZE: u32 = 256; // 256-byte alignment required for dynamic offsets
const MAX_OBJECTS_PER_FRAME: u32 = 64; // Support up to 64 objects per frame
/// Main Renderer that manages all shader types
/// Equivalent to the Java Renderer class that owns all shader instances
use wgpu::{Device, Queue, RenderPass};

use crate::{
    assets::{AssetManager, CubemapAsset, ModelAsset, TextureAsset},
    graphics::Mesh,
    renderer::{
        camera::Camera,
        core::{MeshType, RenderCommand, *},
        precision_math::{calculate_mvp_matrix_64bit, calculate_mvp_matrix_64bit_with_atmosphere},
        shaders::{
            BillboardShader, BlackHoleShader, DefaultShader, LensGlowShader, LineShader,
            PlanetAtmoShader, PointShader, SkyboxShader, SunShader,
        },
        uniforms::StandardMVPUniform,
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

    // Dynamic MVP uniform buffer for 64-bit precision calculations (supports multiple objects)
    pub mvp_uniform_buffer: wgpu::Buffer,
    pub mvp_bind_group: wgpu::BindGroup,

    // Current offset counter for dynamic buffer allocation (resets each frame)
    current_mvp_offset: u32,

    // Frame-level MVP data collection (written once per frame before render pass)
    frame_mvp_data: Vec<u8>, // Raw buffer data to write all at once

    // Render commands with their allocated MVP offsets (collected during preparation phase)
    prepared_render_commands: Vec<(RenderCommand, Mat4, u32)>, // (command, transform, mvp_offset)

    // Default shader bind groups
    pub default_lighting_bind_group: wgpu::BindGroup,
    pub default_texture_bind_group: wgpu::BindGroup,

    // Planet atmosphere shader bind groups
    pub planet_lighting_bind_group: wgpu::BindGroup,
    pub planet_texture_bind_group: wgpu::BindGroup,

    // Billboard shader bind group
    pub billboard_uniform_bind_group: wgpu::BindGroup,

    // Geometry meshes for testing
    cube_mesh: Mesh,
    sphere_model: Arc<ModelAsset>, // Use loaded OBJ model for sphere
    quad_mesh: Mesh,
    line_mesh: Mesh,
    point_mesh: Mesh,

    // Depth texture for rendering
    _depth_texture: wgpu::Texture,
    _depth_view: wgpu::TextureView,

    // Camera matrices for 64-bit precision calculations
    pub view_matrix_d64: DMat4,
    pub projection_matrix_d64: DMat4,
    pub view_projection_matrix_d64: DMat4,
    pub max_view_distance: f32,
    pub log_depth_constant: f32,
}

impl MainRenderer {
    pub async fn new() -> AstrariaResult<Self> {
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
            .ok_or_else(|| {
                AstrariaError::Graphics("Failed to find a suitable graphics adapter".to_string())
            })?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| AstrariaError::Graphics(format!("Failed to create device: {}", e)))?;

        Self::with_device(device, queue).await
    }

    pub async fn with_surface(
        instance: &wgpu::Instance,
        surface: wgpu::Surface,
    ) -> AstrariaResult<(Self, wgpu::Surface)> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| {
                AstrariaError::Graphics("Failed to find a suitable graphics adapter".to_string())
            })?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| AstrariaError::Graphics(format!("Failed to create device: {}", e)))?;

        let main_renderer = Self::with_device(device, queue).await?;
        Ok((main_renderer, surface))
    }

    pub async fn with_device(device: wgpu::Device, queue: wgpu::Queue) -> AstrariaResult<Self> {
        // Create depth texture
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: 800,
                height: 600,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create asset manager and load textures
        let mut asset_manager = AssetManager::new().await?;

        // Load all required textures (using actual asset paths)
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
        let sun_texture = asset_manager
            .load_texture(&device, &queue, "assets/Planet Textures/8k_sun.jpg")
            .await?;
        let skybox_cubemap = asset_manager
            .load_cubemap(
                &device,
                &queue,
                "milkyway",
                &[
                    "assets/skybox/MilkyWayXP.png", // +X (right)
                    "assets/skybox/MilkyWayXN.png", // -X (left)
                    "assets/skybox/MilkyWayYP.png", // +Y (top)
                    "assets/skybox/MilkyWayYN.png", // -Y (bottom)
                    "assets/skybox/MilkyWayZP.png", // +Z (front)
                    "assets/skybox/MilkyWayZN.png", // -Z (back)
                ],
            )
            .await?;
        let atmo_gradient_texture = asset_manager
            .load_texture(&device, &queue, "assets/atmoGradient.png")
            .await?;
        let star_glow_texture = asset_manager
            .load_texture(&device, &queue, "assets/star_glow.png")
            .await?;
        let star_spectrum_texture = asset_manager
            .load_texture(&device, &queue, "assets/star_spectrum_1D.png")
            .await?;

        // Load sphere.obj model
        let sphere_model = asset_manager
            .load_model(&device, "assets/models/sphere.obj")
            .await?;

        // Initialize camera
        let mut camera = Camera::new(800.0 / 600.0); // aspect ratio
        camera.position_relative_to_body(DVec3::ZERO, 5.0, 2.0); // Position at 10 units from origin

        // Create geometry meshes using test geometry
        use crate::graphics::test_geometry::{
            create_test_cube, create_test_line, create_test_point, create_test_quad,
        };

        let (cube_vertices, cube_indices) = create_test_cube();
        let cube_mesh = Mesh::new(&device, &cube_vertices, &cube_indices);

        // Keep the Arc reference to the sphere model

        let (quad_vertices, quad_indices) = create_test_quad();
        let quad_mesh = Mesh::new(&device, &quad_vertices, &quad_indices);

        let (line_vertices, line_indices) = create_test_line();
        let line_mesh = Mesh::new(&device, &line_vertices, &line_indices);

        let (point_vertices, point_indices) = create_test_point();
        let point_mesh = Mesh::new(&device, &point_vertices, &point_indices);

        // Create dynamic MVP bind group layout first
        use crate::renderer::uniforms::buffer_helpers::*;
        let mvp_bind_group_layout =
            create_mvp_bind_group_layout_dynamic(&device, Some("Dynamic MVP Bind Group Layout"));

        // Create shaders
        let default_shader = DefaultShader::new(&device)?;
        let planet_atmo_shader = PlanetAtmoShader::new(
            &device,
            &queue,
            &earth_day_texture.texture,
            &earth_night_texture.texture,
            &atmo_gradient_texture.texture,
        )?;
        let sun_shader = SunShader::new(&device, &mvp_bind_group_layout)?;
        let skybox_shader = SkyboxShader::new(&device)?;
        let billboard_shader = BillboardShader::new(&device, &queue)?;
        let lens_glow_shader = LensGlowShader::new(&device, &queue)?;
        let black_hole_shader = BlackHoleShader::new(&device, &queue)?;
        let line_shader = LineShader::new(&device, &queue)?;
        let point_shader = PointShader::new(&device, &queue)?;

        // Create dynamic MVP uniform buffer (large enough for 64 objects * 256 bytes each)
        let mvp_buffer_size = (MAX_OBJECTS_PER_FRAME * MVP_UNIFORM_ALIGNED_SIZE) as u64;
        let mvp_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dynamic MVP Uniform Buffer"),
            size: mvp_buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create dynamic MVP bind group using the already-created layout
        let mvp_bind_group = create_dynamic_mvp_bind_group(
            &device,
            &mvp_bind_group_layout,
            &mvp_uniform_buffer,
            Some("Dynamic MVP Bind Group"),
        );

        // Create default sampler
        let default_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // Create texture bind groups
        let sun_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Sun Texture Bind Group"),
            layout: &sun_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&sun_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&star_spectrum_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&default_sampler),
                },
            ],
        });

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
                    resource: wgpu::BindingResource::Sampler(&default_sampler),
                },
            ],
        });

        let black_hole_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Black Hole Texture Bind Group"),
            layout: &black_hole_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&skybox_cubemap.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&default_sampler),
                },
            ],
        });

        // Create black hole uniform bind group
        use crate::renderer::shaders::black_hole_shader::BlackHoleUniform;
        let black_hole_uniform = BlackHoleUniform {
            hole_position: [0.0, 0.0, 0.0],
            _padding: 0.0,
        };
        let black_hole_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Black Hole Uniform Buffer"),
                contents: bytemuck::cast_slice(&[black_hole_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let black_hole_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Black Hole Uniform Bind Group"),
            layout: &black_hole_shader.uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: black_hole_uniform_buffer.as_entire_binding(),
            }],
        });

        let lens_glow_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Lens Glow Texture Bind Group"),
            layout: &lens_glow_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&star_glow_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&star_spectrum_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&default_sampler),
                },
            ],
        });

        // Create transform buffer and bind group for older shaders
        let transform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Transform Uniform Buffer"),
            size: std::mem::size_of::<TransformUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Transform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transform Bind Group"),
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        // Create default shader lighting bind group
        use crate::renderer::shaders::default_shader::{LightingUniforms, PointLight};
        let default_lighting = LightingUniforms {
            lights: [PointLight {
                position: [5.0, 5.0, 5.0],
                _padding1: 0.0,
                ambient: [0.1, 0.1, 0.1],
                _padding2: 0.0,
                diffuse: [1.0, 1.0, 1.0],
                _padding3: 0.0,
                specular: [1.0, 1.0, 1.0],
                _padding4: 0.0,
            }; 8],
            num_lights: 1,
            _padding: [0.0, 0.0, 0.0],
        };
        let default_lighting_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Default Lighting Buffer"),
                contents: bytemuck::cast_slice(&[default_lighting]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let default_lighting_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Default Lighting Bind Group"),
            layout: &default_shader.lighting_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: default_lighting_buffer.as_entire_binding(),
            }],
        });

        let default_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Default Texture Bind Group"),
            layout: &default_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&earth_day_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&default_sampler),
                },
            ],
        });

        // Create planet atmosphere shader bind groups
        use crate::renderer::shaders::planet_atmo_shader::{AtmosphereUniform, LightingUniform};
        let planet_lighting = LightingUniform {
            lights: [crate::renderer::shaders::planet_atmo_shader::PointLight {
                position: [5.0, 5.0, 5.0],
                _padding1: 0.0,
                ambient: [0.1, 0.1, 0.1],
                _padding2: 0.0,
                diffuse: [1.0, 1.0, 1.0],
                _padding3: 0.0,
                specular: [1.0, 1.0, 1.0],
                _padding4: 0.0,
            }; 8],
            num_lights: 1,
            _padding: [0, 0, 0, 0, 0, 0, 0],
        };
        let planet_atmosphere = AtmosphereUniform {
            planet_position: [0.0, 0.0, 0.0],
            _padding1: 0.0,
            star_position: [5.0, 5.0, 5.0],
            _padding2: 0.0,
            atmosphere_color_mod: [0.4, 0.6, 1.0, 1.0],
            overglow: 0.1,
            use_ambient_texture: 1,
            _padding3: [0.0, 0.0],
        };

        let planet_lighting_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Planet Lighting Buffer"),
            contents: bytemuck::cast_slice(&[planet_lighting]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let planet_atmosphere_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Planet Atmosphere Buffer"),
                contents: bytemuck::cast_slice(&[planet_atmosphere]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let planet_lighting_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Planet Lighting Bind Group"),
            layout: &planet_atmo_shader.lighting_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: planet_lighting_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: planet_atmosphere_buffer.as_entire_binding(),
                },
            ],
        });

        let planet_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Planet Texture Bind Group"),
            layout: &planet_atmo_shader.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&earth_day_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&earth_night_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&atmo_gradient_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&default_sampler),
                },
            ],
        });

        // Create billboard uniform bind group
        use crate::renderer::shaders::billboard_shader::BillboardUniform;
        let billboard_uniform = BillboardUniform {
            billboard_width: 100.0,
            billboard_height: 100.0,
            screen_width: 800.0,
            screen_height: 600.0,
            billboard_origin: [0.0, 0.0, 0.0],
            _padding: 0.0,
        };
        let billboard_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Billboard Uniform Buffer"),
                contents: bytemuck::cast_slice(&[billboard_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let billboard_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Billboard Uniform Bind Group"),
            layout: &billboard_shader.billboard_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: billboard_uniform_buffer.as_entire_binding(),
            }],
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
            mvp_uniform_buffer,
            mvp_bind_group,
            current_mvp_offset: 0,
            frame_mvp_data: Vec::new(),
            prepared_render_commands: Vec::new(),
            default_lighting_bind_group,
            default_texture_bind_group,
            planet_lighting_bind_group,
            planet_texture_bind_group,
            billboard_uniform_bind_group,
            cube_mesh,
            sphere_model,
            quad_mesh,
            line_mesh,
            point_mesh,
            _depth_texture: depth_texture,
            _depth_view: depth_view,
            view_matrix_d64: DMat4::IDENTITY,
            projection_matrix_d64: DMat4::IDENTITY,
            view_projection_matrix_d64: DMat4::IDENTITY,
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
    pub fn update_camera(&mut self, _delta_time: f32) {
        // Camera movement is now handled directly through process_movement calls
        // GPU uniforms are calculated on-demand via get_uniform()
    }

    /// Compute MVP matrix using 64-bit precision to eliminate NaN issues at astronomical distances
    /// This function takes object position and scale in double precision and returns 32-bit uniform
    fn compute_mvp_matrix(
        &self,
        object_position: DVec3,
        object_scale: DVec3,
    ) -> StandardMVPUniform {
        let mvp_matrix = calculate_mvp_matrix_64bit(
            self.camera.position(),
            self.camera.direction(),
            self.camera.up(), // Use camera's actual up vector
            object_position,
            object_scale,
            self.camera.projection_matrix(),
            false, // Not skybox
        );

        StandardMVPUniform {
            mvp_matrix: mvp_matrix.to_cols_array_2d(),
            camera_position: self.camera.position().as_vec3().to_array(),
            _padding1: 0.0,
            camera_direction: self.camera.direction().to_array(),
            _padding2: 0.0,
            log_depth_constant: self.log_depth_constant,
            far_plane_distance: self.max_view_distance,
            near_plane_distance: 1.0,
            fc_constant: 2.0 / (self.max_view_distance + 1.0).ln(),
            camera_to_object_transform: Mat4::IDENTITY.to_cols_array_2d(), // Default
            light_direction_camera_space: [0.0, 0.0, -1.0],                // Default
            _padding3: 0.0,
        }
    }

    /// Compute MVP matrix with atmospheric data for planetary rendering
    /// Includes relative coordinates between star and planet for atmospheric scattering
    fn compute_mvp_matrix_with_atmosphere(
        &self,
        planet_position: DVec3,
        planet_scale: DVec3,
        star_position: DVec3,
    ) -> StandardMVPUniform {
        let (mvp_matrix, camera_to_object_transform, light_direction_camera_space) =
            calculate_mvp_matrix_64bit_with_atmosphere(
                self.camera.position(),
                self.camera.direction(),
                self.camera.up(), // Use camera's actual up vector
                planet_position,
                planet_scale,
                self.camera.projection_matrix(),
                false, // Not skybox
                Some(star_position),
            );

        StandardMVPUniform {
            mvp_matrix: mvp_matrix.to_cols_array_2d(),
            camera_position: self.camera.position().as_vec3().to_array(),
            _padding1: 0.0,
            camera_direction: self.camera.direction().to_array(),
            _padding2: 0.0,
            log_depth_constant: self.log_depth_constant,
            far_plane_distance: self.max_view_distance,
            near_plane_distance: 1.0,
            fc_constant: 2.0 / (self.max_view_distance + 1.0).ln(),
            camera_to_object_transform: camera_to_object_transform.to_cols_array_2d(),
            light_direction_camera_space: light_direction_camera_space.to_array(),
            _padding3: 0.0,
        }
    }

    /// Compute MVP matrix for skybox (with translation completely removed) using 64-bit precision
    /// This ensures the skybox always appears infinitely far away
    fn compute_skybox_mvp_matrix(&self) -> StandardMVPUniform {
        let mvp_matrix = calculate_mvp_matrix_64bit(
            self.camera.position(),
            self.camera.direction(),
            self.camera.up(), // Use camera's actual up vector
            DVec3::ZERO,      // Skybox doesn't need translation
            DVec3::ONE,       // Default scale
            self.camera.projection_matrix(),
            true, // Is skybox - removes translation
        );

        StandardMVPUniform {
            mvp_matrix: mvp_matrix.to_cols_array_2d(),
            camera_position: self.camera.position().as_vec3().to_array(),
            _padding1: 0.0,
            camera_direction: self.camera.direction().to_array(),
            _padding2: 0.0,
            log_depth_constant: self.log_depth_constant,
            far_plane_distance: self.max_view_distance,
            near_plane_distance: 1.0,
            fc_constant: 2.0 / (self.max_view_distance + 1.0).ln(),
            camera_to_object_transform: Mat4::IDENTITY.to_cols_array_2d(), // No transform for skybox
            light_direction_camera_space: [0.0, 0.0, -1.0],                // Not used for skybox
            _padding3: 0.0,
        }
    }

    /// Reset frame MVP data at the start of each frame
    pub fn begin_frame(&mut self) {
        self.current_mvp_offset = 0;
        self.frame_mvp_data.clear();
        self.prepared_render_commands.clear();
    }

    /// Allocate space for an MVP uniform in the dynamic buffer and return the offset
    fn allocate_mvp_uniform(&mut self, uniform: StandardMVPUniform) -> u32 {
        let offset = self.current_mvp_offset;

        // Check for buffer overflow
        if offset + MVP_UNIFORM_ALIGNED_SIZE > MAX_OBJECTS_PER_FRAME * MVP_UNIFORM_ALIGNED_SIZE {
            log::error!("MVP uniform buffer overflow! Too many objects in frame.");
            return offset; // Return current offset to avoid crash, but rendering may be incorrect
        }

        // Convert uniform to bytes and pad to 256-byte alignment
        let uniform_array = [uniform];
        let uniform_bytes = bytemuck::cast_slice(&uniform_array);
        let mut padded_data = vec![0u8; MVP_UNIFORM_ALIGNED_SIZE as usize];

        // Copy the actual uniform data (first 240 bytes)
        let copy_size = uniform_bytes.len().min(padded_data.len());
        padded_data[..copy_size].copy_from_slice(&uniform_bytes[..copy_size]);

        // Append to frame data
        self.frame_mvp_data.extend_from_slice(&padded_data);

        // Update offset for next allocation
        self.current_mvp_offset += MVP_UNIFORM_ALIGNED_SIZE;

        offset
    }

    /// Upload all frame MVP data to GPU buffer (call once per frame before render pass)
    pub fn upload_frame_mvp_data(&self) {
        if !self.frame_mvp_data.is_empty() {
            self.queue
                .write_buffer(&self.mvp_uniform_buffer, 0, &self.frame_mvp_data);
        }
    }

    /// Prepare a render command for later execution (allocates MVP uniform and stores command)
    /// This should be called during the preparation phase for each object to render
    pub fn prepare_render_command(&mut self, command: RenderCommand, transform: Mat4) {
        // Compute the appropriate MVP uniform based on command type
        let mvp_uniform = match &command {
            RenderCommand::Skybox => self.compute_skybox_mvp_matrix(),
            RenderCommand::AtmosphericPlanet {
                star_position,
                planet_position,
                ..
            } => {
                let (scale, _rotation, translation) = transform.to_scale_rotation_translation();
                let final_planet_position = planet_position.as_dvec3() + translation.as_dvec3();
                let final_star_position = star_position.as_dvec3();
                self.compute_mvp_matrix_with_atmosphere(
                    final_planet_position,
                    scale.as_dvec3(),
                    final_star_position,
                )
            }
            RenderCommand::Sun { star_position, .. } => {
                let (scale, _rotation, translation) = transform.to_scale_rotation_translation();
                self.compute_mvp_matrix(
                    star_position.as_dvec3() + translation.as_dvec3(),
                    scale.as_dvec3(),
                )
            }
            _ => {
                // Default case for other render commands
                let (scale, _rotation, translation) = transform.to_scale_rotation_translation();
                self.compute_mvp_matrix(translation.as_dvec3(), scale.as_dvec3())
            }
        };

        // Allocate MVP uniform and get offset
        let mvp_offset = self.allocate_mvp_uniform(mvp_uniform);

        // Store the command with its MVP offset for later execution
        self.prepared_render_commands
            .push((command, transform, mvp_offset));
    }

    /// Execute all prepared render commands with dynamic offsets
    /// This should be called within the render pass after uploading MVP data
    pub fn execute_prepared_commands<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        for (command, transform, mvp_offset) in &self.prepared_render_commands {
            self.execute_render_command_with_offset(render_pass, command, *transform, *mvp_offset);
        }
    }

    /// Execute a single render command with a specific dynamic MVP offset
    /// This is the core rendering logic separated from MVP computation
    fn execute_render_command_with_offset<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        command: &RenderCommand,
        transform: Mat4,
        mvp_offset: u32,
    ) {
        // Update transform buffer with the provided transform matrix
        let view_matrix = self.camera.view_matrix_f32();
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

        match command {
            RenderCommand::Default {
                mesh_type,
                light_position: _,
                light_color: _,
            } => {
                // No MVP computation - uses pre-allocated uniform with dynamic offset
                render_pass.set_pipeline(&self.default_shader.pipeline);
                render_pass.set_bind_group(0, &self.mvp_bind_group, &[mvp_offset]); // Dynamic offset
                render_pass.set_bind_group(1, &self.default_lighting_bind_group, &[]);
                render_pass.set_bind_group(2, &self.default_texture_bind_group, &[]);

                // Special handling for sphere to use OBJ model
                if matches!(mesh_type, MeshType::Sphere) {
                    render_pass.set_vertex_buffer(0, self.sphere_model.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(
                        self.sphere_model.index_buffer.slice(..),
                        wgpu::IndexFormat::Uint32,
                    );
                    render_pass.draw_indexed(0..self.sphere_model.num_indices, 0, 0..1);
                } else {
                    let mesh = self.get_mesh(mesh_type);
                    render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                    render_pass
                        .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
                }
            }

            RenderCommand::AtmosphericPlanet { .. } => {
                // No MVP computation - uses pre-allocated uniform with dynamic offset
                render_pass.set_pipeline(&self.planet_atmo_shader.pipeline);
                render_pass.set_bind_group(0, &self.mvp_bind_group, &[mvp_offset]); // Dynamic offset
                render_pass.set_bind_group(1, &self.planet_lighting_bind_group, &[]);
                render_pass.set_bind_group(2, &self.planet_texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.sphere_model.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.sphere_model.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.sphere_model.num_indices, 0, 0..1);
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

                render_pass.set_pipeline(&self.sun_shader.pipeline);
                render_pass.set_bind_group(0, &self.mvp_bind_group, &[mvp_offset]); // Dynamic offset
                render_pass.set_bind_group(1, &self.sun_shader.bind_group, &[]);
                render_pass.set_bind_group(2, &self.sun_texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.sphere_model.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.sphere_model.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.sphere_model.num_indices, 0, 0..1);
            }

            RenderCommand::Skybox => {
                render_pass.set_pipeline(&self.skybox_shader.pipeline);
                render_pass.set_bind_group(0, &self.mvp_bind_group, &[mvp_offset]); // Dynamic offset
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
                render_pass.set_bind_group(0, &self.mvp_bind_group, &[mvp_offset]); // Dynamic offset
                render_pass.set_bind_group(1, &self.billboard_uniform_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.quad_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.quad_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.quad_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::LensGlow => {
                render_pass.set_pipeline(&self.lens_glow_shader.pipeline);
                render_pass.set_bind_group(0, &self.mvp_bind_group, &[mvp_offset]); // Dynamic offset
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
                render_pass.set_bind_group(0, &self.mvp_bind_group, &[mvp_offset]); // Dynamic offset
                render_pass.set_bind_group(1, &self.black_hole_uniform_bind_group, &[]);
                render_pass.set_bind_group(2, &self.black_hole_texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.sphere_model.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.sphere_model.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.sphere_model.num_indices, 0, 0..1);
            }

            RenderCommand::Line { color: _ } => {
                render_pass.set_pipeline(&self.line_shader.pipeline);
                render_pass.set_bind_group(0, &self.mvp_bind_group, &[mvp_offset]); // Dynamic offset
                render_pass.set_bind_group(1, &self.line_shader.line_bind_group, &[]); // Line color uniform
                render_pass.set_vertex_buffer(0, self.line_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.line_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.line_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::Point => {
                render_pass.set_pipeline(&self.point_shader.pipeline);
                render_pass.set_bind_group(0, &self.mvp_bind_group, &[mvp_offset]); // Dynamic offset
                render_pass.set_vertex_buffer(0, self.point_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.point_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.point_mesh.num_indices, 0, 0..1);
            }
        }
    }

    /// Legacy helper method for single command execution using two-phase approach
    /// Use this only for simple cases - prefer the full two-phase approach for multiple objects
    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        command: &RenderCommand,
        transform: Mat4,
    ) {
        self.begin_frame();
        self.prepare_render_command(command.clone(), transform);
        self.upload_frame_mvp_data();
        self.execute_prepared_commands(render_pass);
    }

    /// Helper method to get the appropriate mesh for a given mesh type
    fn get_mesh(&self, mesh_type: &MeshType) -> &Mesh {
        match mesh_type {
            MeshType::Cube => &self.cube_mesh,
            MeshType::Sphere => panic!("Sphere mesh should use sphere_model directly"),
            MeshType::Quad => &self.quad_mesh,
            MeshType::Line => &self.line_mesh,
            MeshType::Point => &self.point_mesh,
        }
    }
}
