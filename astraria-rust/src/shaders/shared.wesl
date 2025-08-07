// Shared uniform definitions for all shaders - wesl package
// This package provides common uniform structures for consistent usage across all shaders

struct StandardMVPUniform {
    mvp_matrix: mat4x4<f32>,        // 0-63 (64 bytes)
    camera_position: vec3<f32>,     // 64-75 (12 bytes)
    _padding1: f32,                 // 76-79 (4 bytes padding)  
    camera_direction: vec3<f32>,    // 80-91 (12 bytes)
    _padding2: f32,                 // 92-95 (4 bytes padding)
    log_depth_constant: f32,        // 96-99 (4 bytes)
    far_plane_distance: f32,        // 100-103 (4 bytes)
    near_plane_distance: f32,       // 104-107 (4 bytes)
    fc_constant: f32,               // 108-111 (4 bytes)
    mv_matrix: mat4x4<f32>,         // 112-175 (64 bytes)
};

// Global binding for the MVP uniform will be declared in each shader that uses it

// Common vertex input structure
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>, 
    @location(2) normal: vec3<f32>,
};

// Common vertex output structure  
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
};

// WebGPU-compatible logarithmic depth buffer function
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