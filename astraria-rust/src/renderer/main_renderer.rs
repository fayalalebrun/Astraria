use glam::{DMat4, DVec3, Mat4, Vec4Swizzles};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::{Device, Queue, RenderPass};

use crate::{
    AstrariaError, AstrariaResult,
    assets::{AssetManager, CubemapAsset, ModelAsset, TextureAsset},
    generated_shaders,
    graphics::{Mesh, SkyboxMesh},
    renderer::{
        camera::Camera,
        core::{MeshType, RenderCommand, *},
        occlusion::OcclusionSystem,
        precision_math::calculate_mvp_matrix_64bit_with_atmosphere,
        shaders::{
            BillboardShader, BlackHoleShader, DefaultShader, LensGlowShader, LineShader,
            PlanetAtmoShader, PointShader, SkyboxShader, SunShader,
        },
    },
};

/// Main rendering coordinator that manages all specialized shaders
/// Based on the Java Renderer.java architecture - now using generated bind groups
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

    // Generated bind groups for each shader type
    pub default_lighting_bind_group: generated_shaders::default::bind_groups::BindGroup1,
    pub default_texture_bind_group: generated_shaders::default::bind_groups::BindGroup2,

    pub planet_lighting_bind_group: generated_shaders::planet_atmo::bind_groups::BindGroup1,
    pub planet_texture_bind_group: generated_shaders::planet_atmo::bind_groups::BindGroup2,

    pub sun_uniform_bind_group: generated_shaders::sun_shader::bind_groups::BindGroup1,
    pub sun_texture_bind_group: generated_shaders::sun_shader::bind_groups::BindGroup2,

    pub skybox_texture_bind_group: generated_shaders::skybox::bind_groups::BindGroup1,

    // Billboard shader only uses MVP bind group (BindGroup0), no separate uniform bind group needed
    pub black_hole_uniform_bind_group: generated_shaders::black_hole::bind_groups::BindGroup1,
    pub black_hole_texture_bind_group: generated_shaders::black_hole::bind_groups::BindGroup2,

    pub lens_glow_uniform_bind_group: generated_shaders::lens_glow::bind_groups::BindGroup1,

    pub line_uniform_bind_group: generated_shaders::line::bind_groups::BindGroup1,

    pub point_uniform_bind_group: generated_shaders::point::bind_groups::BindGroup1,

    // Per-object MVP uniform buffers (no more dynamic offsets)
    mvp_buffers: Vec<wgpu::Buffer>,
    pub mvp_bind_groups: Vec<(generated_shaders::default::bind_groups::BindGroup0, usize)>, // (bind_group, buffer_index)

    // Prepared render commands with their MVP bind groups
    prepared_render_commands: Vec<(RenderCommand, Mat4, usize)>, // (command, transform, mvp_bind_group_index)

    // Geometry meshes for testing
    cube_mesh: Mesh,
    skybox_mesh: SkyboxMesh, // Separate mesh for skybox with correct vertex type
    sphere_model: Arc<ModelAsset>, // Use loaded OBJ model for sphere
    quad_mesh: Mesh,
    line_mesh: Mesh,
    point_mesh: Mesh,

    // Depth texture for rendering
    _depth_texture: wgpu::Texture,
    _depth_view: wgpu::TextureView,

    // Default sampler for textures
    default_sampler: wgpu::Sampler,

    // Camera matrices for 64-bit precision calculations
    pub view_matrix_d64: DMat4,
    pub projection_matrix_d64: DMat4,
    pub view_projection_matrix_d64: DMat4,

    // Simplified occlusion query system for lens glow visibility testing
    occlusion_system: OcclusionSystem,
    pub max_view_distance: f32,
    pub log_depth_constant: f32,
}

impl MainRenderer {
    /// Create a dynamic lighting bind group for regular planets (default shader)
    fn create_planet_lighting_bind_group(
        &self,
        planet_world_pos: glam::DVec3,
        sun_world_pos: glam::DVec3,
    ) -> AstrariaResult<generated_shaders::default::bind_groups::BindGroup1> {
        log::debug!("MainRenderer: Creating dynamic lighting bind group for regular planet");

        // Calculate light direction from planet to sun in world space
        let light_direction_world = (sun_world_pos - planet_world_pos).normalize();

        // Create the lighting uniform with computed light direction
        let lighting_uniform = generated_shaders::default::LightingUniforms {
            lights: [generated_shaders::default::DirectionalLight {
                // Light direction FROM object TO sun in WORLD SPACE
                direction: light_direction_world.as_vec3(),
                _padding1: 0.0,
                ambient: glam::Vec3::new(0.1, 0.1, 0.1),
                _padding2: 0.0,
                diffuse: glam::Vec3::new(1.0, 1.0, 1.0),
                _padding3: 0.0,
                specular: glam::Vec3::new(1.0, 1.0, 1.0),
                _padding4: 0.0,
            }; 8],
            num_lights: 1,
            _padding: [glam::Vec4::ZERO; 10], // Match default shader structure
        };

        // Create lighting buffer using unsafe raw bytes (since structs don't implement Pod)
        let lighting_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Dynamic Planet Lighting Uniform Buffer"),
                contents: unsafe {
                    std::slice::from_raw_parts(
                        &lighting_uniform as *const _ as *const u8,
                        std::mem::size_of::<generated_shaders::default::LightingUniforms>(),
                    )
                },
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        // Create the bind group with dynamic lighting uniform
        let bind_group = generated_shaders::default::bind_groups::BindGroup1::from_bindings(
            &self.device,
            generated_shaders::default::bind_groups::BindGroupLayout1 {
                lighting: wgpu::BufferBinding {
                    buffer: &lighting_buffer,
                    offset: 0,
                    size: None,
                },
            },
        );

        log::debug!(
            "MainRenderer: Successfully created dynamic lighting bind group for regular planet"
        );
        Ok(bind_group)
    }

    /// Create a dynamic lens glow uniform bind group with physics-calculated size and visibility
    fn create_lens_glow_uniform_bind_group(
        &self,
        glow_size: f32,
        star_id: u32,
        star_position: DVec3,
        camera_direction: DVec3,
        temperature: f32,
    ) -> AstrariaResult<generated_shaders::lens_glow::bind_groups::BindGroup1> {
        let visibility_factor = self.get_star_visibility(star_id);

        // TODO: Update when generated structure matches WESL with visibility_factor
        let lens_glow_uniform = generated_shaders::lens_glow::LensGlowUniform {
            glow_size,
            screen_width: 800.0,  // TODO: Get from actual surface config
            screen_height: 800.0, // TODO: Get from actual surface config
        };

        // Create uniform buffer
        let uniform_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Dynamic Lens Glow Uniform Buffer"),
                contents: unsafe {
                    std::slice::from_raw_parts(
                        &lens_glow_uniform as *const _ as *const u8,
                        std::mem::size_of::<generated_shaders::lens_glow::LensGlowUniform>(),
                    )
                },
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        // Create the bind group with glow texture and sampler (updated to match generated structure)
        let bind_group = generated_shaders::lens_glow::bind_groups::BindGroup1::from_bindings(
            &self.device,
            generated_shaders::lens_glow::bind_groups::BindGroupLayout1 {
                lens_glow: wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: None,
                },
                glow_texture: &self.star_glow_texture.view,
                glow_sampler: &self.default_sampler,
            },
        );

