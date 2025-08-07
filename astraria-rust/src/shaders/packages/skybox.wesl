// Skybox shader package for cubemap background rendering
// Uses shared uniform definitions for consistency

import shared::{StandardMVPUniform, mvp, model_to_clip_coordinates};

struct VertexInput {
    position: vec3<f32> @location(0),
};

struct VertexOutput {
    clip_position: vec4<f32> @builtin(position),
    tex_coords: vec3<f32> @location(0),
};

// Note: StandardMVPUniform is declared in shared.wesl at @group(0) @binding(0)

@group(1) @binding(0)
var skybox_texture: texture_cube<f32>;

@group(1) @binding(1)
var skybox_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Use position as texture coordinates (like original)
    out.tex_coords = input.position;
    
    // Use pre-computed MVP matrix (translation removal handled in CPU for skybox)
    // The CPU computes a special skybox MVP matrix with translation removed for precision
    var clip_pos = mvp.mvp_matrix * vec4<f32>(input.position, 1.0);
    
    // Force depth to maximum value (skybox is at infinite distance)
    // In WebGPU, depth range is [0,1] with 1.0 being farthest
    out.clip_position = vec4<f32>(clip_pos.x, clip_pos.y, clip_pos.w, clip_pos.w);
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the cubemap directly (like original)
    return textureSample(skybox_texture, skybox_sampler, input.tex_coords);
}