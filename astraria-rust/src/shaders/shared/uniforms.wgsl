// Shared uniform definitions for all shaders
// This file is included by all WGSL shaders to ensure consistency

// StandardMVPUniform structure with explicit padding for WGSL vec3 alignment
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

// Legacy binding for backward compatibility 
@group(0) @binding(0) var<uniform> mvp: StandardMVPUniform;

// Note: Shaders should use mvp.field_name to access uniform data
// Example: mvp.camera_position, mvp.mvp_matrix, etc.