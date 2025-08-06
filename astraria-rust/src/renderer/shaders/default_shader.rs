/// Default shader for planet and object rendering with PBR lighting
/// Equivalent to the Java DefaultShader class
use std::path::Path;
use wgpu::{Buffer, Device, Queue, RenderPass, RenderPipeline};

use crate::{
    AstrariaResult,
    assets::ModelAsset,
    graphics::Vertex,
    renderer::{shader_utils::load_preprocessed_wgsl, uniforms::StandardMVPUniform},
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DirectionalLight {
    pub direction: [f32; 3], // Normalized direction from object to light (WORLD SPACE)
    pub _padding1: f32,
    pub ambient: [f32; 3],
    pub _padding2: f32,
    pub diffuse: [f32; 3],
    pub _padding3: f32,
    pub specular: [f32; 3],
    pub _padding4: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightingUniforms {
    pub lights: [DirectionalLight; 8],
    pub num_lights: i32,
    pub _padding: [f32; 3],
}

pub struct DefaultShader {
    pub pipeline: RenderPipeline,
    uniform_buffer: Buffer,
    mvp_bind_group: wgpu::BindGroup,
    pub lighting_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl DefaultShader {
    pub fn new(device: &Device) -> AstrariaResult<Self> {
        let shader_path = Path::new("src/shaders/default.wgsl");
        let shader_source = load_preprocessed_wgsl(shader_path)
            .map_err(|e| crate::AstrariaError::Graphics(format!("Failed to load shader: {}", e)))?;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Default Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // MVP bind group layout (group 0) - use dynamic layout for compatibility with MainRenderer
        let mvp_bind_group_layout =
            crate::renderer::uniforms::buffer_helpers::create_mvp_bind_group_layout_dynamic(
                device,
                Some("Default MVP Bind Group Layout"),
            );

        // Lighting bind group layout (group 1)
        let lighting_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Lighting Bind Group Layout"),
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

        // Texture bind group layout (group 2)
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
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Default Uniform Buffer"),
            size: 256, // Match dynamic binding size requirement
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mvp_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("MVP Bind Group"),
            layout: &mvp_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Bind group will be managed globally by MainRenderer

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Default Pipeline Layout"),
            bind_group_layouts: &[
                &mvp_bind_group_layout,
                &lighting_bind_group_layout,
                &texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Default Render Pipeline"),
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
                    blend: Some(wgpu::BlendState::REPLACE),
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
            uniform_buffer,
            mvp_bind_group,
            lighting_bind_group_layout,
            texture_bind_group_layout,
        })
    }

    pub fn update_uniforms(&self, queue: &Queue, uniform: &StandardMVPUniform) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[*uniform]));
    }

    pub fn render_model<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        model: &'a ModelAsset,
        lighting_bind_group: &'a wgpu::BindGroup,
        texture_bind_group: &'a wgpu::BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.mvp_bind_group, &[0]);
        render_pass.set_bind_group(1, lighting_bind_group, &[]);
        render_pass.set_bind_group(2, texture_bind_group, &[]);
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
        lighting_bind_group: &'a wgpu::BindGroup,
        texture_bind_group: &'a wgpu::BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.mvp_bind_group, &[0]);
        render_pass.set_bind_group(1, lighting_bind_group, &[]);
        render_pass.set_bind_group(2, texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);
    }

    /// Render a mesh using this shader
    pub fn render_mesh<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        mesh: &'a crate::graphics::Mesh,
        lighting_bind_group: &'a wgpu::BindGroup,
        texture_bind_group: &'a wgpu::BindGroup,
    ) {
        self.render_geometry(
            render_pass,
            &mesh.vertex_buffer,
            &mesh.index_buffer,
            mesh.num_indices,
            lighting_bind_group,
            texture_bind_group,
        );
    }
}
