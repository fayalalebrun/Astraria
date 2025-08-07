// Lens glow shader package for stellar lens flare effects
// Uses shared uniform definitions for consistency

import super::shared::{StandardMVPUniform, model_to_clip_coordinates, VertexInput};

struct LensGlowUniform {
    screen_dimensions: vec2<f32>,   // Screen width and height (8 bytes)
    glow_size: vec2<f32>,          // Width and height of the glow effect (8 bytes, total: 16)  
    star_position: vec3<f32>,      // Star position in world coordinates (12 bytes)
    _padding1: f32,                // Align to 16-byte boundary (4 bytes, total: 32)
    camera_direction: vec3<f32>,   // Camera forward direction (12 bytes)
    temperature: f32,              // Star temperature for spectrum mapping (4 bytes, total: 48)
};


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) dot_view: f32,
};

@group(0) @binding(0) var<uniform> mvp: StandardMVPUniform;

@group(1) @binding(0) var<uniform> lens_glow: LensGlowUniform;

@group(2) @binding(0) var glow_texture: texture_2d<f32>;
@group(2) @binding(1) var spectrum_texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;

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
    out.tex_coord = input.tex_coord;
    out.dot_view = 1.0; // Placeholder for lens flare visibility calculation
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample glow texture
    let glow_color = textureSample(glow_texture, texture_sampler, input.tex_coord);
    
    // Map temperature to spectrum
    let u = (lens_glow.temperature - 800.0) / 29200.0; // 800K to 30000K range
    let spectrum_color = textureSample(spectrum_texture, texture_sampler, vec2<f32>(u, 0.5));
    
    // Apply lens flare effect
    let final_color = glow_color.rgb * spectrum_color.rgb * input.dot_view;
    
    return vec4<f32>(final_color, glow_color.a);
}