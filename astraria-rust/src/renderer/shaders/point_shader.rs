use bytemuck::{Pod, Zeroable};
/// Point shader for distant object rendering
/// Renders point primitives with logarithmic depth buffer support
use wgpu::{Device, Queue, RenderPipeline};

use crate::{graphics::Vertex, AstrariaResult};

// CameraUniform and TransformUniform are now imported from core.rs to eliminate duplication

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PointUniform {
    pub color: [f32; 4], // Point color with alpha
}

pub struct PointShader {
    pub pipeline: RenderPipeline,
}

impl PointShader {
    pub fn new(device: &Device, _queue: &Queue) -> AstrariaResult<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Point Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/point.wgsl").into()),
        });

        // Use shared bind group layouts from MainRenderer
        let camera_bind_group_layout =
            crate::renderer::core::create_camera_bind_group_layout(device);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Point Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Point Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: (std::mem::size_of::<[f32; 3]>()
                                + std::mem::size_of::<[f32; 2]>())
                                as wgpu::BufferAddress,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }],
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
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None, // No depth buffer for test mode
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(Self { pipeline })
    }
}
