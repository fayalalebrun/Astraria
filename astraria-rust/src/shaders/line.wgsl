// Line shader for orbital path rendering with logarithmic depth
// Direct port from the original Astraria GLSL line shaders

struct CameraUniform {
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

struct LineUniform {
    color: vec4<f32>,  // Line color with alpha
};

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) log_z: f32,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> line_uniform: LineUniform;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Basic transformation to clip space
    out.clip_position = camera.projection_matrix * camera.view_matrix * vec4<f32>(input.position, 1.0);
    
    // Calculate logarithmic depth (corrected from original shader)
    if (out.clip_position.w > 0.0) {
        out.log_z = log(out.clip_position.w * camera.log_depth_constant + 1.0) * camera.fc_constant;
        out.clip_position.z = (2.0 * out.log_z - 1.0) * out.clip_position.w;
    } else {
        out.log_z = out.clip_position.z;
    }
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return line_uniform.color; // Use color from uniform
}