use bytemuck::{Pod, Zeroable};
/// Lens glow shader for stellar lens flare effects
/// Renders lens flares with temperature-based colors and occlusion testing
use wgpu::{Device, Queue, RenderPipeline};

use crate::{graphics::Vertex, renderer::core::*, AstrariaResult};

// CameraUniform and TransformUniform are now imported from core.rs to eliminate duplication

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LensGlowUniform {
    pub screen_dimensions: [f32; 2], // Screen width and height  (8 bytes)
    pub glow_size: [f32; 2], // Width and height of the glow effect (8 bytes) = 16 bytes so far
    pub star_position: [f32; 3], // Star position in world coordinates (12 bytes)
    pub _padding1: f32,      // (4 bytes) = 16 bytes, total 32 bytes
    pub camera_direction: [f32; 3], // Camera forward direction (12 bytes)
    pub _padding2: f32,      // (4 bytes) = 16 bytes, total 48 bytes
    pub temperature: f32,    // Star temperature for spectrum mapping (4 bytes)
    pub _padding3: [f32; 7], // (28 bytes) = 32 bytes, total 80 bytes
}

pub struct LensGlowShader {
    pub pipeline: RenderPipeline,
    pub transform_buffer: wgpu::Buffer,
    pub lens_glow_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl LensGlowShader {
    pub fn new(device: &Device, queue: &Queue) -> AstrariaResult<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Lens Glow Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/lens_glow.wgsl").into()),
        });

        // Camera bind group layout (group 0) - shared with other shaders
        let camera_bind_group_layout =
            crate::renderer::core::create_camera_bind_group_layout(device);

        // Lens glow specific bind group layout (group 1) - transform and lens_glow uniforms
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Lens Glow Uniform Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
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

        // Texture bind group layout (group 2)
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Lens Glow Texture Bind Group Layout"),
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Lens Glow Pipeline Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &uniform_bind_group_layout,
                &texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Lens Glow Pipeline"),
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
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }), // No depth buffer for test mode
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Create transform uniform buffer
        let transform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Lens Glow Transform Buffer"),
            size: std::mem::size_of::<TransformUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create lens glow uniform buffer
        let lens_glow_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Lens Glow Uniform Buffer"),
            size: std::mem::size_of::<LensGlowUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Initialize transform uniform (identity matrix)
        let transform_uniform = TransformUniform {
            model_matrix: glam::Mat4::IDENTITY.to_cols_array_2d(),
            model_view_matrix: glam::Mat4::IDENTITY.to_cols_array_2d(),
            normal_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
            _padding: [0.0; 4],
        };
        queue.write_buffer(
            &transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );

        // Initialize lens glow uniform
        let lens_glow_uniform = LensGlowUniform {
            screen_dimensions: [1920.0, 1080.0],
            glow_size: [64.0, 64.0],
            star_position: [0.0, 0.0, 0.0],
            _padding1: 0.0,
            camera_direction: [0.0, 0.0, 1.0],
            _padding2: 0.0,
            temperature: 5778.0, // Sun temperature
            _padding3: [0.0; 7],
        };
        queue.write_buffer(
            &lens_glow_buffer,
            0,
            bytemuck::cast_slice(&[lens_glow_uniform]),
        );

        // Create lens glow specific bind group (group 1)
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Lens Glow Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: lens_glow_buffer.as_entire_binding(),
                },
            ],
        });

        Ok(Self {
            pipeline,
            transform_buffer,
            lens_glow_buffer,
            uniform_bind_group,
            texture_bind_group_layout,
        })
    }

    pub fn update_transform(&self, queue: &Queue, transform: &TransformUniform) {
        queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[*transform]),
        );
    }

    pub fn update_lens_glow(&self, queue: &Queue, lens_glow: &LensGlowUniform) {
        queue.write_buffer(
            &self.lens_glow_buffer,
            0,
            bytemuck::cast_slice(&[*lens_glow]),
        );
    }
}
