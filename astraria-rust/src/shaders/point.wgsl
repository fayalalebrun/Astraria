// Point shader for distant object rendering with logarithmic depth
// Direct port from the original Astraria GLSL point shaders

struct CameraUniform {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
    _padding1: f32,
    log_depth_constant: f32,
    far_plane_distance: f32,
    _padding2: vec2<f32>,
};

struct TransformUniform {
    model_matrix: mat4x4<f32>,
    model_view_matrix: mat4x4<f32>,
};

struct PointUniform {
    color: vec4<f32>,  // Point color with alpha
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Logarithmic depth buffer function (from original shader)
fn model_to_clip_coordinates(
    position: vec4<f32>,
    model_view_perspective_matrix: mat4x4<f32>,
    depth_constant: f32,
    far_plane_distance: f32
) -> vec4<f32> {
    var clip = model_view_perspective_matrix * position;
    
    // Apply logarithmic depth transformation
    clip.z = ((2.0 * log(depth_constant * clip.z + 1.0) / log(depth_constant * far_plane_distance + 1.0)) - 1.0) * clip.w;
    
    return clip;
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Simple transformation for testing
    out.clip_position = camera.projection_matrix * camera.view_matrix * vec4<f32>(input.position, 1.0);
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 1.0, 1.0); // Magenta color for distant objects
}