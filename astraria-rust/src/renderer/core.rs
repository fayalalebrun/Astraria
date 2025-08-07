use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;
/// Core rendering functionality shared between main app and shader testing
use wgpu::{Buffer, Device, Texture};

use crate::generated_shaders::common::VertexInput;

/// Shared uniform buffer structures - consolidated from all shaders
/// This is the master definition used by all shaders to eliminate code duplication

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
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
    pub fc_constant: f32,                      // 4 bytes (for logarithmic depth calculations)
} // Total: 240 bytes

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    pub model_matrix: [[f32; 4]; 4],      // 64 bytes
    pub model_view_matrix: [[f32; 4]; 4], // 64 bytes
    pub normal_matrix: [[f32; 4]; 3],     // 48 bytes (mat3x3 stored as 3 vec4 for alignment)
    pub _padding: [f32; 4],               // 16 bytes
} // Total: 192 bytes

use crate::renderer::buffers::LightingUniform;

/// Create a skybox cube with all 6 faces visible from inside
pub fn create_cube_geometry() -> (Vec<VertexInput>, Vec<u32>) {
    // Delegate to regular test cube, not skybox
    crate::graphics::create_test_cube()
}

/// Unified render command enum that describes all shader types and their parameters
/// This eliminates code duplication by providing a single render interface
#[derive(Debug, Clone)]
pub enum RenderCommand {
    /// Default PBR shader for basic objects
    Default {
        mesh_type: MeshType,
        light_position: Vec3,
        light_color: Vec3,
    },

    /// Planet with atmospheric scattering
    AtmosphericPlanet {
        atmosphere_color: glam::Vec4,
        overglow: f32,
        use_ambient_texture: bool,
    },

    /// Sun/star with stellar surface rendering
    Sun { temperature: f32 },

    /// Skybox background
    Skybox,

    /// Billboard sprite (screen-aligned)
    Billboard,

    /// Lens glow/flare effect
    LensGlow,

    /// Black hole with gravitational lensing
    BlackHole,

    /// Orbital path lines
    Line { color: glam::Vec4 },

    /// Point rendering for distant objects
    Point,
}

/// Mesh types available for rendering
#[derive(Debug, Clone, Copy)]
pub enum MeshType {
    Cube,
    Sphere,
    Quad,
    Line,
    Point,
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
    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            aspect: wgpu::TextureAspect::All,
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: output_buffer,
            layout: wgpu::TexelCopyBufferLayout {
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
    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;
    (padded_bytes_per_row * height) as wgpu::BufferAddress
}

/// Remove padding from buffer data for image creation
pub fn remove_padding_from_buffer_data(data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let bytes_per_pixel = 4; // RGBA8
    let unpadded_bytes_per_row = bytes_per_pixel * width;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

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
