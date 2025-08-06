use crate::renderer::shader_utils::load_preprocessed_wgsl;
use crate::{graphics::Vertex, AstrariaResult};
use glam::Vec3;
use std::path::Path;
/// Sun shader for stellar temperature rendering (800K-30000K)
/// Based on Java SunShader class implementation
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue, RenderPass, RenderPipeline};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SunUniform {
    pub temperature: f32, // Star temperature in Kelvin (800-30000)
    pub _padding1: f32,
    pub _padding2: f32,
    pub _padding3: f32,
    pub camera_to_sun_direction: [f32; 3], // Direction from camera to sun (16-byte aligned)
    pub _padding4: f32,
    pub sun_position: [f32; 3], // Sun position relative to camera (16-byte aligned)
    pub _padding5: f32,
    pub _padding6: [f32; 16], // Additional padding to reach 112 bytes (64 bytes = 16 f32s)
}

pub struct SunShader {
    pub pipeline: RenderPipeline,
    pub bind_group_layout: BindGroupLayout,
    pub texture_bind_group_layout: BindGroupLayout,
    pub uniform_buffer: Buffer,
    pub bind_group: BindGroup,
}

impl SunShader {
    pub fn new(
        device: &Device,
        camera_bind_group_layout: &BindGroupLayout,
    ) -> AstrariaResult<Self> {
        // Load shader
        let shader_path = Path::new("src/shaders/sun_shader.wgsl");
        let shader_source = load_preprocessed_wgsl(shader_path)
            .map_err(|e| crate::AstrariaError::Graphics(format!("Failed to load shader: {}", e)))?;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sun Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create sun-specific bind group layout
        let sun_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Sun Bind Group Layout"),
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

        // Create texture bind group layout
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Sun Texture Bind Group Layout"),
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

        // Create render pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Sun Pipeline Layout"),
            bind_group_layouts: &[
                camera_bind_group_layout,   // group(0) - StandardMVPUniform
                &sun_bind_group_layout,     // group(1) - SunUniform
                &texture_bind_group_layout, // group(2) - textures and sampler
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sun Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3],
                }],
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

        // Create uniform buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sun Uniform Buffer"),
            size: std::mem::size_of::<SunUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Sun Bind Group"),
            layout: &sun_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Ok(Self {
            pipeline,
            bind_group_layout: sun_bind_group_layout,
            texture_bind_group_layout,
            uniform_buffer,
            bind_group,
        })
    }

    pub fn update_uniforms(
        &self,
        queue: &Queue,
        temperature: f32,
        sun_position: Vec3,
        camera_position: Vec3,
    ) {
        let camera_to_sun = (sun_position - camera_position).normalize();

        let uniforms = SunUniform {
            temperature,
            _padding1: 0.0,
            _padding2: 0.0,
            _padding3: 0.0,
            camera_to_sun_direction: camera_to_sun.to_array(),
            _padding4: 0.0,
            sun_position: sun_position.to_array(),
            _padding5: 0.0,
            _padding6: [0.0; 16],
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
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
        render_pass.set_bind_group(0, mvp_bind_group, &[0]); // group(0) - StandardMVPUniform
        render_pass.set_bind_group(1, &self.bind_group, &[]); // group(1) - SunUniform
        render_pass.set_bind_group(2, texture_bind_group, &[]); // group(2) - textures and sampler
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..index_count, 0, 0..1);
    }
}
