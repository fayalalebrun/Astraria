/// Billboard shader for screen-aligned sprite rendering
/// Now uses shared uniform buffers from MainRenderer
use bytemuck::{Pod, Zeroable};
use wgpu::{Device, Queue, RenderPipeline};

use crate::{graphics::Vertex, AstrariaResult};

// CameraUniform and TransformUniform are now imported from core.rs to eliminate duplication

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BillboardUniform {
    pub billboard_width: f32,       // Width of billboard in pixels
    pub billboard_height: f32,      // Height of billboard in pixels
    pub screen_width: f32,          // Screen width in pixels
    pub screen_height: f32,         // Screen height in pixels
    pub billboard_origin: [f32; 3], // 3D world position of billboard center
    pub _padding: f32,
}

pub struct BillboardShader {
    pub pipeline: RenderPipeline,
    pub billboard_bind_group_layout: wgpu::BindGroupLayout,
}

impl BillboardShader {
    pub fn new(device: &Device, _queue: &Queue) -> AstrariaResult<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Billboard Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/billboard.wgsl").into()),
        });

        // Use shared bind group layouts from MainRenderer
        let camera_bind_group_layout =
            crate::renderer::uniforms::buffer_helpers::create_mvp_bind_group_layout_dynamic(
                device,
                Some("Billboard MVP Bind Group Layout"),
            );

        // Create billboard-specific bind group layout
        let billboard_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Billboard Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Billboard Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &billboard_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Billboard Pipeline"),
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
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
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

        Ok(Self {
            pipeline,
            billboard_bind_group_layout,
        })
    }
}
