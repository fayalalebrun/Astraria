use crate::{AstrariaResult, generated_shaders};
/// Skybox shader for cubemap background rendering
/// Refactored to use standardized MVP matrix approach with 64-bit precision calculations
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, RenderPass, RenderPipeline};

pub struct SkyboxShader {
    pub pipeline: RenderPipeline,
    pub texture_bind_group_layout: BindGroupLayout,
    pub mvp_bind_group_layout: BindGroupLayout,
}

impl SkyboxShader {
    pub fn new(device: &Device) -> AstrariaResult<Self> {
        // Use generated shader module
        let shader = generated_shaders::skybox::create_shader_module(device);

        // Use generated bind group layouts
        let mvp_bind_group_layout =
            generated_shaders::skybox::bind_groups::BindGroup0::get_bind_group_layout(device);
        let texture_bind_group_layout =
            generated_shaders::skybox::bind_groups::BindGroup1::get_bind_group_layout(device);

        // Use generated pipeline layout
        let pipeline_layout = generated_shaders::skybox::create_pipeline_layout(device);

        // Use generated vertex entry
        let vertex_entry = generated_shaders::skybox::vs_main_entry(wgpu::VertexStepMode::Vertex);

        // Use generated fragment entry
        let fragment_entry =
            generated_shaders::skybox::fs_main_entry([Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })]);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Skybox Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: generated_shaders::skybox::vertex_state(&shader, &vertex_entry),
            fragment: Some(generated_shaders::skybox::fragment_state(
                &shader,
                &fragment_entry,
            )),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // No culling needed for inside-out skybox cube
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: false, // Don't write depth for skybox
                depth_compare: wgpu::CompareFunction::LessEqual, // Standard depth test
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

        Ok(Self {
            pipeline,
            texture_bind_group_layout,
            mvp_bind_group_layout,
        })
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        vertex_buffer: &'a Buffer,
        index_buffer: &'a Buffer,
        index_count: u32,
        mvp_bind_group: &'a BindGroup,
        texture_bind_group: &'a BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, mvp_bind_group, &[]);
        render_pass.set_bind_group(1, texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..index_count, 0, 0..1);
    }
}
