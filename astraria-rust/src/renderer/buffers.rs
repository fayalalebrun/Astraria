use crate::{assets::AssetManager, renderer::core::*, AstrariaResult};
use glam::{Mat4, Vec3};
/// Buffer management for vertex data, uniforms, and other GPU resources
use wgpu::{util::DeviceExt, BindGroup, Buffer, Device, Queue, Sampler};

// CameraUniform and TransformUniform are now imported from core.rs to eliminate duplication

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DirectionalLight {
    pub direction: [f32; 3], // Normalized direction from object to light
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
pub struct LightingUniform {
    pub lights: [DirectionalLight; 8],
    pub num_lights: i32,
    pub _padding: [f32; 3],
}

pub struct BufferManager {
    pub camera_buffer: Buffer,
    pub transform_buffer: Buffer,
    pub triangle_transform_buffer: Buffer,
    pub cube_transform_buffer: Buffer,
    pub lighting_buffer: Buffer,
    pub camera_bind_group: BindGroup,
    pub transform_bind_group: BindGroup,
    pub triangle_transform_bind_group: BindGroup,
    pub cube_transform_bind_group: BindGroup,
    pub lighting_bind_group: BindGroup,
    pub default_texture_bind_group: BindGroup,
    pub default_sampler: Sampler,
}

impl BufferManager {
    pub fn new(
        device: &Device,
        asset_manager: &mut AssetManager,
        queue: &Queue,
    ) -> AstrariaResult<Self> {
        // Create uniform buffers
        let camera_uniform = CameraUniform {
            view_matrix: Mat4::IDENTITY.to_cols_array_2d(),
            projection_matrix: Mat4::IDENTITY.to_cols_array_2d(),
            view_projection_matrix: Mat4::IDENTITY.to_cols_array_2d(),
            camera_position: [0.0, 0.0, 0.0],
            _padding1: 0.0,
            camera_direction: [0.0, 0.0, -1.0],
            _padding2: 0.0,
            log_depth_constant: 1.0,
            far_plane_distance: 1e11,
            near_plane_distance: 0.1,
            fc_constant: 2.0 / (1e11f32 + 1.0).ln(),
        };

        let transform_uniform = TransformUniform {
            model_matrix: Mat4::IDENTITY.to_cols_array_2d(),
            model_view_matrix: Mat4::IDENTITY.to_cols_array_2d(),
            normal_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
            _padding: [0.0; 4],
        };

        let default_light = DirectionalLight {
            direction: [1.0, 1.0, -1.0], // Light from upper right
            _padding1: 0.0,
            ambient: [0.1, 0.1, 0.1],
            _padding2: 0.0,
            diffuse: [1.0, 1.0, 1.0],
            _padding3: 0.0,
            specular: [1.0, 1.0, 1.0],
            _padding4: 0.0,
        };

        let mut lights = [DirectionalLight {
            direction: [0.0, 0.0, -1.0],
            _padding1: 0.0,
            ambient: [0.0; 3],
            _padding2: 0.0,
            diffuse: [0.0; 3],
            _padding3: 0.0,
            specular: [0.0; 3],
            _padding4: 0.0,
        }; 8];
        lights[0] = default_light;

        let lighting_uniform = LightingUniform {
            lights,
            num_lights: 1,
            _padding: [0.0; 3],
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create separate transform buffers for triangle and cube
        let triangle_transform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Triangle Transform Buffer"),
                contents: bytemuck::cast_slice(&[transform_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let cube_transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let lighting_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Lighting Buffer"),
            contents: bytemuck::cast_slice(&[lighting_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layouts (these should match the pipeline)
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

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

        // Create bind groups
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transform Bind Group"),
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        let triangle_transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Triangle Transform Bind Group"),
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: triangle_transform_buffer.as_entire_binding(),
            }],
        });

        let cube_transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Cube Transform Bind Group"),
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: cube_transform_buffer.as_entire_binding(),
            }],
        });

        let lighting_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Lighting Bind Group"),
            layout: &lighting_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: lighting_buffer.as_entire_binding(),
            }],
        });

        // Create default texture and sampler
        let default_texture = asset_manager.create_default_texture(device, queue)?;

        let default_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Default Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

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

        let default_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Default Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&default_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&default_sampler),
                },
            ],
        });

        Ok(Self {
            camera_buffer,
            transform_buffer,
            triangle_transform_buffer,
            cube_transform_buffer,
            lighting_buffer,
            camera_bind_group,
            transform_bind_group,
            triangle_transform_bind_group,
            cube_transform_bind_group,
            lighting_bind_group,
            default_texture_bind_group,
            default_sampler,
        })
    }

    pub fn update_camera(
        &self,
        queue: &Queue,
        view: Mat4,
        proj: Mat4,
        view_proj: Mat4,
        view_pos: Vec3,
        view_dir: Vec3,
    ) {
        let camera_uniform = CameraUniform {
            view_matrix: view.to_cols_array_2d(),
            projection_matrix: proj.to_cols_array_2d(),
            view_projection_matrix: view_proj.to_cols_array_2d(),
            camera_position: view_pos.to_array(),
            _padding1: 0.0,
            camera_direction: view_dir.to_array(),
            _padding2: 0.0,
            log_depth_constant: 1.0,
            far_plane_distance: 1e11,
            near_plane_distance: 0.1,
            fc_constant: 2.0 / (1e11f32 + 1.0).ln(),
        };

        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    pub fn update_transform(&self, queue: &Queue, model: Mat4) {
        let view_matrix = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
        let model_view_matrix = view_matrix * model;
        let normal_matrix = model_view_matrix.inverse().transpose();
        let transform_uniform = TransformUniform {
            model_matrix: model.to_cols_array_2d(),
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

        queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
    }

    pub fn update_lighting(&self, queue: &Queue, lights: &[DirectionalLight]) {
        let mut lighting_lights = [DirectionalLight {
            direction: [0.0, 0.0, -1.0],
            _padding1: 0.0,
            ambient: [0.0; 3],
            _padding2: 0.0,
            diffuse: [0.0; 3],
            _padding3: 0.0,
            specular: [0.0; 3],
            _padding4: 0.0,
        }; 8];

        let num_lights = lights.len().min(8);
        lighting_lights[..num_lights].copy_from_slice(&lights[..num_lights]);

        let lighting_uniform = LightingUniform {
            lights: lighting_lights,
            num_lights: num_lights as i32,
            _padding: [0.0; 3],
        };

        queue.write_buffer(
            &self.lighting_buffer,
            0,
            bytemuck::cast_slice(&[lighting_uniform]),
        );
    }

    pub fn update_triangle_transform(&self, queue: &Queue, model: Mat4) {
        let view_matrix = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
        let model_view_matrix = view_matrix * model;
        let normal_matrix = model_view_matrix.inverse().transpose();
        let transform_uniform = TransformUniform {
            model_matrix: model.to_cols_array_2d(),
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

        queue.write_buffer(
            &self.triangle_transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
    }

    pub fn update_cube_transform(&self, queue: &Queue, model: Mat4) {
        let view_matrix = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
        let model_view_matrix = view_matrix * model;
        let normal_matrix = model_view_matrix.inverse().transpose();
        let transform_uniform = TransformUniform {
            model_matrix: model.to_cols_array_2d(),
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

        queue.write_buffer(
            &self.cube_transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
    }
}
