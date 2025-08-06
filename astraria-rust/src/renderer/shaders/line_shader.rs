use crate::renderer::shader_utils::load_preprocessed_wgsl;
use bytemuck::{Pod, Zeroable};
use std::path::Path;
/// Line shader for orbital path rendering
/// Renders line geometry with logarithmic depth buffer support
use wgpu::{Device, Queue, RenderPipeline};

use crate::AstrariaResult;

// CameraUniform and TransformUniform are now imported from core.rs to eliminate duplication

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LineUniform {
    pub color: [f32; 4], // Line color with alpha
}

pub struct LineShader {
    pub pipeline: RenderPipeline,
    pub line_buffer: wgpu::Buffer,
    pub line_bind_group: wgpu::BindGroup,
}

impl LineShader {
    pub fn new(device: &Device, queue: &Queue) -> AstrariaResult<Self> {
        let shader_path = Path::new("src/shaders/line.wgsl");
        let shader_source = load_preprocessed_wgsl(shader_path)
            .map_err(|e| crate::AstrariaError::Graphics(format!("Failed to load shader: {}", e)))?;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Line Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Camera bind group layout (group 0) - shared with other shaders
        let camera_bind_group_layout =
            crate::renderer::uniforms::buffer_helpers::create_mvp_bind_group_layout_dynamic(
                device,
                Some("Line MVP Bind Group Layout"),
            );

        // Line-specific bind group layout (group 1)
        let line_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Line Specific Bind Group Layout"),
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Line Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &line_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x3,
                    }],
                }],
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
                topology: wgpu::PrimitiveTopology::LineList,
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

        // Create line-specific uniform buffer (color)
        let line_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Line Uniform Buffer"),
            size: std::mem::size_of::<LineUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Initialize line color
        let line_uniform = LineUniform {
            color: [0.0, 1.0, 0.0, 1.0], // Green color for orbital paths
        };
        queue.write_buffer(&line_buffer, 0, bytemuck::cast_slice(&[line_uniform]));

        // Create line-specific bind group (group 1)
        let line_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Line Bind Group"),
            layout: &line_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: line_buffer.as_entire_binding(),
            }],
        });

        Ok(Self {
            pipeline,
            line_buffer,
            line_bind_group,
        })
    }

    pub fn update_line_color(&self, queue: &Queue, color: [f32; 4]) {
        let line_uniform = LineUniform { color };
        queue.write_buffer(&self.line_buffer, 0, bytemuck::cast_slice(&[line_uniform]));
    }
}
