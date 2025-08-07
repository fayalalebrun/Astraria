// Point shader package for distant object rendering
// Uses shared uniform definitions for consistency

import shared::{StandardMVPUniform, mvp, model_to_clip_coordinates};

struct PointUniform {
    point_color: vec4<f32>,     // RGBA color of the point
    point_size: f32,            // Point size in pixels
    _padding1: f32,
    _padding2: f32,
    _padding3: f32,
};

struct VertexInput {
    position: vec3<f32> @location(0),
    tex_coord: vec2<f32> @location(1),
    normal: vec3<f32> @location(2),
};

struct VertexOutput {
    clip_position: vec4<f32> @builtin(position),
    point_size: f32 @builtin(point_size),
    color: vec4<f32> @location(0),
};

@group(1) @binding(0)
var<uniform> point: PointUniform;

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
    out.point_size = point.point_size;
    out.color = point.point_color;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple circular point rendering
    let center = vec2<f32>(0.5, 0.5);
    let coord = gl_PointCoord;
    let distance_from_center = length(coord - center);
    
    if (distance_from_center > 0.5) {
        discard;
    }
    
    return input.color;
}