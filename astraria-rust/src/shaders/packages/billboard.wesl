// Billboard shader package for screen-aligned sprite rendering
// Uses shared uniform definitions for consistency

import shared::{StandardMVPUniform, mvp, model_to_clip_coordinates};

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
    position: vec3<f32> @location(0),
    tex_coord: vec2<f32> @location(1),
};

struct VertexOutput {
    clip_position: vec4<f32> @builtin(position),
    tex_coords: vec2<f32> @location(0),
};

@group(1) @binding(0)
var<uniform> billboard: BillboardUniform;

// Note: StandardMVPUniform is declared above at @group(0) @binding(0)

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