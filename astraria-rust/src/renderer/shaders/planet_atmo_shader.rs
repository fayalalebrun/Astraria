use glam::{Mat3, Mat4, Vec3, Vec4};
/// Planet atmosphere shader using the complete WGSL implementation
/// Matches the original Java atmospheric scattering with full feature set
use wgpu::{BindGroup, Buffer, Device, Queue, RenderPass, RenderPipeline};

use crate::{assets::ModelAsset, graphics::Vertex, renderer::core::*, AstrariaResult};

// CameraUniform and TransformUniform are now imported from core.rs to eliminate duplication

// Point light structure
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLight {
    pub position: [f32; 3],
    pub _padding1: f32,
    pub ambient: [f32; 3],
    pub _padding2: f32,
    pub diffuse: [f32; 3],
    pub _padding3: f32,
    pub specular: [f32; 3],
    pub _padding4: f32,
}

// Lighting uniform - must match WGSL vec3<i32> alignment (16 bytes)
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightingUniform {
    pub lights: [PointLight; 8], // 8 * 64 = 512 bytes
    pub num_lights: i32,         // 4 bytes
    pub _padding: [i32; 7],      // 28 bytes to reach 544 total (512 + 4 + 28 = 544)
}

// Atmosphere specific uniforms
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AtmosphereUniform {
    pub star_position: [f32; 3],
    pub _padding1: f32,
    pub planet_position: [f32; 3],
    pub _padding2: f32,
    pub atmosphere_color_mod: [f32; 4],
    pub overglow: f32,
    pub use_ambient_texture: i32,
    pub _padding3: [f32; 2],
}

pub struct PlanetAtmoShader {
    pub pipeline: RenderPipeline,
    // Bind group layouts
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    // Buffers
    camera_buffer: Buffer,
    transform_buffer: Buffer,
    lighting_buffer: Buffer,
    atmosphere_buffer: Buffer,
    // Bind groups
    pub camera_bind_group: BindGroup,
    pub transform_bind_group: BindGroup,
    pub lighting_bind_group: BindGroup,
    texture_bind_group: Option<BindGroup>,
}

