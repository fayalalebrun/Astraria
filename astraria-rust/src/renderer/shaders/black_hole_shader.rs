use crate::renderer::shader_utils::load_preprocessed_wgsl;
use bytemuck::{Pod, Zeroable};
use std::path::Path;
/// Black hole shader for gravitational lensing simulation
/// Simulates gravitational lensing effects using refraction shaders and skybox sampling
use wgpu::{Device, Queue, RenderPipeline};

use crate::{AstrariaResult, graphics::Vertex};

// CameraUniform and TransformUniform are now imported from core.rs to eliminate duplication

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BlackHoleUniform {
    pub hole_position: [f32; 3], // Black hole position in world coordinates
    pub _padding: f32,
}

pub struct BlackHoleShader {
    pub pipeline: RenderPipeline,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl BlackHoleShader {
    pub fn new(device: &Device, _queue: &Queue) -> AstrariaResult<Self> {
        let shader_path = Path::new("src/shaders/black_hole.wgsl");
        let shader_source = load_preprocessed_wgsl(shader_path)
            .map_err(|e| crate::AstrariaError::Graphics(format!("Failed to load shader: {}", e)))?;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Black Hole Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Camera bind group layout (group 0) - shared with other shaders
        let camera_bind_group_layout =
            crate::renderer::uniforms::buffer_helpers::create_mvp_bind_group_layout_dynamic(
                device,
                Some("BlackHole MVP Bind Group Layout"),
            );

        // Black hole specific bind group layout (group 1) - black hole uniform only
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Black Hole Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Texture bind group layout (group 2) - for skybox cubemap
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Black Hole Texture Bind Group Layout"),
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Black Hole Pipeline Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &uniform_bind_group_layout,
                &texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Black Hole Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
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
            cache: None,
            multiview: None,
        });

        Ok(Self {
            pipeline,
            uniform_bind_group_layout,
            texture_bind_group_layout,
        })
    }
}
