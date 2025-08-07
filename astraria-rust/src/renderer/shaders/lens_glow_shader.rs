/// Lens glow shader for stellar lens flare effects
/// Renders lens flares with temperature-based colors and occlusion testing
use wgpu::{Device, Queue, RenderPipeline};

use crate::{AstrariaResult, generated_shaders};

pub struct LensGlowShader {
    pub pipeline: RenderPipeline,
}

impl LensGlowShader {
    pub fn new(device: &Device, _queue: &Queue) -> AstrariaResult<Self> {
        // Use generated shader module
        let shader = generated_shaders::lens_glow::create_shader_module(device);

        // Use generated pipeline layout
        let pipeline_layout = generated_shaders::lens_glow::create_pipeline_layout(device);

        // Use generated vertex and fragment entries
        let vertex_entry =
            generated_shaders::lens_glow::vs_main_entry(wgpu::VertexStepMode::Vertex);
        let fragment_entry =
            generated_shaders::lens_glow::fs_main_entry([Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })]);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Lens Glow Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: generated_shaders::lens_glow::vertex_state(&shader, &vertex_entry),
            fragment: Some(generated_shaders::lens_glow::fragment_state(
                &shader,
                &fragment_entry,
            )),
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
            }),
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
