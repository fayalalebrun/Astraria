use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;
/// Core rendering functionality shared between main app and shader testing
use wgpu::{Buffer, Device, Texture};

use crate::graphics::Vertex;

/// Shared uniform buffer structures - consolidated from all shaders
/// This is the master definition used by all shaders to eliminate code duplication

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_matrix: [[f32; 4]; 4],           // 64 bytes
    pub projection_matrix: [[f32; 4]; 4],     // 64 bytes
    pub view_projection_matrix: [[f32; 4]; 4], // 64 bytes
    pub camera_position: [f32; 3],            // 12 bytes
    pub _padding1: f32,                       // 4 bytes
    pub camera_direction: [f32; 3],           // 12 bytes
    pub _padding2: f32,                       // 4 bytes
    pub log_depth_constant: f32,              // 4 bytes
    pub far_plane_distance: f32,              // 4 bytes
    pub near_plane_distance: f32,             // 4 bytes
    pub fc_constant: f32,                     // 4 bytes (for logarithmic depth calculations)
}                                             // Total: 240 bytes

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    pub model_matrix: [[f32; 4]; 4],          // 64 bytes
    pub model_view_matrix: [[f32; 4]; 4],     // 64 bytes
    pub normal_matrix: [[f32; 4]; 3],         // 48 bytes (mat3x3 stored as 3 vec4 for alignment)
    pub _padding: [f32; 4],                   // 16 bytes
}                                             // Total: 192 bytes

use crate::renderer::buffers::LightingUniform;

/// Shared geometry creation functions
pub fn create_cube_geometry() -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        // Front face
        Vertex {
            position: [-1.0, -1.0, 1.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0, 1.0],
            tex_coord: [1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            tex_coord: [1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0, 1.0],
            tex_coord: [0.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        // Back face
        Vertex {
            position: [-1.0, -1.0, -1.0],
            tex_coord: [1.0, 0.0],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [-1.0, 1.0, -1.0],
            tex_coord: [1.0, 1.0],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0, -1.0],
            tex_coord: [0.0, 1.0],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [1.0, -1.0, -1.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, 0.0, -1.0],
        },
    ];

    let indices = vec![
        0, 1, 2, 2, 3, 0, // front
        4, 5, 6, 6, 7, 4, // back
    ];

    (vertices, indices)
}

pub fn create_sphere_geometry(radius: f32, rings: u32, sectors: u32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices
    for i in 0..=rings {
        let phi = std::f32::consts::PI * i as f32 / rings as f32;
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();

        for j in 0..=sectors {
            let theta = 2.0 * std::f32::consts::PI * j as f32 / sectors as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            let x = sin_phi * cos_theta;
            let y = cos_phi;
            let z = sin_phi * sin_theta;

            let u = j as f32 / sectors as f32;
            let v = i as f32 / rings as f32;

            vertices.push(Vertex {
                position: [x * radius, y * radius, z * radius],
                tex_coord: [u, v],
                normal: [x, y, z], // Normal is the same as position for unit sphere
            });
        }
    }

    // Generate indices
    for i in 0..rings {
        for j in 0..sectors {
            let first = i * (sectors + 1) + j;
            let second = first + sectors + 1;

            indices.push(first);
            indices.push(second);
            indices.push(first + 1);

            indices.push(second);
            indices.push(second + 1);
            indices.push(first + 1);
        }
    }

    (vertices, indices)
}

