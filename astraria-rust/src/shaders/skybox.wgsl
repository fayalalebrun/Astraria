// Skybox shader for cubemap background rendering
// Direct port from the original Astraria GLSL skybox shaders

struct CameraUniform {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    view_projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
    camera_direction: vec3<f32>,
    log_depth_constant: f32,
    far_plane_distance: f32,
    near_plane_distance: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var skybox_texture: texture_cube<f32>;

@group(1) @binding(1)
var skybox_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Use position as texture coordinates (like original)
    out.tex_coords = input.position;
    
    // Remove translation from view matrix for skybox effect
    var view_no_translation = camera.view_matrix;
    view_no_translation[3][0] = 0.0;
    view_no_translation[3][1] = 0.0;
    view_no_translation[3][2] = 0.0;
    
    // Transform position
    let clip_pos = camera.projection_matrix * view_no_translation * vec4<f32>(input.position, 1.0);
    
    // Set z = w for max depth (equivalent to gl_Position.xyww in original)
    out.clip_position = vec4<f32>(clip_pos.xy, clip_pos.w, clip_pos.w);
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the cubemap directly (like original)
    return textureSample(skybox_texture, skybox_sampler, input.tex_coords);
}