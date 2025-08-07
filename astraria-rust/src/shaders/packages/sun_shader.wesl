// Sun shader package for stellar temperature rendering (800K-30000K)
// Uses shared uniform definitions for consistency

import shared::{StandardMVPUniform, mvp, model_to_clip_coordinates};

// Sun-specific uniforms
struct SunUniform {
    temperature: f32,           // Star temperature in Kelvin (800-30000)
    _padding1: f32,
    _padding2: f32,
    _padding3: f32,
    camera_to_sun_direction: vec3<f32>, // Direction from camera to sun (16-byte aligned)
    _padding4: f32,
    sun_position: vec3<f32>,    // Sun position relative to camera (16-byte aligned)
    _padding5: f32,
    _padding6: vec4<f32>,       // Additional padding to reach 112 bytes with alignment
    _padding7: vec4<f32>,
    _padding8: vec4<f32>,
};

struct VertexInput {
    position: vec3<f32> @location(0),
    tex_coord: vec2<f32> @location(1),
    normal: vec3<f32> @location(2),
};

struct VertexOutput {
    clip_position: vec4<f32> @builtin(position),
    tex_coords: vec2<f32> @location(0),
    normal: vec3<f32> @location(1),
    frag_pos: vec3<f32> @location(2),
    model_view_pos: vec3<f32> @location(3),
};

// Bind groups
// Note: StandardMVPUniform is declared above at @group(0) @binding(0)

@group(1) @binding(0)
var<uniform> sun: SunUniform;

@group(2) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(2) @binding(1)
var sun_gradient_texture: texture_2d<f32>;  
@group(2) @binding(2)
var texture_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Use pre-computed MVP matrix (calculated with 64-bit precision on CPU)
    let vertex_position = vec4<f32>(input.position, 1.0);
    out.clip_position = model_to_clip_coordinates(
        vertex_position,
        mvp.mvp_matrix,
        mvp.log_depth_constant,
        mvp.far_plane_distance
    );
    out.tex_coords = input.tex_coord;
    out.normal = input.normal;
    out.frag_pos = input.position; // Approximate world position for sun effect
    out.model_view_pos = input.position;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample base sun texture
    let base_color = textureSample(diffuse_texture, texture_sampler, input.tex_coords).rgb;
    
    // Map temperature to gradient texture coordinate
    // Temperature range: 800K to 30000K (total range: 29200K)
    let u = (sun.temperature - 800.0) / 29200.0;
    
    // Sample temperature gradient 
    let temperature_color = textureSample(sun_gradient_texture, texture_sampler, vec2<f32>(u, 0.5)).rgb;
    
    // Apply temperature coloring to base texture
    let final_color = base_color * temperature_color * 3.0; // Brighten the sun
    
    return vec4<f32>(final_color, 1.0);
}