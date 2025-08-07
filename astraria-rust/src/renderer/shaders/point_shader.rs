/// Point shader for distant object rendering
/// Renders point primitives with logarithmic depth buffer support
use wgpu::{Device, Queue, RenderPipeline};

use crate::{AstrariaResult, generated_shaders};

pub struct PointShader {
    pub pipeline: RenderPipeline,
}

impl PointShader {
    pub fn new(device: &Device, _queue: &Queue) -> AstrariaResult<Self> {
        // Use generated shader module
        let shader = generated_shaders::point::create_shader_module(device);

        // Use generated bind group layouts
        let _mvp_bind_group_layout =
            generated_shaders::point::bind_groups::BindGroup0::get_bind_group_layout(device);
        let _point_bind_group_layout =
            generated_shaders::point::bind_groups::BindGroup1::get_bind_group_layout(device);

        // Use generated pipeline layout
        let pipeline_layout = generated_shaders::point::create_pipeline_layout(device);

        // Use generated vertex and fragment entries
        let vertex_entry = generated_shaders::point::vs_main_entry(wgpu::VertexStepMode::Vertex);
        let fragment_entry =
            generated_shaders::point::fs_main_entry([Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })]);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Point Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: generated_shaders::point::vertex_state(&shader, &vertex_entry),
            fragment: Some(generated_shaders::point::fragment_state(
                &shader,
                &fragment_entry,
            )),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
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
            cache: None,
            multiview: None,
        });

        Ok(Self { pipeline })
    }
}