/// Create a simple quad geometry for billboard/sprite rendering (matching Java implementation)
pub fn create_quad_geometry() -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        // Simple quad from -1 to 1 in X and Y, Z=0 (matching Java Billboard.java)
        Vertex {
            position: [-1.0, 1.0, 0.0],
            tex_coord: [0.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        }, // top-left
        Vertex {
            position: [1.0, 1.0, 0.0],
            tex_coord: [1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        }, // top-right
        Vertex {
            position: [-1.0, -1.0, 0.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        }, // bottom-left
        Vertex {
            position: [1.0, -1.0, 0.0],
            tex_coord: [1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        }, // bottom-right
    ];

    let indices = vec![
        0, 2, 1, // First triangle
        1, 2, 3, // Second triangle
    ];

    (vertices, indices)
}

/// Create line geometry for orbital path rendering (matching Java implementation)
pub fn create_line_geometry() -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Create a simple orbital path - a circle with multiple points for line strip rendering
    let num_points = 64;
    let radius = 2.0;

    for i in 0..num_points {
        let angle = (i as f32 / num_points as f32) * 2.0 * std::f32::consts::PI;
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        let y = 0.0; // Orbital plane

        vertices.push(Vertex {
            position: [x, y, z],
            tex_coord: [0.0, 0.0],   // Not used for lines
            normal: [0.0, 1.0, 0.0], // Not used for lines
        });

        // For line strip, we connect each point to the next
        if i < num_points - 1 {
            indices.push(i);
            indices.push(i + 1);
        }
    }

    // Close the orbital path by connecting last point to first
    indices.push(num_points - 1);
    indices.push(0);

    (vertices, indices)
}

/// Create point geometry for distant object rendering (matching Java implementation)
pub fn create_point_geometry() -> (Vec<Vertex>, Vec<u32>) {
    // According to Java implementation, point shader uses single vertex at origin
    // Position is handled via model matrix transformation
    let vertices = vec![
        Vertex {
            position: [0.0, 0.0, 0.0], // Single vertex at origin
            tex_coord: [0.0, 0.0],     // Not used for points
            normal: [0.0, 0.0, 1.0],   // Not used for points
        },
        // Add a few more points to make a small cluster of distant objects
        Vertex {
            position: [0.5, 0.5, 0.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.0, -0.5, 0.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
    ];

    // For point rendering, indices are just sequential
    let indices = vec![0, 1, 2, 3];

    (vertices, indices)
}

/// Shared render setup functions
pub struct RenderSetup {
    pub camera_uniform: CameraUniform,
    pub transform_uniform: TransformUniform,
    pub lighting_uniform: LightingUniform,
}

impl RenderSetup {
    pub fn new(width: u32, height: u32) -> Self {
        let view_matrix = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
        let projection_matrix = Mat4::perspective_rh(
            45.0_f32.to_radians(),
            width as f32 / height as f32,
            0.1,
            100.0,
        );
        let camera_uniform = CameraUniform {
            view_matrix: view_matrix.to_cols_array_2d(),
            projection_matrix: projection_matrix.to_cols_array_2d(),
            view_projection_matrix: (projection_matrix * view_matrix).to_cols_array_2d(),
            camera_position: [0.0, 0.0, 3.0],
            _padding1: 0.0,
            camera_direction: [0.0, 0.0, -1.0],
            _padding2: 0.0,
            log_depth_constant: 1.0,
            far_plane_distance: 100.0,
            near_plane_distance: 0.1,
            fc_constant: 2.0 / (100.0f32 + 1.0).ln(),
        };

        let transform_uniform = TransformUniform {
            model_matrix: Mat4::IDENTITY.to_cols_array_2d(),
            model_view_matrix: (view_matrix * Mat4::IDENTITY).to_cols_array_2d(),
            normal_matrix: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
            _padding: [0.0; 4],
        };

        use crate::renderer::buffers::PointLight;
        let default_light = PointLight {
            position: [2.0, 2.0, 2.0],
            _padding1: 0.0,
            ambient: [0.1, 0.1, 0.1],
            _padding2: 0.0,
            diffuse: [1.0, 1.0, 1.0],
            _padding3: 0.0,
            specular: [1.0, 1.0, 1.0],
            _padding4: 0.0,
        };
        
        let mut lights = [PointLight {
            position: [0.0; 3],
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

        Self {
            camera_uniform,
            transform_uniform,
            lighting_uniform,
        }
    }

    pub fn create_buffers(&self, device: &Device) -> (Buffer, Buffer, Buffer) {
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[self.camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[self.transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let lighting_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Lighting Buffer"),
            contents: bytemuck::cast_slice(&[self.lighting_uniform]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        (camera_buffer, transform_buffer, lighting_buffer)
    }
}

/// Shared texture to buffer copy with proper alignment
pub fn copy_texture_to_buffer_aligned(
    encoder: &mut wgpu::CommandEncoder,
    texture: &Texture,
    output_buffer: &Buffer,
    width: u32,
    height: u32,
) {
    let bytes_per_pixel = 4; // RGBA8
    let unpadded_bytes_per_row = bytes_per_pixel * width;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row = (unpadded_bytes_per_row + align - 1) / align * align;

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::ImageCopyBuffer {
            buffer: output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        texture.size(),
    );
}

/// Calculate aligned buffer size for texture copying
pub fn calculate_aligned_buffer_size(width: u32, height: u32) -> wgpu::BufferAddress {
    let bytes_per_pixel = 4; // RGBA8
    let unpadded_bytes_per_row = bytes_per_pixel * width;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row = (unpadded_bytes_per_row + align - 1) / align * align;
    (padded_bytes_per_row * height) as wgpu::BufferAddress
}

/// Remove padding from buffer data for image creation
pub fn remove_padding_from_buffer_data(data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let bytes_per_pixel = 4; // RGBA8
    let unpadded_bytes_per_row = bytes_per_pixel * width;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row = (unpadded_bytes_per_row + align - 1) / align * align;

    if padded_bytes_per_row != unpadded_bytes_per_row {
        let mut unpadded_data = Vec::with_capacity((unpadded_bytes_per_row * height) as usize);
        for row in 0..height {
            let row_start = (row * padded_bytes_per_row) as usize;
            let row_end = row_start + unpadded_bytes_per_row as usize;
            unpadded_data.extend_from_slice(&data[row_start..row_end]);
        }
        unpadded_data
    } else {
        data.to_vec()
    }
}

/// Create bind group layout for camera uniforms
pub fn create_camera_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
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
    })
}

/// Create bind group layout for transform uniforms
pub fn create_transform_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Transform Bind Group Layout"),
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
    })
}
