/// Shared uniform definitions for all shaders
/// This module provides DRY (Don't Repeat Yourself) uniform structures for consistent
/// usage across all WGSL shaders and their corresponding Rust implementations.
use bytemuck::{Pod, Zeroable};

/// Standardized MVP uniform structure used by ALL shaders
/// This replaces the previous separate CameraUniform and TransformUniform structures
/// to eliminate the NaN issue caused by f32 precision loss at astronomical distances.
///
/// All matrix calculations are performed in 64-bit precision on the CPU side,
/// and only the final MVP matrix is converted to f32 for GPU usage.
///
/// For atmospheric effects, we use relative transforms and directions instead of
/// absolute world coordinates to avoid precision issues at astronomical scales.
// New efficient uniform separation

/// Camera/Frame uniform - data that changes once per frame
/// Contains only essential camera data, no redundant matrices
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    /// Camera world position for lighting calculations
    pub camera_position: [f32; 3], // 12 bytes
    pub _padding1: f32, // 4 bytes

    /// Camera direction vector for view-dependent effects  
    pub camera_direction: [f32; 3], // 12 bytes
    pub _padding2: f32, // 4 bytes

    /// Logarithmic depth buffer support
    pub log_depth_constant: f32, // 4 bytes
    pub far_plane_distance: f32,  // 4 bytes
    pub near_plane_distance: f32, // 4 bytes
    pub fc_constant: f32,         // 4 bytes
} // Total: 48 bytes

/// Object uniform - data that changes per object
/// Contains all matrices needed for rendering each object
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ObjectUniform {
    /// Pre-computed Model-View-Projection matrix (64-bit precision on CPU)
    pub mvp_matrix: [[f32; 4]; 4], // 64 bytes

    /// Model matrix for normal calculations and world space transforms
    pub model_matrix: [[f32; 4]; 4], // 64 bytes

    /// Camera-relative model transform (transforms model vertices to camera-relative space)
    pub mv_matrix: [[f32; 4]; 4], // 64 bytes

    /// Light direction in camera space for per-object lighting
    pub light_direction_camera_space: [f32; 3], // 12 bytes
    pub _padding: f32, // 4 bytes
} // Total: 208 bytes

// Legacy struct for backwards compatibility - will be removed
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct StandardMVPUniform {
    pub mvp_matrix: [[f32; 4]; 4],
    pub camera_position: [f32; 3],
    pub _padding1: f32,
    pub camera_direction: [f32; 3],
    pub _padding2: f32,
    pub log_depth_constant: f32,
    pub far_plane_distance: f32,
    pub near_plane_distance: f32,
    pub fc_constant: f32,
    pub mv_matrix: [[f32; 4]; 4],
    pub light_direction_camera_space: [f32; 3],
    pub _padding3: f32,
} // Total: 240 bytes

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            camera_position: [0.0; 3],
            _padding1: 0.0,
            camera_direction: [0.0, 0.0, -1.0], // Default forward direction
            _padding2: 0.0,
            log_depth_constant: 1.0,
            far_plane_distance: 1e11,
            near_plane_distance: 1e3,
            fc_constant: 1.0,
        }
    }
}

impl Default for ObjectUniform {
    fn default() -> Self {
        Self {
            mvp_matrix: {
                let mut matrix = [[0.0; 4]; 4];
                // Identity matrix
                matrix[0][0] = 1.0;
                matrix[1][1] = 1.0;
                matrix[2][2] = 1.0;
                matrix[3][3] = 1.0;
                matrix
            },
            model_matrix: {
                let mut matrix = [[0.0; 4]; 4];
                // Identity matrix
                matrix[0][0] = 1.0;
                matrix[1][1] = 1.0;
                matrix[2][2] = 1.0;
                matrix[3][3] = 1.0;
                matrix
            },
            mv_matrix: [[0.0; 4]; 4],
            light_direction_camera_space: [0.0, 0.0, -1.0], // Default light direction
            _padding: 0.0,
        }
    }
}

