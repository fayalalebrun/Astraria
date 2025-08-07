// Black hole shader package for gravitational lensing simulation
// Uses shared uniform definitions for consistency

import shared::{StandardMVPUniform, mvp, model_to_clip_coordinates};

struct BlackHoleUniform {
    schwarzschild_radius: f32,  // Schwarzschild radius for lensing calculations
    event_horizon: f32,         // Event horizon radius
    accretion_disk_inner: f32,  // Inner radius of accretion disk
    accretion_disk_outer: f32,  // Outer radius of accretion disk
    black_hole_position: vec3<f32>, // Position in world coordinates
    _padding: f32,
};

struct VertexInput {
    position: vec3<f32> @location(0),
    tex_coord: vec2<f32> @location(1),
    normal: vec3<f32> @location(2),
};

struct VertexOutput {
    clip_position: vec4<f32> @builtin(position),
    tex_coords: vec2<f32> @location(0),
    world_position: vec3<f32> @location(1),
};

@group(1) @binding(0)
var<uniform> black_hole: BlackHoleUniform;

@group(2) @binding(0)
var accretion_texture: texture_2d<f32>;

@group(2) @binding(1)
var texture_sampler: sampler;

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
    out.world_position = input.position;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate distance from fragment to black hole
    let distance_to_bh = length(input.world_position - black_hole.black_hole_position);
    
    // Simple black hole visualization
    if (distance_to_bh < black_hole.event_horizon) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Pure black inside event horizon
    } else if (distance_to_bh < black_hole.accretion_disk_outer && distance_to_bh > black_hole.accretion_disk_inner) {
        // Accretion disk
        let accretion_color = textureSample(accretion_texture, texture_sampler, input.tex_coords);
        return vec4<f32>(accretion_color.rgb * 2.0, 1.0); // Bright accretion disk
    } else {
        // Gravitational lensing effect (simplified)
        let lensing_factor = black_hole.schwarzschild_radius / distance_to_bh;
        return vec4<f32>(lensing_factor, lensing_factor * 0.5, 0.0, 0.1);
    }
}