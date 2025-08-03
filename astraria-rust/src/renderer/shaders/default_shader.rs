use glam::Mat4;
/// Default shader for planet and object rendering with PBR lighting
/// Equivalent to the Java DefaultShader class
use wgpu::{Buffer, Device, Queue, RenderPass, RenderPipeline};

use crate::{assets::ModelAsset, graphics::Vertex, AstrariaResult};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DefaultUniforms {
    pub view_projection: [[f32; 4]; 4],
}

pub struct DefaultShader {
    pipeline: RenderPipeline,
    uniform_buffer: Buffer,
    bind_group: wgpu::BindGroup,
}

impl DefaultShader {
    pub fn new(device: &Device) -> AstrariaResult<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Default Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../shaders/default_simple.wgsl").into(),
            ),
        });

        // DefaultShader needs its own bind group layout for view-projection matrix
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Default Shader Bind Group Layout"),
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

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Default Uniform Buffer"),
            size: std::mem::size_of::<DefaultUniforms>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Default Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Bind group will be managed globally by MainRenderer

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Default Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Default Render Pipeline"),
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
                    blend: Some(wgpu::BlendState::REPLACE),
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
            uniform_buffer,
            bind_group,
        })
    }

    pub fn update_uniforms(
        &self,
        queue: &Queue,
        view_matrix: Mat4,
        projection_matrix: Mat4,
        model_matrix: Mat4,
    ) {
        let view_projection = projection_matrix * view_matrix * model_matrix;
        let uniforms = DefaultUniforms {
            view_projection: view_projection.to_cols_array_2d(),
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    pub fn render_model<'a>(&'a self, render_pass: &mut RenderPass<'a>, model: &'a ModelAsset) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
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
        render_pass.set_bind_group(0, &self.bind_group, &[]);
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