impl Default for StandardMVPUniform {
    fn default() -> Self {
        Self {
            mvp_matrix: [[0.0; 4]; 4],
            camera_position: [0.0; 3],
            _padding1: 0.0,
            camera_direction: [0.0, 0.0, -1.0],
            _padding2: 0.0,
            log_depth_constant: 1.0,
            far_plane_distance: 1e11,
            near_plane_distance: 1e3,
            fc_constant: 1.0,
            mv_matrix: [[0.0; 4]; 4],
            light_direction_camera_space: [0.0, 0.0, -1.0],
            _padding3: 0.0,
        }
    }
}

/// Legacy uniform structures for backward compatibility during transition
/// These will be removed once all shaders are converted to StandardMVPUniform

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct LegacyCameraUniform {
    pub view_matrix: [[f32; 4]; 4],            // 64 bytes
    pub projection_matrix: [[f32; 4]; 4],      // 64 bytes
    pub view_projection_matrix: [[f32; 4]; 4], // 64 bytes
    pub camera_position: [f32; 3],             // 12 bytes
    pub _padding1: f32,                        // 4 bytes
    pub camera_direction: [f32; 3],            // 12 bytes
    pub _padding2: f32,                        // 4 bytes
    pub log_depth_constant: f32,               // 4 bytes
    pub far_plane_distance: f32,               // 4 bytes
    pub near_plane_distance: f32,              // 4 bytes
    pub fc_constant: f32,                      // 4 bytes
} // Total: 240 bytes

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct LegacyTransformUniform {
    pub model_matrix: [[f32; 4]; 4],      // 64 bytes
    pub model_view_matrix: [[f32; 4]; 4], // 64 bytes
    pub normal_matrix: [[f32; 4]; 3],     // 48 bytes (mat3x3 stored as 3 vec4 for alignment)
    pub _padding: [f32; 4],               // 16 bytes
} // Total: 192 bytes

/// Helper functions for creating GPU buffers with standardized uniforms
pub mod buffer_helpers {
    use super::*;
    use wgpu::{Buffer, Device};

    /// Create a buffer for CameraUniform
    pub fn create_camera_buffer(device: &Device, label: Option<&str>) -> Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    /// Create a buffer for ObjectUniform
    pub fn create_object_buffer(device: &Device, label: Option<&str>) -> Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: std::mem::size_of::<ObjectUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    /// Create a bind group layout for CameraUniform + ObjectUniform
    pub fn create_camera_object_bind_group_layout(
        device: &Device,
        label: Option<&str>,
    ) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: &[
                // Binding 0: Camera uniform
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Binding 1: Object uniform
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        })
    }

    /// Create a buffer for StandardMVPUniform
    pub fn create_mvp_uniform_buffer(device: &Device, label: Option<&str>) -> Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: std::mem::size_of::<StandardMVPUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    /// Create a bind group layout for StandardMVPUniform
    pub fn create_mvp_bind_group_layout(
        device: &Device,
        label: Option<&str>,
    ) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
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

    /// Create a bind group layout for StandardMVPUniform with dynamic offsets
    pub fn create_mvp_bind_group_layout_dynamic(
        device: &Device,
        label: Option<&str>,
    ) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(std::num::NonZeroU64::new(256).unwrap()), // 256 bytes per uniform (aligned)
                },
                count: None,
            }],
        })
    }

    /// Create a large MVP uniform buffer for multiple objects with dynamic offsets
    /// Buffer size calculation: 256 bytes per uniform (aligned) * max_objects
    pub fn create_dynamic_mvp_uniform_buffer(
        device: &Device,
        max_objects: u32,
        label: Option<&str>,
    ) -> Buffer {
        const MVP_UNIFORM_SIZE: u64 = 256; // Aligned size for dynamic offsets
        let total_size = MVP_UNIFORM_SIZE * max_objects as u64;

        device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: total_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    /// Create a bind group for StandardMVPUniform
    pub fn create_mvp_bind_group(
        device: &Device,
        layout: &wgpu::BindGroupLayout,
        buffer: &Buffer,
        label: Option<&str>,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }

    /// Create a bind group for dynamic MVP uniform buffer
    pub fn create_dynamic_mvp_bind_group(
        device: &Device,
        layout: &wgpu::BindGroupLayout,
        buffer: &Buffer,
        label: Option<&str>,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer,
                    offset: 0,
                    size: Some(std::num::NonZeroU64::new(256).unwrap()), // Size of one uniform block
                }),
            }],
        })
    }
}
