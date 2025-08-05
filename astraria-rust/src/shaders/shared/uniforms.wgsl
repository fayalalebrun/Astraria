// Shared uniform definitions for all shaders
// This file is included by all WGSL shaders to ensure consistency

// Legacy StandardMVPUniform structure for backward compatibility
// TODO: This will be replaced with separate CameraUniform + ObjectUniform
struct StandardMVPUniform {
    mvp_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
    _padding1: f32,
    camera_direction: vec3<f32>,
    _padding2: f32,
    log_depth_constant: f32,
    far_plane_distance: f32,
    near_plane_distance: f32,
    fc_constant: f32,
    mv_matrix: mat4x4<f32>,
    _padding3: f32,
    _padding4: f32,
};

// Legacy binding for backward compatibility 
@group(0) @binding(0) var<uniform> mvp: StandardMVPUniform;

// Note: Shaders should use mvp.field_name to access uniform data
// Example: mvp.camera_position, mvp.mvp_matrix, etc.