        Ok(bind_group)
    }

    /// Create a dynamic lighting bind group for atmospheric planets with specific atmospheric color
    fn create_atmospheric_lighting_bind_group(
        &self,
        atmosphere_color: glam::Vec4,
        overglow: f32,
        use_ambient_texture: bool,
        planet_world_pos: glam::DVec3,
        sun_world_pos: glam::DVec3,
    ) -> AstrariaResult<generated_shaders::planet_atmo::bind_groups::BindGroup1> {
        log::debug!(
            "MainRenderer: Creating dynamic lighting bind group with atmo_color: {:?}, overglow: {}, use_ambient: {}",
            atmosphere_color,
            overglow,
            use_ambient_texture
        );

        // Calculate light direction from planet to sun in world space
        let light_direction_world = (sun_world_pos - planet_world_pos).normalize();

        // Create the lighting uniform with computed light direction
        let lighting_uniform = generated_shaders::planet_atmo::LightingUniform {
            lights: [generated_shaders::planet_atmo::DirectionalLight {
                // Light direction FROM planet TO sun in WORLD SPACE
                direction: light_direction_world.as_vec3(),
                _padding1: 0.0,
                ambient: glam::Vec3::new(0.1, 0.1, 0.1),
                _padding2: 0.0,
                diffuse: glam::Vec3::new(1.0, 1.0, 1.0),
                _padding3: 0.0,
                specular: glam::Vec3::new(1.0, 1.0, 1.0),
                _padding4: 0.0,
            }; 8],
            num_lights: 1,
            _padding: [glam::Vec4::ZERO; 16],
        };

        // Create the atmospheric uniform data
        let atmospheric_uniform = generated_shaders::planet_atmo::AtmosphereUniform {
            atmosphere_color_mod: atmosphere_color,
            overglow,
            use_ambient_texture: if use_ambient_texture { 1 } else { 0 },
            _padding: glam::Vec2::ZERO,
        };

        // Create lighting buffer using unsafe raw bytes (since structs don't implement Pod)
        let lighting_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Dynamic Lighting Uniform Buffer"),
                contents: unsafe {
                    std::slice::from_raw_parts(
                        &lighting_uniform as *const _ as *const u8,
                        std::mem::size_of::<generated_shaders::planet_atmo::LightingUniform>(),
                    )
                },
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        // Create atmospheric buffer using unsafe raw bytes
        let atmospheric_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Dynamic Atmospheric Uniform Buffer"),
                    contents: unsafe {
                        std::slice::from_raw_parts(
                            &atmospheric_uniform as *const _ as *const u8,
                            std::mem::size_of::<generated_shaders::planet_atmo::AtmosphereUniform>(
                            ),
                        )
                    },
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        // Create the bind group with dynamic uniforms
        let bind_group = generated_shaders::planet_atmo::bind_groups::BindGroup1::from_bindings(
            &self.device,
            generated_shaders::planet_atmo::bind_groups::BindGroupLayout1 {
                lighting: wgpu::BufferBinding {
                    buffer: &lighting_buffer,
                    offset: 0,
                    size: None,
                },
                atmosphere: wgpu::BufferBinding {
                    buffer: &atmospheric_buffer,
                    offset: 0,
                    size: None,
                },
            },
        );

        log::debug!("MainRenderer: Successfully created dynamic lighting bind group");
        Ok(bind_group)
    }

    /// Create a dynamic texture bind group for atmospheric planets
    fn create_atmospheric_texture_bind_group(
        &self,
        main_texture_path: &str,
        ambient_texture_path: Option<&str>,
        use_ambient: bool,
    ) -> AstrariaResult<generated_shaders::planet_atmo::bind_groups::BindGroup2> {
        log::debug!(
            "MainRenderer: Creating dynamic bind group for textures: main={}, ambient={:?}",
            main_texture_path,
            ambient_texture_path
        );

        // Get the main texture from the asset manager - try multiple possible cache keys
        let main_texture = self
            .asset_manager
            .get_texture_handle(main_texture_path)
            .or_else(|| {
                self.asset_manager
                    .get_texture_handle(&format!("assets/{}", main_texture_path))
            })
            .or_else(|| {
                self.asset_manager
                    .get_texture_handle(&format!("./{}", main_texture_path))
            })
            .ok_or_else(|| {
                log::error!(
                    "MainRenderer: Texture cache keys available: {:?}",
                    self.asset_manager.cache_stats()
                );
                crate::AstrariaError::AssetLoading(format!(
                    "Main texture not found in cache: {} (tried keys: '{}', 'assets/{}', './{})'",
                    main_texture_path, main_texture_path, main_texture_path, main_texture_path
                ))
            })?;

        // Get the ambient texture (or fallback to main texture if not available)
        let ambient_texture = if use_ambient {
            if let Some(ambient_path) = ambient_texture_path {
                self.asset_manager.get_texture_handle(ambient_path)
                    .unwrap_or_else(|| {
                        log::warn!("MainRenderer: Ambient texture {} not found, using main texture as fallback", ambient_path);
                        Arc::clone(&main_texture)
                    })
            } else {
                log::warn!(
                    "MainRenderer: use_ambient is true but no ambient path provided, using main texture"
                );
                Arc::clone(&main_texture)
            }
        } else {
            Arc::clone(&main_texture)
        };

        // Get the atmospheric gradient texture (should be pre-loaded)
        let atmo_gradient = self
            .asset_manager
            .get_texture_handle("atmoGradient.png")
            .or_else(|| {
                self.asset_manager
                    .get_texture_handle("assets/atmoGradient.png")
            })
            .unwrap_or_else(|| {
                log::warn!(
                    "MainRenderer: Atmospheric gradient not found, using main texture as fallback"
                );
                Arc::clone(&main_texture)
            });

        // Create the bind group with the dynamic textures
        let bind_group = generated_shaders::planet_atmo::bind_groups::BindGroup2::from_bindings(
            &self.device,
            generated_shaders::planet_atmo::bind_groups::BindGroupLayout2 {
                ambient_texture: &ambient_texture.view,
                diffuse_texture: &main_texture.view,
                atmosphere_gradient_texture: &atmo_gradient.view,
                texture_sampler: &self.default_sampler,
            },
        );

        log::debug!("MainRenderer: Successfully created dynamic texture bind group");
        Ok(bind_group)
    }

    /// Load textures for all bodies in a scenario
    pub async fn load_scenario_textures(
        &mut self,
        asset_manager: &mut crate::assets::AssetManager,
        scenario: &crate::scenario::Scenario,
    ) -> AstrariaResult<()> {
        log::info!(
            "MainRenderer: Loading textures for {} bodies in scenario",
            scenario.bodies.len()
        );

        for body in &scenario.bodies {
            match &body.body_type {
                crate::scenario::BodyType::Planet { texture_path, .. }
                | crate::scenario::BodyType::Star { texture_path, .. } => {
                    match asset_manager
                        .load_texture(&self.device, &self.queue, texture_path)
                        .await
                    {
                        Ok(_) => {
                            // Also load into MainRenderer's asset manager for cache consistency
                            let _ = self
                                .asset_manager
                                .load_texture(&self.device, &self.queue, texture_path)
                                .await;
                        }
                        Err(e) => {
                            log::warn!(
                                "MainRenderer: Failed to load texture {} for {}: {}",
                                texture_path,
                                body.name,
                                e
                            );
                        }
                    }
                }
                crate::scenario::BodyType::PlanetAtmo {
                    texture_path,
                    ambient_texture,
                    ..
                } => {
                    match asset_manager
                        .load_texture(&self.device, &self.queue, texture_path)
                        .await
                    {
                        Ok(_) => {
                            // Also load into MainRenderer's asset manager for cache consistency
                            let _ = self
                                .asset_manager
                                .load_texture(&self.device, &self.queue, texture_path)
                                .await;
                        }
                        Err(e) => {
                            log::warn!(
                                "MainRenderer: Failed to load main texture {} for {}: {}",
                                texture_path,
                                body.name,
                                e
                            );
                        }
                    }

                    if let Some(ambient_path) = ambient_texture {
                        match asset_manager
                            .load_texture(&self.device, &self.queue, ambient_path)
                            .await
                        {
                            Ok(_) => {
                                // Also load into MainRenderer's asset manager for cache consistency
                                let _ = self
                                    .asset_manager
                                    .load_texture(&self.device, &self.queue, ambient_path)
                                    .await;
                            }
                            Err(e) => {
                                log::warn!(
                                    "MainRenderer: Failed to load ambient texture {} for {}: {}",
                                    ambient_path,
                                    body.name,
                                    e
                                );
                            }
                        }
                    }
                }
                _ => {} // Black holes don't have textures
            }
        }

        let (textures, models, cubemaps) = self.asset_manager.cache_stats();
        log::info!(
            "MainRenderer: Scenario texture loading complete - MainRenderer cache now has: textures={}, models={}, cubemaps={}",
            textures,
            models,
            cubemaps
        );
        Ok(())
    }

    pub async fn new() -> AstrariaResult<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
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
            .map_err(|e| {
                AstrariaError::Graphics(format!("Failed to find a suitable graphics adapter: {e}"))
            })?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::default(),
            })
            .await
            .map_err(|e| AstrariaError::Graphics(format!("Failed to create device: {e}")))?;

        Self::with_device(device, queue).await
    }

    pub async fn with_surface<'a>(
        instance: &'a wgpu::Instance,
        surface: wgpu::Surface<'a>,
    ) -> AstrariaResult<(Self, wgpu::Surface<'static>)> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| {
                AstrariaError::Graphics(format!("Failed to find a suitable graphics adapter: {e}"))
            })?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::default(),
            })
            .await
            .map_err(|e| AstrariaError::Graphics(format!("Failed to create device: {e}")))?;

        let main_renderer = Self::with_device(device, queue).await?;
        let surface: wgpu::Surface<'static> = unsafe { std::mem::transmute(surface) };
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
            create_skybox_cube, create_test_cube, create_test_line, create_test_point,
            create_test_quad,
        };

        let (cube_vertices, cube_indices) = create_test_cube();
        let cube_mesh = Mesh::new(&device, &cube_vertices, &cube_indices);

        let (skybox_vertices, skybox_indices) = create_skybox_cube();
        let skybox_mesh = SkyboxMesh::new(&device, &skybox_vertices, &skybox_indices);

        // Keep the Arc reference to the sphere model

        let (quad_vertices, quad_indices) = create_test_quad();
        let quad_mesh = Mesh::new(&device, &quad_vertices, &quad_indices);

        let (line_vertices, line_indices) = create_test_line();
        let line_mesh = Mesh::new(&device, &line_vertices, &line_indices);

        let (point_vertices, point_indices) = create_test_point();
        let point_mesh = Mesh::new(&device, &point_vertices, &point_indices);

        // Create shaders
        let default_shader = DefaultShader::new(&device)?;
        let planet_atmo_shader = PlanetAtmoShader::new(&device, &queue)?;
        let sun_shader = SunShader::new(&device, &queue)?;
        let skybox_shader = SkyboxShader::new(&device)?;
        let billboard_shader = BillboardShader::new(&device, &queue)?;
        let lens_glow_shader = LensGlowShader::new(&device, &queue)?;
        let black_hole_shader = BlackHoleShader::new(&device, &queue)?;
        let line_shader = LineShader::new(&device, &queue)?;
        let point_shader = PointShader::new(&device, &queue)?;

        // Initialize MVP buffers and bind groups storage
        let mvp_buffers = Vec::new();
        let mvp_bind_groups = Vec::new();

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

        // Create generated texture bind groups
        let sun_texture_bind_group =
            generated_shaders::sun_shader::bind_groups::BindGroup2::from_bindings(
                &device,
                generated_shaders::sun_shader::bind_groups::BindGroupLayout2 {
                    diffuse_texture: &sun_texture.view,
                    sun_gradient_texture: &star_spectrum_texture.view,
                    texture_sampler: &default_sampler,
                },
            );

        let skybox_texture_bind_group =
            generated_shaders::skybox::bind_groups::BindGroup1::from_bindings(
                &device,
                generated_shaders::skybox::bind_groups::BindGroupLayout1 {
                    skybox_texture: &skybox_cubemap.view,
                    skybox_sampler: &default_sampler,
                },
            );

        let black_hole_texture_bind_group =
            generated_shaders::black_hole::bind_groups::BindGroup2::from_bindings(
                &device,
                generated_shaders::black_hole::bind_groups::BindGroupLayout2 {
                    accretion_texture: &star_glow_texture.view,
                    texture_sampler: &default_sampler,
                },
            );

        // Create black hole uniform bind group using generated types
        #[repr(C)]
        #[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        struct BlackHoleUniformPod {
            schwarzschild_radius: f32,
            event_horizon: f32,
            accretion_disk_inner: f32,
            accretion_disk_outer: f32,
            black_hole_position: [f32; 3], // Array version for Pod compatibility
            _padding: f32,
        }

        let black_hole_uniform_pod = BlackHoleUniformPod {
            schwarzschild_radius: 2.0,
            event_horizon: 1.0,
            accretion_disk_inner: 3.0,
            accretion_disk_outer: 6.0,
            black_hole_position: [0.0, 0.0, 0.0],
            _padding: 0.0,
        };
        let black_hole_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Black Hole Uniform Buffer"),
                contents: bytemuck::cast_slice(&[black_hole_uniform_pod]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let black_hole_uniform_bind_group =
            generated_shaders::black_hole::bind_groups::BindGroup1::from_bindings(
                &device,
                generated_shaders::black_hole::bind_groups::BindGroupLayout1 {
                    black_hole: wgpu::BufferBinding {
                        buffer: &black_hole_uniform_buffer,
                        offset: 0,
                        size: None,
                    },
                },
            );

        // Create lens glow uniform bind group using generated types
        let lens_glow_uniform = generated_shaders::lens_glow::LensGlowUniform {
            glow_size: 10.0,      // Default size
            screen_width: 800.0,  // Default test screen dimensions
            screen_height: 800.0, // Default test screen dimensions
        };
        let lens_glow_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Lens Glow Uniform Buffer"),
                contents: unsafe {
                    std::slice::from_raw_parts(
                        &lens_glow_uniform as *const _ as *const u8,
                        std::mem::size_of::<generated_shaders::lens_glow::LensGlowUniform>(),
                    )
                },
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let lens_glow_uniform_bind_group =
            generated_shaders::lens_glow::bind_groups::BindGroup1::from_bindings(
                &device,
                generated_shaders::lens_glow::bind_groups::BindGroupLayout1 {
                    lens_glow: wgpu::BufferBinding {
                        buffer: &lens_glow_uniform_buffer,
                        offset: 0,
                        size: None,
                    },
                    glow_texture: &star_glow_texture.view,
                    glow_sampler: &default_sampler,
                },
            );

        // Transform buffer system is no longer needed with generated bind groups

        // Create default shader lighting bind group using generated types
        let default_lighting = generated_shaders::default::LightingUniforms {
            lights: [generated_shaders::default::DirectionalLight {
                // Default sun direction: coming from upper right (WORLD SPACE)
                direction: glam::Vec3::new(1.0, 1.0, -1.0), // Will be normalized in shader
                _padding1: 0.0,
                ambient: glam::Vec3::new(0.1, 0.1, 0.1),
                _padding2: 0.0,
                diffuse: glam::Vec3::new(1.0, 1.0, 1.0),
                _padding3: 0.0,
                specular: glam::Vec3::new(1.0, 1.0, 1.0),
                _padding4: 0.0,
            }; 8],
            num_lights: 1,
            _padding: [glam::Vec4::ZERO; 10], // Array of 10 Vec4s for proper alignment
        };
        let default_lighting_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Default Lighting Buffer"),
                contents: unsafe {
                    std::slice::from_raw_parts(
                        &default_lighting as *const _ as *const u8,
                        std::mem::size_of::<generated_shaders::default::LightingUniforms>(),
                    )
                },
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let default_lighting_bind_group =
            generated_shaders::default::bind_groups::BindGroup1::from_bindings(
                &device,
                generated_shaders::default::bind_groups::BindGroupLayout1 {
                    lighting: wgpu::BufferBinding {
                        buffer: &default_lighting_buffer,
                        offset: 0,
                        size: None,
                    },
                },
            );

        let default_texture_bind_group =
            generated_shaders::default::bind_groups::BindGroup2::from_bindings(
                &device,
                generated_shaders::default::bind_groups::BindGroupLayout2 {
                    diffuse_texture: &earth_day_texture.view,
                    diffuse_sampler: &default_sampler,
                },
            );

        // Create planet atmosphere shader bind groups using generated types
        let planet_lighting = generated_shaders::planet_atmo::LightingUniform {
            lights: [generated_shaders::planet_atmo::DirectionalLight {
                // Sun direction in WORLD SPACE - should be computed from actual sun position
                // For now using a default direction
                direction: glam::Vec3::new(1.0, 0.0, 0.0), // Light coming from +X direction
                _padding1: 0.0,
                ambient: glam::Vec3::new(0.1, 0.1, 0.1),
                _padding2: 0.0,
                diffuse: glam::Vec3::new(1.0, 1.0, 1.0),
                _padding3: 0.0,
                specular: glam::Vec3::new(1.0, 1.0, 1.0),
                _padding4: 0.0,
            }; 8],
            num_lights: 1,
            _padding: [glam::Vec4::ZERO; 16],
        };
        let planet_atmosphere = generated_shaders::planet_atmo::AtmosphereUniform {
            atmosphere_color_mod: glam::Vec4::new(0.4, 0.6, 1.0, 1.0),
            overglow: 0.1,
            use_ambient_texture: 1,
            _padding: glam::Vec2::new(0.0, 0.0),
        };

        let planet_lighting_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Planet Lighting Buffer"),
            contents: unsafe {
                std::slice::from_raw_parts(
                    &planet_lighting as *const _ as *const u8,
                    std::mem::size_of::<generated_shaders::planet_atmo::LightingUniform>(),
                )
            },
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let planet_atmosphere_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Planet Atmosphere Buffer"),
                contents: unsafe {
                    std::slice::from_raw_parts(
                        &planet_atmosphere as *const _ as *const u8,
                        std::mem::size_of::<generated_shaders::planet_atmo::AtmosphereUniform>(),
                    )
                },
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let planet_lighting_bind_group =
            generated_shaders::planet_atmo::bind_groups::BindGroup1::from_bindings(
                &device,
                generated_shaders::planet_atmo::bind_groups::BindGroupLayout1 {
                    lighting: wgpu::BufferBinding {
                        buffer: &planet_lighting_buffer,
                        offset: 0,
                        size: None,
                    },
                    atmosphere: wgpu::BufferBinding {
                        buffer: &planet_atmosphere_buffer,
                        offset: 0,
                        size: None,
                    },
                },
            );

        let planet_texture_bind_group =
            generated_shaders::planet_atmo::bind_groups::BindGroup2::from_bindings(
                &device,
                generated_shaders::planet_atmo::bind_groups::BindGroupLayout2 {
                    ambient_texture: &earth_night_texture.view,
                    diffuse_texture: &earth_day_texture.view,
                    atmosphere_gradient_texture: &atmo_gradient_texture.view,
                    texture_sampler: &default_sampler,
                },
            );

        // Billboard shader only uses MVP bind group, no separate uniform needed

        // Create sun uniform bind group using generated types
        let sun_uniform = generated_shaders::sun_shader::SunUniform {
            temperature: 5778.0, // Default sun temperature
            _padding1: 0.0,
            _padding2: 0.0,
            _padding3: 0.0,
            camera_to_sun_direction: glam::Vec3::ZERO,
            _padding4: 0.0,
            sun_position: glam::Vec3::ZERO,
            _padding5: 0.0,
            _padding6: glam::Vec4::ZERO,
            _padding7: glam::Vec4::ZERO,
            _padding8: glam::Vec4::ZERO,
        };

        // Since generated types don't implement Pod, we need to serialize manually for now
        // TODO: Implement Pod for generated types or use a different serialization method
        let sun_uniform_data = unsafe {
            std::slice::from_raw_parts(
                &sun_uniform as *const _ as *const u8,
                std::mem::size_of::<generated_shaders::sun_shader::SunUniform>(),
            )
        };

        let sun_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sun Uniform Buffer"),
            contents: sun_uniform_data,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let sun_uniform_bind_group =
            generated_shaders::sun_shader::bind_groups::BindGroup1::from_bindings(
                &device,
                generated_shaders::sun_shader::bind_groups::BindGroupLayout1 {
                    sun: wgpu::BufferBinding {
                        buffer: &sun_uniform_buffer,
                        offset: 0,
                        size: None,
                    },
                },
            );

        // Lens glow now only uses MVP, no uniform bind group needed

        // Create line uniform bind group using generated types
        let line_uniform = generated_shaders::line::LineUniform {
            line_color: glam::Vec4::new(1.0, 1.0, 1.0, 1.0), // Default white color
            line_width: 1.0,
            _padding1: 0.0,
            _padding2: 0.0,
            _padding3: 0.0,
        };
        let line_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Uniform Buffer"),
            contents: unsafe {
                std::slice::from_raw_parts(
                    &line_uniform as *const _ as *const u8,
                    std::mem::size_of::<generated_shaders::line::LineUniform>(),
                )
            },
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let line_uniform_bind_group =
            generated_shaders::line::bind_groups::BindGroup1::from_bindings(
                &device,
                generated_shaders::line::bind_groups::BindGroupLayout1 {
                    line: wgpu::BufferBinding {
                        buffer: &line_uniform_buffer,
                        offset: 0,
                        size: None,
                    },
                },
            );

        // Create point uniform bind group using generated types
        let point_uniform = generated_shaders::point::PointUniform {
            point_color: glam::Vec4::new(1.0, 1.0, 1.0, 1.0), // Default white color
            point_size: 1.0,
            _padding1: 0.0,
            _padding2: 0.0,
            _padding3: 0.0,
        };
        let point_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Point Uniform Buffer"),
            contents: unsafe {
                std::slice::from_raw_parts(
                    &point_uniform as *const _ as *const u8,
                    std::mem::size_of::<generated_shaders::point::PointUniform>(),
                )
            },
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let point_uniform_bind_group =
            generated_shaders::point::bind_groups::BindGroup1::from_bindings(
                &device,
                generated_shaders::point::bind_groups::BindGroupLayout1 {
                    point: wgpu::BufferBinding {
                        buffer: &point_uniform_buffer,
                        offset: 0,
                        size: None,
                    },
                },
            );

        // Initialize occlusion query system
        let occlusion_system = OcclusionSystem::new(&device, &queue).map_err(|e| {
            AstrariaError::RenderingError(format!("Failed to create occlusion system: {}", e))
        })?;

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
            default_lighting_bind_group,
            default_texture_bind_group,
            planet_lighting_bind_group,
            planet_texture_bind_group,
            sun_uniform_bind_group,
            sun_texture_bind_group,
            skybox_texture_bind_group,
            black_hole_uniform_bind_group,
            black_hole_texture_bind_group,
            lens_glow_uniform_bind_group,
            line_uniform_bind_group,
            point_uniform_bind_group,
            mvp_buffers,
            mvp_bind_groups,
            prepared_render_commands: Vec::new(),
            cube_mesh,
            skybox_mesh,
            sphere_model,
            quad_mesh,
            line_mesh,
            point_mesh,
            _depth_texture: depth_texture,
            _depth_view: depth_view,
            default_sampler,
            view_matrix_d64: DMat4::IDENTITY,
            projection_matrix_d64: DMat4::IDENTITY,
            view_projection_matrix_d64: DMat4::IDENTITY,
            occlusion_system,
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

    /// Begin occlusion testing for a star
    pub fn test_star_occlusion(
        &mut self,
        star_id: u32,
        world_position: DVec3,
    ) -> AstrariaResult<()> {
        log::debug!(
            "MainRenderer: Testing occlusion for star {} at {:?}",
            star_id,
            world_position
        );
        self.occlusion_system
            .test_star_occlusion(star_id, world_position)
            .map_err(|e| AstrariaError::RenderingError(format!("Occlusion test failed: {}", e)))
    }

    /// Get visibility factor for a star (0.0 = occluded, 1.0 = visible)
    pub fn get_star_visibility(&self, star_id: u32) -> f32 {
        let visibility = self.occlusion_system.get_star_visibility(star_id);
        log::debug!("MainRenderer: Star {} visibility: {}", star_id, visibility);
        visibility
    }

    /// Process occlusion query results from previous frames
    pub fn process_occlusion_results(&mut self) -> AstrariaResult<()> {
        self.occlusion_system
            .process_query_results(&self.device)
            .map_err(|e| {
                AstrariaError::RenderingError(format!("Occlusion processing failed: {}", e))
            })?;
        self.occlusion_system.cleanup_old_queries();
        Ok(())
    }

    /// Get the first MVP bind group for occlusion queries (returns None if no bind groups exist)
    fn get_first_mvp_bind_group(
        &self,
    ) -> Option<&generated_shaders::default::bind_groups::BindGroup0> {
        self.mvp_bind_groups
            .first()
            .map(|(bind_group, _)| bind_group)
    }

    /// Execute occlusion queries for pending stars
    pub fn execute_occlusion_queries_with_bind_group(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        color_view: &wgpu::TextureView,
        depth_view: &wgpu::TextureView,
        screen_width: f32,
        screen_height: f32,
    ) -> AstrariaResult<()> {
        // Check if we have MVP bind groups
        if let Some((mvp_bind_group, _)) = self.mvp_bind_groups.first() {
            // Get camera data for occlusion queries
            let camera_view = self.camera.view_matrix();
            let camera_projection = self.camera.projection_matrix();
            let camera_position = self.camera.position();

            self.occlusion_system
                .execute_occlusion_queries(
                    encoder,
                    camera_view,
                    camera_projection,
                    camera_position,
                    color_view,
                    depth_view,
                    mvp_bind_group,
                    &self.queue,
                    screen_width,
                    screen_height,
                )
                .map_err(|e| {
                    AstrariaError::RenderingError(format!(
                        "Occlusion query execution failed: {}",
                        e
                    ))
                })
        } else {
            log::warn!("No MVP bind groups available for occlusion queries");
            Ok(())
        }
    }

    /// Update camera with movement and GPU uniforms
    pub fn update_camera(&mut self, _delta_time: f32) {
        // Camera movement is now handled directly through process_movement calls
        // GPU uniforms are calculated on-demand via get_uniform()
    }

    /// Unified uniform computation method that handles all rendering cases
    /// Uses 64-bit precision for astronomical coordinates and converts to 32-bit for GPU
    pub fn compute_uniform(
        &self,
        object_position: DVec3,
        object_scale: DVec3,
        light_position: Option<DVec3>,
        is_skybox: bool,
    ) -> generated_shaders::default::StandardMVPUniform {
        // Use the unified atmospheric computation for all cases
        let (mvp_matrix, camera_relative_transform) = calculate_mvp_matrix_64bit_with_atmosphere(
            &self.camera,
            object_position,
            object_scale,
            is_skybox,
            light_position, // None for basic objects, Some(pos) for atmospheric
        );

        // Create the unified uniform using generated types
        let mut uniform = generated_shaders::default::StandardMVPUniform {
            mvp_matrix,
            camera_position: self.camera.position().as_vec3(),
            _padding1: 0.0,
            camera_direction: self.camera.direction(),
            _padding2: 0.0,
            log_depth_constant: self.log_depth_constant,
            far_plane_distance: self.max_view_distance,
            near_plane_distance: 1.0,
            fc_constant: 2.0 / (self.max_view_distance + 1.0).ln(),
            mv_matrix: camera_relative_transform,
        };

        // Special case overrides for skybox
        if is_skybox {
            uniform.mv_matrix = Mat4::IDENTITY;
        }

        uniform
    }

    /// Convenience method for basic object rendering (no atmospheric effects)
    pub fn compute_uniform_basic(
        &self,
        object_position: DVec3,
        object_scale: DVec3,
    ) -> generated_shaders::default::StandardMVPUniform {
        self.compute_uniform(object_position, object_scale, None, false)
    }

    /// Convenience method for atmospheric planet rendering
    pub fn compute_uniform_atmospheric(
        &self,
        planet_position: DVec3,
        planet_scale: DVec3,
        star_position: DVec3,
    ) -> generated_shaders::default::StandardMVPUniform {
        self.compute_uniform(planet_position, planet_scale, Some(star_position), false)
    }

    /// Convenience method for skybox rendering
    pub fn compute_uniform_skybox(&self) -> generated_shaders::default::StandardMVPUniform {
        self.compute_uniform(DVec3::ZERO, DVec3::ONE, None, true)
    }

    /// Reset frame data at the start of each frame
    pub fn begin_frame(&mut self) {
        self.prepared_render_commands.clear();
        // Clear old MVP bind groups (keep buffers for reuse)
        self.mvp_bind_groups.clear();
    }

    /// Create or reuse an MVP buffer and bind group for a uniform
    fn get_or_create_mvp_bind_group(
        &mut self,
        uniform: generated_shaders::default::StandardMVPUniform,
    ) -> usize {
        // Find an available buffer or create a new one
        let buffer_index = if self.mvp_buffers.len() <= self.mvp_bind_groups.len() {
            // Need a new buffer
            let buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("MVP Uniform Buffer {}", self.mvp_buffers.len())),
                    contents: unsafe {
                        std::slice::from_raw_parts(
                            &uniform as *const _ as *const u8,
                            std::mem::size_of::<generated_shaders::default::StandardMVPUniform>(),
                        )
                    },
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
            self.mvp_buffers.push(buffer);
            self.mvp_buffers.len() - 1
        } else {
            // Reuse existing buffer
            let buffer_index = self.mvp_bind_groups.len();
            self.queue
                .write_buffer(&self.mvp_buffers[buffer_index], 0, unsafe {
                    std::slice::from_raw_parts(
                        &uniform as *const _ as *const u8,
                        std::mem::size_of::<generated_shaders::default::StandardMVPUniform>(),
                    )
                });
            buffer_index
        };

        // Create bind group for this buffer
        let bind_group = generated_shaders::default::bind_groups::BindGroup0::from_bindings(
            &self.device,
            generated_shaders::default::bind_groups::BindGroupLayout0 {
                mvp: wgpu::BufferBinding {
                    buffer: &self.mvp_buffers[buffer_index],
                    offset: 0,
                    size: None,
                },
            },
        );

        let bind_group_index = self.mvp_bind_groups.len();
        self.mvp_bind_groups.push((bind_group, buffer_index));
        bind_group_index
    }

    /// Prepare a render command for later execution (creates MVP uniform and bind group)
    /// This should be called during the preparation phase for each object to render
    pub fn prepare_render_command(&mut self, command: RenderCommand, transform: Mat4) {
        // Compute the appropriate MVP uniform based on command type
        let mvp_uniform = match &command {
            RenderCommand::Skybox => self.compute_uniform_skybox(),
            RenderCommand::AtmosphericPlanet { .. } => {
                let (scale, _rotation, translation) = transform.to_scale_rotation_translation();
                let final_planet_position = translation.as_dvec3();

                // TODO: CRITICAL - ATMOSPHERIC RENDERING BROKEN!
                // The Java version expects actual star position to calculate light direction
                // for atmospheric scattering effects. We're currently passing DVec3::ZERO
                // which breaks the atmosphere rendering completely.
                //
                // SOLUTION NEEDED: We need to pass the actual star position, but in a
                // magnitude-reduced form to avoid f32 precision issues. Options:
                // 1. Pass star position relative to planet (star_pos - planet_pos)
                // 2. Use camera-relative coordinates for both positions
                // 3. Add star position to RenderCommand::AtmosphericPlanet
                // 4. Implement a scene graph to track star-planet relationships
                //
                // For now using origin as star position which is WRONG!
                let final_star_position = DVec3::ZERO;
                self.compute_uniform_atmospheric(
                    final_planet_position,
                    scale.as_dvec3(),
                    final_star_position,
                )
            }
            RenderCommand::Sun { .. } => {
                let (scale, _rotation, translation) = transform.to_scale_rotation_translation();
                self.compute_uniform_basic(translation.as_dvec3(), scale.as_dvec3())
            }
            _ => {
                // Default case for other render commands
                let (scale, _rotation, translation) = transform.to_scale_rotation_translation();
                self.compute_uniform_basic(translation.as_dvec3(), scale.as_dvec3())
            }
        };

        // Create or reuse MVP bind group
        let mvp_bind_group_index = self.get_or_create_mvp_bind_group(mvp_uniform);

        // Store the command with its MVP bind group index for later execution
        self.prepared_render_commands
            .push((command, transform, mvp_bind_group_index));
    }

    /// Execute all prepared render commands with their MVP bind groups
    /// This should be called within the render pass
    pub fn execute_prepared_commands<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        for (command, transform, mvp_bind_group_index) in &self.prepared_render_commands {
            self.execute_render_command_with_bind_group(
                render_pass,
                command,
                *transform,
                *mvp_bind_group_index,
            );
        }
    }

    /// Execute a single render command with its MVP bind group
    /// This is the core rendering logic using generated bind groups
    fn execute_render_command_with_bind_group<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        command: &RenderCommand,
        _transform: Mat4,
        mvp_bind_group_index: usize,
    ) {
        // Get the MVP bind group for this command
        let (mvp_bind_group, _buffer_index) = &self.mvp_bind_groups[mvp_bind_group_index];

        match command {
            RenderCommand::Default {
                mesh_type,
                light_position: _,
                light_color: _,
            } => {
                render_pass.set_pipeline(&self.default_shader.pipeline);
                mvp_bind_group.set(render_pass); // Use generated set method
                self.default_lighting_bind_group.set(render_pass); // Use generated set method
                self.default_texture_bind_group.set(render_pass); // Use generated set method

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

            RenderCommand::Planet {
                texture_path,
                planet_position,
                sun_position,
            } => {
                render_pass.set_pipeline(&self.default_shader.pipeline);
                mvp_bind_group.set(render_pass); // MVP bind group

                // Create dynamic lighting bind group with planet-to-sun direction
                let dynamic_lighting_bind_group =
                    self.create_planet_lighting_bind_group(*planet_position, *sun_position);
                match dynamic_lighting_bind_group {
                    Ok(lighting_bind_group) => {
                        lighting_bind_group.set(render_pass);
                        log::debug!(
                            "MainRenderer: Using dynamic lighting bind group for regular planet"
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "MainRenderer: Failed to create dynamic planet lighting: {}, falling back to hardcoded",
                            e
                        );
                        self.default_lighting_bind_group.set(render_pass); // Fallback
                    }
                }

                // Create dynamic texture bind group for the planet's specific texture
                let planet_texture = self
                    .asset_manager
                    .get_texture_handle(texture_path)
                    .or_else(|| {
                        self.asset_manager
                            .get_texture_handle(&format!("assets/{}", texture_path))
                    })
                    .or_else(|| {
                        self.asset_manager
                            .get_texture_handle(&format!("./{}", texture_path))
                    });

                match planet_texture {
                    Some(texture) => {
                        // Create dynamic texture bind group with the planet's texture
                        let dynamic_texture_bind_group =
                            generated_shaders::default::bind_groups::BindGroup2::from_bindings(
                                &self.device,
                                generated_shaders::default::bind_groups::BindGroupLayout2 {
                                    diffuse_texture: &texture.view,
                                    diffuse_sampler: &self.default_sampler,
                                },
                            );
                        dynamic_texture_bind_group.set(render_pass);
                        log::debug!(
                            "MainRenderer: Using dynamic texture bind group for planet: {}",
                            texture_path
                        );
                    }
                    None => {
                        // Fall back to default texture if planet texture not found
                        log::warn!(
                            "MainRenderer: Planet texture {} not found in cache, using default",
                            texture_path
                        );
                        self.default_texture_bind_group.set(render_pass);
                    }
                }

                // Render using sphere model
                render_pass.set_vertex_buffer(0, self.sphere_model.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.sphere_model.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.sphere_model.num_indices, 0, 0..1);
            }

            RenderCommand::AtmosphericPlanet {
                texture_path,
                ambient_texture_path,
                use_ambient_texture,
                atmosphere_color,
                overglow,
                planet_position,
                sun_position,
            } => {
                render_pass.set_pipeline(&self.planet_atmo_shader.pipeline);

                // Create appropriate MVP bind group for planet_atmo shader
                let planet_mvp_bind_group =
                    generated_shaders::planet_atmo::bind_groups::BindGroup0::from_bindings(
                        &self.device,
                        generated_shaders::planet_atmo::bind_groups::BindGroupLayout0 {
                            mvp: wgpu::BufferBinding {
                                buffer: &self.mvp_buffers
                                    [self.mvp_bind_groups[mvp_bind_group_index].1],
                                offset: 0,
                                size: None,
                            },
                        },
                    );
                planet_mvp_bind_group.set(render_pass);

                // Create dynamic lighting bind group with the correct atmospheric color and planet-to-sun direction
                let dynamic_lighting_bind_group = self.create_atmospheric_lighting_bind_group(
                    *atmosphere_color,
                    *overglow,
                    *use_ambient_texture,
                    *planet_position,
                    *sun_position,
                );

                match dynamic_lighting_bind_group {
                    Ok(lighting_bind_group) => {
                        lighting_bind_group.set(render_pass);
                        log::debug!(
                            "MainRenderer: Using dynamic lighting bind group with atmo_color: {:?}",
                            atmosphere_color
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "MainRenderer: Failed to create dynamic lighting bind group: {}, falling back to hardcoded",
                            e
                        );
                        self.planet_lighting_bind_group.set(render_pass);
                    }
                }

                // Create dynamic texture bind group based on the texture paths
                let dynamic_texture_bind_group = self.create_atmospheric_texture_bind_group(
                    texture_path,
                    ambient_texture_path.as_deref(),
                    *use_ambient_texture,
                );

                match dynamic_texture_bind_group {
                    Ok(texture_bind_group) => {
                        texture_bind_group.set(render_pass);
                        log::debug!(
                            "MainRenderer: Using dynamic texture bind group for atmospheric planet"
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "MainRenderer: Failed to create dynamic texture bind group: {}, falling back to hardcoded",
                            e
                        );
                        self.planet_texture_bind_group.set(render_pass);
                    }
                }

                render_pass.set_vertex_buffer(0, self.sphere_model.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.sphere_model.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.sphere_model.num_indices, 0, 0..1);
            }

            RenderCommand::Sun { temperature } => {
                // Update sun uniforms if needed
                let star_position = glam::Vec3::ZERO; // Placeholder, position is handled by MVP matrix
                let camera_position = self.camera.position().as_vec3();
                render_pass.set_pipeline(&self.sun_shader.pipeline);
                // Create appropriate MVP bind group for sun shader
                let sun_mvp_bind_group =
                    generated_shaders::sun_shader::bind_groups::BindGroup0::from_bindings(
                        &self.device,
                        generated_shaders::sun_shader::bind_groups::BindGroupLayout0 {
                            mvp: wgpu::BufferBinding {
                                buffer: &self.mvp_buffers
                                    [self.mvp_bind_groups[mvp_bind_group_index].1],
                                offset: 0,
                                size: None,
                            },
                        },
                    );
                sun_mvp_bind_group.set(render_pass);
                self.sun_uniform_bind_group.set(render_pass);
                self.sun_texture_bind_group.set(render_pass);
                render_pass.set_vertex_buffer(0, self.sphere_model.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.sphere_model.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.sphere_model.num_indices, 0, 0..1);
            }

            RenderCommand::Skybox => {
                render_pass.set_pipeline(&self.skybox_shader.pipeline);
                // Create appropriate MVP bind group for skybox shader
                let skybox_mvp_bind_group =
                    generated_shaders::skybox::bind_groups::BindGroup0::from_bindings(
                        &self.device,
                        generated_shaders::skybox::bind_groups::BindGroupLayout0 {
                            mvp: wgpu::BufferBinding {
                                buffer: &self.mvp_buffers
                                    [self.mvp_bind_groups[mvp_bind_group_index].1],
                                offset: 0,
                                size: None,
                            },
                        },
                    );
                skybox_mvp_bind_group.set(render_pass);
                self.skybox_texture_bind_group.set(render_pass);
                render_pass.set_vertex_buffer(0, self.skybox_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.skybox_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.skybox_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::Billboard => {
                render_pass.set_pipeline(&self.billboard_shader.pipeline);
                // Create appropriate MVP bind group for billboard shader
                let billboard_mvp_bind_group =
                    generated_shaders::billboard::bind_groups::BindGroup0::from_bindings(
                        &self.device,
                        generated_shaders::billboard::bind_groups::BindGroupLayout0 {
                            mvp: wgpu::BufferBinding {
                                buffer: &self.mvp_buffers
                                    [self.mvp_bind_groups[mvp_bind_group_index].1],
                                offset: 0,
                                size: None,
                            },
                        },
                    );
                billboard_mvp_bind_group.set(render_pass);
                // Billboard shader only uses MVP, no uniform bind group
                render_pass.set_vertex_buffer(0, self.quad_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.quad_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.quad_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::LensGlow {
                star_id,
                star_position,
                star_temperature,
                star_radius,
                camera_distance,
            } => {
                render_pass.set_pipeline(&self.lens_glow_shader.pipeline);

                // Calculate physics-based glow size for uniform
                // Java uses star.getRadius() * 200 as diameter input
                let diameter_km = ((*star_radius as f64) / 1000.0) * 200.0; // Convert radius to km, then multiply by 200
                let physics_size = crate::renderer::calculate_lens_glow_size(
                    diameter_km,
                    *star_temperature as f64,
                    *camera_distance,
                );

                // Create dynamic uniform bind group with physics size, visibility, and star data
                let camera_direction = self.camera.get_forward_direction();
                let dynamic_uniform_bind_group = self.create_lens_glow_uniform_bind_group(
                    physics_size as f32,
                    *star_id,
                    *star_position,
                    camera_direction,
                    *star_temperature,
                );

                match dynamic_uniform_bind_group {
                    Ok(uniform_bind_group) => {
                        // Create appropriate MVP bind group for lens glow shader
                        let lens_glow_mvp_bind_group =
                            generated_shaders::lens_glow::bind_groups::BindGroup0::from_bindings(
                                &self.device,
                                generated_shaders::lens_glow::bind_groups::BindGroupLayout0 {
                                    mvp: wgpu::BufferBinding {
                                        buffer: &self.mvp_buffers
                                            [self.mvp_bind_groups[mvp_bind_group_index].1],
                                        offset: 0,
                                        size: None,
                                    },
                                },
                            );
                        lens_glow_mvp_bind_group.set(render_pass);
                        uniform_bind_group.set(render_pass);
                        render_pass.set_vertex_buffer(0, self.quad_mesh.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(
                            self.quad_mesh.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint32,
                        );
                        render_pass.draw_indexed(0..self.quad_mesh.num_indices, 0, 0..1);
                    }
                    Err(e) => {
                        log::warn!("Failed to create lens glow uniform bind group: {}", e);
                    }
                }
            }

            RenderCommand::BlackHole => {
                render_pass.set_pipeline(&self.black_hole_shader.pipeline);
                // Create appropriate MVP bind group for black hole shader
                let black_hole_mvp_bind_group =
                    generated_shaders::black_hole::bind_groups::BindGroup0::from_bindings(
                        &self.device,
                        generated_shaders::black_hole::bind_groups::BindGroupLayout0 {
                            mvp: wgpu::BufferBinding {
                                buffer: &self.mvp_buffers
                                    [self.mvp_bind_groups[mvp_bind_group_index].1],
                                offset: 0,
                                size: None,
                            },
                        },
                    );
                black_hole_mvp_bind_group.set(render_pass);
                self.black_hole_uniform_bind_group.set(render_pass);
                self.black_hole_texture_bind_group.set(render_pass);
                render_pass.set_vertex_buffer(0, self.sphere_model.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.sphere_model.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.sphere_model.num_indices, 0, 0..1);
            }

            RenderCommand::Line { color: _ } => {
                render_pass.set_pipeline(&self.line_shader.pipeline);
                // Create appropriate MVP bind group for line shader
                let line_mvp_bind_group =
                    generated_shaders::line::bind_groups::BindGroup0::from_bindings(
                        &self.device,
                        generated_shaders::line::bind_groups::BindGroupLayout0 {
                            mvp: wgpu::BufferBinding {
                                buffer: &self.mvp_buffers
                                    [self.mvp_bind_groups[mvp_bind_group_index].1],
                                offset: 0,
                                size: None,
                            },
                        },
                    );
                line_mvp_bind_group.set(render_pass);
                self.line_uniform_bind_group.set(render_pass);
                render_pass.set_vertex_buffer(0, self.line_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.line_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.line_mesh.num_indices, 0, 0..1);
            }

            RenderCommand::Point => {
                render_pass.set_pipeline(&self.point_shader.pipeline);
                // Create appropriate MVP bind group for point shader
                let point_mvp_bind_group =
                    generated_shaders::point::bind_groups::BindGroup0::from_bindings(
                        &self.device,
                        generated_shaders::point::bind_groups::BindGroupLayout0 {
                            mvp: wgpu::BufferBinding {
                                buffer: &self.mvp_buffers
                                    [self.mvp_bind_groups[mvp_bind_group_index].1],
                                offset: 0,
                                size: None,
                            },
                        },
                    );
                point_mvp_bind_group.set(render_pass);
                self.point_uniform_bind_group.set(render_pass);
                render_pass.set_vertex_buffer(0, self.point_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.point_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.point_mesh.num_indices, 0, 0..1);
            }
        }
    }

    /// Legacy helper method for single command execution using new per-object approach
    /// Use this only for simple cases - prefer the full two-phase approach for multiple objects
    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        command: &RenderCommand,
        transform: Mat4,
    ) {
        self.begin_frame();
        self.prepare_render_command(command.clone(), transform);
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
