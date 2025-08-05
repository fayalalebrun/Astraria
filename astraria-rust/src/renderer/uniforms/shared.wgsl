// Shared WGSL uniform definitions for all Astraria shaders
// This file provides DRY (Don't Repeat Yourself) uniform structures for consistent
// usage across all shaders to eliminate duplication and ensure compatibility.

/// Standardized MVP uniform structure used by ALL shaders
/// This replaces the previous separate CameraUniform and TransformUniform structures
/// to eliminate the NaN issue caused by f32 precision loss at astronomical distances.
///
/// All matrix calculations are performed in 64-bit precision on the CPU side,
/// and only the final MVP matrix is converted to f32 for GPU usage.
struct StandardMVPUniform {
    /// Pre-computed Model-View-Projection matrix calculated in 64-bit precision
    /// This matrix already incorporates:
    /// - Model matrix (object position, rotation, scale)
    /// - View matrix (camera position and orientation)  
    /// - Projection matrix (perspective/orthographic projection)
    mvp_matrix: mat4x4<f32>,
    
    /// Camera world position (converted from DVec3 to f32 for GPU)
    /// Used for lighting calculations and atmospheric effects
    camera_position: vec3<f32>,
    _padding1: f32,
    
    /// Camera direction vector for view-dependent effects
    camera_direction: vec3<f32>,
    _padding2: f32,
    
    /// Logarithmic depth buffer support for astronomical scale rendering
    log_depth_constant: f32,
    far_plane_distance: f32,
    near_plane_distance: f32,
    fc_constant: f32,
};

/// Standard bind group 0 binding for MVP uniform (used by ALL shaders)
@group(0) @binding(0)
var<uniform> mvp: StandardMVPUniform;

/// WebGPU-compatible logarithmic depth buffer function
/// Based on Outerra's optimized implementation for [0,1] depth range
/// This function should be used by all shaders that need logarithmic depth support
fn model_to_clip_coordinates(
    position: vec4<f32>,
    mvp_matrix: mat4x4<f32>,
    depth_constant: f32,
    far_plane_distance: f32
) -> vec4<f32> {
    var clip = mvp_matrix * position;
    
    // WebGPU logarithmic depth: maps to [0,1] range instead of OpenGL's [-1,1]
    // Formula: z = log2(max(1e-6, 1.0 + w)) * Fcoef
    // where Fcoef = 1.0 / log2(farplane + 1.0) for [0,1] range
    let fcoef = 1.0 / log2(far_plane_distance + 1.0);
    clip.z = log2(max(1e-6, 1.0 + clip.w)) * fcoef * clip.w;
    
    return clip;
}

/// Simplified logarithmic depth function using the standard MVP uniform
/// This is the preferred function for most shaders
fn apply_logarithmic_depth(position: vec4<f32>) -> vec4<f32> {
    return model_to_clip_coordinates(
        position,
        mvp.mvp_matrix,
        mvp.log_depth_constant,
        mvp.far_plane_distance
    );
}

/// Legacy uniform structures for backward compatibility during transition
/// These will be removed once all shaders are converted to StandardMVPUniform

struct LegacyCameraUniform {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    view_projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
    _padding1: f32,
    camera_direction: vec3<f32>,
    _padding2: f32,
    log_depth_constant: f32,
    far_plane_distance: f32,
    near_plane_distance: f32,
    fc_constant: f32,
};

struct LegacyTransformUniform {
    model_matrix: mat4x4<f32>,
    model_view_matrix: mat4x4<f32>,
    normal_matrix: mat3x3<f32>,
};