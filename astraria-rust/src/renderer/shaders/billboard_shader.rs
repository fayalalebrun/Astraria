use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3, Vec4};
use std::sync::Arc;
use wgpu::util::DeviceExt;
/// Billboard shader for screen-aligned sprites
/// Renders quads that always face the camera, maintaining constant pixel size
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue, RenderPass, RenderPipeline};

use crate::{
    assets::{AssetManager, ModelAsset},
    graphics::Vertex,
    renderer::core::*,
    AstrariaError, AstrariaResult,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub view_projection_matrix: [[f32; 4]; 4],
    pub camera_position: [f32; 3],
    pub _padding1: f32,
    pub camera_direction: [f32; 3],
    pub _padding2: f32,
    pub log_depth_constant: f32,
    pub far_plane_distance: f32,
    pub near_plane_distance: f32,
    pub _padding3: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TransformUniform {
    pub model_matrix: [[f32; 4]; 4],      // 64 bytes
    pub model_view_matrix: [[f32; 4]; 4], // 64 bytes
    pub normal_matrix: [[f32; 4]; 3],     // 48 bytes (mat3x3 stored as 3 vec4 for alignment)
    pub _padding: [f32; 4],               // 16 bytes (total = 192 bytes)
}

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
    pub bind_group: BindGroup,
    pub camera_buffer: wgpu::Buffer,
    camera_bind_group_layout: wgpu::BindGroupLayout,
    transform_bind_group_layout: wgpu::BindGroupLayout,
    billboard_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl BillboardShader {
    pub fn new(device: &Device, queue: &Queue) -> AstrariaResult<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Billboard Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/billboard.wgsl").into()),
        });

        // Camera bind group layout (group 0)
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
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

        // Transform bind group layout (group 1)
        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Transform Bind Group Layout"),
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

        // Billboard bind group layout (group 2)
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

        // Texture bind group layout (group 3)
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Billboard Texture Bind Group Layout"),
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Billboard Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
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
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
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
            depth_stencil: None, // No depth buffer for test mode
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Create a simple camera buffer for testing
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Billboard Camera Buffer"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create a simple bind group for testing
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Billboard Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        Ok(Self {
            pipeline,
            bind_group,
            camera_buffer,
            camera_bind_group_layout,
            transform_bind_group_layout,
            billboard_bind_group_layout,
            texture_bind_group_layout,
        })
    }

    pub fn update_uniforms(
        &self,
        device: &Device,
        queue: &Queue,
        view_matrix: Mat4,
        projection_matrix: Mat4,
        camera_position: Vec3,
        billboard_origin: Vec3,
        billboard_size: (f32, f32),
        screen_size: (f32, f32),
    ) -> AstrariaResult<(wgpu::BindGroup, wgpu::BindGroup, wgpu::BindGroup)> {
        // Camera uniforms
        let camera_uniform = CameraUniform {
            view_matrix: view_matrix.to_cols_array_2d(),
            projection_matrix: projection_matrix.to_cols_array_2d(),
            view_projection_matrix: (projection_matrix * view_matrix).to_cols_array_2d(),
            camera_position: camera_position.to_array(),
            _padding1: 0.0,
            camera_direction: [0.0, 0.0, -1.0],
            _padding2: 0.0,
            log_depth_constant: 1.0,
            far_plane_distance: 1000.0,
            near_plane_distance: 0.1,
            _padding3: 0.0,
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Billboard Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Transform uniforms
        let model_matrix = Mat4::IDENTITY;
        let model_view_matrix = view_matrix * model_matrix;
        let normal_matrix = model_view_matrix.inverse().transpose();

        let transform_uniform = TransformUniform {
            model_matrix: model_matrix.to_cols_array_2d(),
            model_view_matrix: model_view_matrix.to_cols_array_2d(),
            normal_matrix: [
                [
                    normal_matrix.x_axis.x,
                    normal_matrix.x_axis.y,
                    normal_matrix.x_axis.z,
                    0.0,
                ],
                [
                    normal_matrix.y_axis.x,
                    normal_matrix.y_axis.y,
                    normal_matrix.y_axis.z,
                    0.0,
                ],
                [
                    normal_matrix.z_axis.x,
                    normal_matrix.z_axis.y,
                    normal_matrix.z_axis.z,
                    0.0,
                ],
            ],
            _padding: [0.0; 4],
        };

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Billboard Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Billboard uniforms
        let billboard_uniform = BillboardUniform {
            billboard_width: billboard_size.0,
            billboard_height: billboard_size.1,
            screen_width: screen_size.0,
            screen_height: screen_size.1,
            billboard_origin: billboard_origin.to_array(),
            _padding: 0.0,
        };

        let billboard_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Billboard Uniform Buffer"),
            contents: bytemuck::cast_slice(&[billboard_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind groups
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Billboard Camera Bind Group"),
            layout: &self.camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Billboard Transform Bind Group"),
            layout: &self.transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        let billboard_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Billboard Bind Group"),
            layout: &self.billboard_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: billboard_buffer.as_entire_binding(),
            }],
        });

        Ok((
            camera_bind_group,
            transform_bind_group,
            billboard_bind_group,
        ))
    }

    pub fn render_geometry<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        vertex_buffer: &'a Buffer,
        index_buffer: &'a Buffer,
        num_indices: u32,
        camera_bind_group: &'a BindGroup,
        transform_bind_group: &'a BindGroup,
        billboard_bind_group: &'a BindGroup,
        texture_bind_group: &'a BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        render_pass.set_bind_group(1, transform_bind_group, &[]);
        render_pass.set_bind_group(2, billboard_bind_group, &[]);
        render_pass.set_bind_group(3, texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);
    }

    pub fn update_simple_uniforms(
        &self,
        queue: &Queue,
        view_matrix: Mat4,
        projection_matrix: Mat4,
    ) {
        let camera_uniform = CameraUniform {
            view_matrix: view_matrix.to_cols_array_2d(),
            projection_matrix: projection_matrix.to_cols_array_2d(),
            view_projection_matrix: (projection_matrix * view_matrix).to_cols_array_2d(),
            camera_position: [0.0, 0.0, 3.0],
            _padding1: 0.0,
            camera_direction: [0.0, 0.0, -1.0],
            _padding2: 0.0,
            log_depth_constant: 1.0,
            far_plane_distance: 1000.0,
            near_plane_distance: 0.1,
            _padding3: 0.0,
        };

        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }
}
