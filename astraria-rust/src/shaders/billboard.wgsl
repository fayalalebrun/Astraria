// Billboard shader for screen-aligned sprite rendering  
// Refactored to use standardized MVP matrix approach with 64-bit precision calculations

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

@group(1) @binding(0)
var<uniform> billboard: BillboardUniform;

// Note: StandardMVPUniform is declared above at @group(0) @binding(0)

// Logarithmic depth buffer transformation (from original implementation)
fn model_to_clip_coordinates(
    position: vec4<f32>,
    model_view_perspective_matrix: mat4x4<f32>,
    depth_constant: f32,
    far_plane_distance: f32
) -> vec4<f32> {
    var clip = model_view_perspective_matrix * position;
    
    // WebGPU logarithmic depth: maps to [0,1] range instead of OpenGL's [-1,1]
    // Formula: z = log2(max(1e-6, 1.0 + w)) * Fcoef
    // where Fcoef = 1.0 / log2(farplane + 1.0) for [0,1] range
    let fcoef = 1.0 / log2(far_plane_distance + 1.0);
    clip.z = log2(max(1e-6, 1.0 + clip.w)) * fcoef * clip.w;
    
    return clip;
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Use pre-computed MVP matrix (calculated with 64-bit precision on CPU)
    out.clip_position = model_to_clip_coordinates(
        vec4<f32>(input.position, 1.0),
        mvp.mvp_matrix,
        mvp.log_depth_constant,
        mvp.far_plane_distance
    );
    out.tex_coords = input.tex_coord;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Return a bright billboard color for testing
    return vec4<f32>(1.0, 0.5, 0.0, 1.0); // Orange color to show it's working
}