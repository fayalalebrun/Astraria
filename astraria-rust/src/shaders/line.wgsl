// Line shader for orbital path rendering with logarithmic depth
// Direct port from the original Astraria GLSL line shaders

// Standardized MVP uniform structure (shared across all shaders)
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
};

@group(0) @binding(0)
var<uniform> mvp: StandardMVPUniform;

// Legacy structures removed - now using StandardMVPUniform exclusively

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


@group(1) @binding(0)
var<uniform> line_uniform: LineUniform;

// WebGPU-compatible logarithmic depth buffer function
fn model_to_clip_coordinates(
    position: vec4<f32>,
    mvp_matrix: mat4x4<f32>,
    depth_constant: f32,
    far_plane_distance: f32
) -> vec4<f32> {
    var clip = mvp_matrix * position;
    
    // WebGPU logarithmic depth: maps to [0,1] range instead of OpenGL's [-1,1]
    let fcoef = 1.0 / log2(far_plane_distance + 1.0);
    clip.z = log2(max(1e-6, 1.0 + clip.w)) * fcoef * clip.w;
    
    return clip;
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Use logarithmic depth transformation
    out.clip_position = model_to_clip_coordinates(
        vec4<f32>(input.position, 1.0),
        mvp.mvp_matrix,
        mvp.log_depth_constant,
        mvp.far_plane_distance
    );
    
    out.log_z = out.clip_position.z;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return line_uniform.color; // Use color from uniform
}