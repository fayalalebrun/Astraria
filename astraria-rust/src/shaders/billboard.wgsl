// Billboard shader for screen-aligned sprite rendering
// Direct port from the original Astraria GLSL billboard shaders

// Camera uniform (shared with other shaders)
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

// Transform uniform
struct TransformUniform {
    model_matrix: mat4x4<f32>,
    model_view_matrix: mat4x4<f32>,
    normal_matrix: mat3x3<f32>,
};

// Billboard-specific uniforms
struct BillboardUniform {
    billboard_width: f32,       // Width of billboard in pixels
    billboard_height: f32,      // Height of billboard in pixels
    screen_width: f32,          // Screen width in pixels
    screen_height: f32,         // Screen height in pixels
    billboard_origin: vec3<f32>, // 3D world position of billboard center
    _padding: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

// Bind groups - simplified for testing
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Logarithmic depth buffer transformation (from original implementation)
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
    
    // Simple quad rendering (for testing)
    out.clip_position = camera.projection_matrix * camera.view_matrix * vec4<f32>(input.position, 1.0);
    out.tex_coords = input.tex_coord;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Return a bright billboard color for testing
    return vec4<f32>(1.0, 0.5, 0.0, 1.0); // Orange color to show it's working
}