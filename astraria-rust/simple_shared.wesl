// Simple shared definitions without attributes

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
};

struct VertexInput {
    position: vec3<f32>,
    tex_coord: vec2<f32>, 
    normal: vec3<f32>,
};

struct VertexOutput {
    clip_position: vec4<f32>,
    tex_coord: vec2<f32>,
    world_normal: vec3<f32>,
    world_position: vec3<f32>,
};