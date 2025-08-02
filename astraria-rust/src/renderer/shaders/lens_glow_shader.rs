use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3};
use std::sync::Arc;
use wgpu::util::DeviceExt;
/// Lens glow shader for stellar lens flare effects
/// Renders lens flares with temperature-based colors and occlusion testing
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
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TransformUniform {
    pub model_matrix: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LensGlowUniform {
    pub screen_dimensions: [f32; 2], // Screen width and height  (8 bytes)
    pub glow_size: [f32; 2], // Width and height of the glow effect (8 bytes) = 16 bytes so far
    pub star_position: [f32; 3], // Star position in world coordinates (12 bytes)
    pub _padding1: f32,      // (4 bytes) = 16 bytes, total 32 bytes
    pub camera_direction: [f32; 3], // Camera forward direction (12 bytes)
    pub _padding2: f32,      // (4 bytes) = 16 bytes, total 48 bytes
    pub temperature: f32,    // Star temperature for spectrum mapping (4 bytes)
    pub _padding3: [f32; 7], // (28 bytes) = 32 bytes, total 80 bytes
}

pub struct LensGlowShader {
    pub pipeline: RenderPipeline,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: BindGroup,
    pub camera_buffer: Buffer,
    pub transform_buffer: Buffer,
    pub lens_glow_buffer: Buffer,
}

impl LensGlowShader {
    pub fn new(device: &Device, queue: &Queue) -> AstrariaResult<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Lens Glow Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/lens_glow.wgsl").into()),
        });

        // Uniform bind group layout (group 0) - 3 separate uniform buffers
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Lens Glow Uniform Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        // Texture bind group layout (group 1)
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Lens Glow Texture Bind Group Layout"),
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Lens Glow Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Lens Glow Pipeline"),
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

        // Create camera uniform buffer
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Lens Glow Camera Buffer"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create transform uniform buffer
        let transform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Lens Glow Transform Buffer"),
            size: std::mem::size_of::<TransformUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create lens glow uniform buffer
        let lens_glow_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Lens Glow Uniform Buffer"),
            size: std::mem::size_of::<LensGlowUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Initialize with placeholder data
        let camera_uniform = CameraUniform {
            view_matrix: glam::Mat4::IDENTITY.to_cols_array_2d(),
            projection_matrix: glam::Mat4::IDENTITY.to_cols_array_2d(),
        };

        let transform_uniform = TransformUniform {
            model_matrix: glam::Mat4::IDENTITY.to_cols_array_2d(),
        };

        let lens_glow_uniform = LensGlowUniform {
            screen_dimensions: [800.0, 600.0],
            glow_size: [100.0, 100.0],
            star_position: [0.0, 0.0, 0.0],
            _padding1: 0.0,
            camera_direction: [0.0, 0.0, -1.0],
            _padding2: 0.0,
            temperature: 5778.0,
            _padding3: [0.0; 7],
        };

        queue.write_buffer(&camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
        queue.write_buffer(
            &transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
        queue.write_buffer(
            &lens_glow_buffer,
            0,
            bytemuck::cast_slice(&[lens_glow_uniform]),
        );

        // Create bind group with all 3 uniform buffers
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Lens Glow Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: transform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: lens_glow_buffer.as_entire_binding(),
                },
            ],
        });

        Ok(Self {
            pipeline,
            uniform_bind_group_layout,
            texture_bind_group_layout,
            bind_group,
            camera_buffer,
            transform_buffer,
            lens_glow_buffer,
        })
    }

    pub fn update_uniforms(
        &self,
        device: &Device,
        queue: &Queue,
        view_matrix: Mat4,
        projection_matrix: Mat4,
        star_position: Vec3,
        camera_direction: Vec3,
        temperature: f32,
        screen_size: Vec2,
        glow_size: Vec2,
    ) -> AstrariaResult<wgpu::BindGroup> {
        // Camera uniforms
        let camera_uniform = CameraUniform {
            view_matrix: view_matrix.to_cols_array_2d(),
            projection_matrix: projection_matrix.to_cols_array_2d(),
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Lens Glow Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Transform uniforms
        let transform_uniform = TransformUniform {
            model_matrix: Mat4::IDENTITY.to_cols_array_2d(),
        };

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Lens Glow Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Lens glow uniforms
        let lens_glow_uniform = LensGlowUniform {
            screen_dimensions: screen_size.to_array(),
            glow_size: glow_size.to_array(),
            star_position: star_position.to_array(),
            _padding1: 0.0,
            camera_direction: camera_direction.to_array(),
            _padding2: 0.0,
            temperature,
            _padding3: [0.0; 7],
        };

        let lens_glow_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Lens Glow Uniform Buffer"),
            contents: bytemuck::cast_slice(&[lens_glow_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create uniform bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Lens Glow Uniform Bind Group"),
            layout: &self.uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: transform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: lens_glow_buffer.as_entire_binding(),
                },
            ],
        });

        Ok(uniform_bind_group)
    }

    pub fn render_geometry<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        vertex_buffer: &'a Buffer,
        index_buffer: &'a Buffer,
        num_indices: u32,
        uniform_bind_group: &'a BindGroup,
        texture_bind_group: &'a BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, uniform_bind_group, &[]);
        render_pass.set_bind_group(1, texture_bind_group, &[]);
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
        };

        let transform_uniform = TransformUniform {
            model_matrix: Mat4::IDENTITY.to_cols_array_2d(),
        };

        let lens_glow_uniform = LensGlowUniform {
            screen_dimensions: [800.0, 600.0],
            glow_size: [100.0, 100.0],
            star_position: [0.0, 0.0, -5.0], // Star position in front of camera
            _padding1: 0.0,
            camera_direction: [0.0, 0.0, -1.0],
            _padding2: 0.0,
            temperature: 5778.0, // Sun temperature
            _padding3: [0.0; 7],
        };

        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
        queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
        queue.write_buffer(
            &self.lens_glow_buffer,
            0,
            bytemuck::cast_slice(&[lens_glow_uniform]),
        );
    }
}