impl PlanetAtmoShader {
    pub fn new(
        device: &Device,
        _queue: &Queue,
        day_texture: &wgpu::Texture,
        night_texture: &wgpu::Texture,
        atmosphere_texture: &wgpu::Texture,
    ) -> AstrariaResult<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Planet Atmosphere Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/planet_atmo.wgsl").into()),
        });

        // Camera bind group layout (group 0)
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
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

        // Transform bind group layout (group 1)
        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Transform Bind Group Layout"),
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

        // Lighting bind group layout (group 2)
        let lighting_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Lighting Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        // Texture bind group layout (group 3)
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
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
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        // Create buffers
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let transform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Transform Uniform Buffer"),
            size: std::mem::size_of::<TransformUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let lighting_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Lighting Uniform Buffer"),
            size: std::mem::size_of::<LightingUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let atmosphere_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Atmosphere Uniform Buffer"),
            size: std::mem::size_of::<AtmosphereUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind groups
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
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

        let lighting_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Lighting Bind Group"),
            layout: &lighting_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: lighting_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: atmosphere_buffer.as_entire_binding(),
                },
            ],
        });

        // Use provided Earth textures
        let ambient_texture = night_texture; // Night texture is ambient lighting
        let diffuse_texture = day_texture; // Day texture is diffuse lighting
        let atmosphere_gradient_texture = atmosphere_texture;

        // Textures are already loaded with real Earth data, no initialization needed

        // Create sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // Create texture bind group
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &ambient_texture.create_view(&Default::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &diffuse_texture.create_view(&Default::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(
                        &atmosphere_gradient_texture.create_view(&Default::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Planet Atmosphere Pipeline Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &transform_bind_group_layout,
                &lighting_bind_group_layout,
                &texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Planet Atmosphere Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(Self {
            pipeline,
            texture_bind_group_layout,
            camera_buffer,
            transform_buffer,
            lighting_buffer,
            atmosphere_buffer,
            camera_bind_group,
            transform_bind_group,
            lighting_bind_group,
            texture_bind_group: Some(texture_bind_group),
        })
    }

    pub fn update_uniforms(
        &self,
        queue: &Queue,
        view_matrix: Mat4,
        projection_matrix: Mat4,
        model_matrix: Mat4,
        light_position: Vec3,
        _light_color: Vec3,
        star_position: Vec3,
        planet_position: Vec3,
        _atmosphere_color_mod: Vec4,
        _overglow: f32,
        use_ambient_texture: bool,
    ) {
        // Camera uniforms
        let view_projection = projection_matrix * view_matrix;
        let model_view = view_matrix * model_matrix;
        let camera_uniforms = CameraUniform {
            view_matrix: view_matrix.to_cols_array_2d(),
            projection_matrix: projection_matrix.to_cols_array_2d(),
            view_projection_matrix: view_projection.to_cols_array_2d(),
            camera_position: [0.0, 0.0, 3.0], // Fixed for testing
            _padding1: 0.0,
            camera_direction: [0.0, 0.0, -1.0],
            _padding2: 0.0,
            log_depth_constant: 1.0, // From Java: LOGDEPTHCONSTANT = 1f
            far_plane_distance: 100000000000.0, // From Java: MAXVIEWDISTANCE = 100000000000f
            near_plane_distance: 1.0, // From Java: near plane = 1f
            fc_constant: 2.0 / (100000000000.0f32 + 1.0).ln(),
        };

        // Transform uniforms
        let normal_matrix = Mat3::from_mat4(model_view.inverse().transpose());
        // Convert mat3 to 3x vec4 for proper WGSL alignment
        let normal_cols = normal_matrix.to_cols_array_2d();
        let normal_matrix_aligned = [
            [normal_cols[0][0], normal_cols[0][1], normal_cols[0][2], 0.0],
            [normal_cols[1][0], normal_cols[1][1], normal_cols[1][2], 0.0],
            [normal_cols[2][0], normal_cols[2][1], normal_cols[2][2], 0.0],
        ];

        let transform_uniforms = TransformUniform {
            model_matrix: model_matrix.to_cols_array_2d(),
            model_view_matrix: model_view.to_cols_array_2d(),
            normal_matrix: normal_matrix_aligned,
            _padding: [0.0; 4],
        };

        // Lighting uniforms - using proper star lighting values from Java
        let point_light = PointLight {
            position: light_position.to_array(),
            _padding1: 0.0,
            ambient: [0.0, 0.0, 0.0], // Stars have no ambient light
            _padding2: 0.0,
            diffuse: [0.8, 0.8, 0.8], // Strong white diffuse light from star
            _padding3: 0.0,
            specular: [1.0, 1.0, 1.0], // Full white specular
            _padding4: 0.0,
        };

        let mut lights = [PointLight {
            position: [0.0; 3],
            _padding1: 0.0,
            ambient: [0.0; 3],
            _padding2: 0.0,
            diffuse: [0.0; 3],
            _padding3: 0.0,
            specular: [0.0; 3],
            _padding4: 0.0,
        }; 8];
        lights[0] = point_light;

        let lighting_uniforms = LightingUniform {
            lights,
            num_lights: 1,
            _padding: [0; 7],
        };

        // Atmosphere uniforms - using Venus-like atmosphere parameters
        let atmosphere_uniforms = AtmosphereUniform {
            star_position: star_position.to_array(),
            _padding1: 0.0,
            planet_position: planet_position.to_array(),
            _padding2: 0.0,
            atmosphere_color_mod: [0.984, 0.843, 0.616, 1.0], // Venus atmosphere color (yellow/orange)
            overglow: 0.1,                                    // Standard terminator transition
            use_ambient_texture: if use_ambient_texture { 1 } else { 0 },
            _padding3: [0.0; 2],
        };

        // Write buffers
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniforms]),
        );
        queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniforms]),
        );
        queue.write_buffer(
            &self.lighting_buffer,
            0,
            bytemuck::cast_slice(&[lighting_uniforms]),
        );
        queue.write_buffer(
            &self.atmosphere_buffer,
            0,
            bytemuck::cast_slice(&[atmosphere_uniforms]),
        );
    }

    pub fn render_planet<'a>(&'a self, render_pass: &mut RenderPass<'a>, model: &'a ModelAsset) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(1, &self.transform_bind_group, &[]);
        render_pass.set_bind_group(2, &self.lighting_bind_group, &[]);
        if let Some(ref texture_bind_group) = self.texture_bind_group {
            render_pass.set_bind_group(3, texture_bind_group, &[]);
        }
        render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
        render_pass.set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..model.num_indices, 0, 0..1);
    }

    pub fn render_geometry<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        vertex_buffer: &'a Buffer,
        index_buffer: &'a Buffer,
        num_indices: u32,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(1, &self.transform_bind_group, &[]);
        render_pass.set_bind_group(2, &self.lighting_bind_group, &[]);
        if let Some(ref texture_bind_group) = self.texture_bind_group {
            render_pass.set_bind_group(3, texture_bind_group, &[]);
        }
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);
    }

    /// Render a mesh using this shader
    pub fn render_mesh<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        mesh: &'a crate::graphics::Mesh,
    ) {
        self.render_geometry(
            render_pass,
            &mesh.vertex_buffer,
            &mesh.index_buffer,
            mesh.num_indices,
        );
    }
}
