// Lens glow shader package for stellar lens flare effects
// Uses shared uniform definitions for consistency

import super::shared::{StandardMVPUniform, model_to_clip_coordinates, VertexInput};

struct LensGlowUniform {
    glow_size: f32,           // Size factor for the glow
    screen_width: f32,        // Screen width in pixels
    screen_height: f32,       // Screen height in pixels
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

@group(0) @binding(0) var<uniform> mvp: StandardMVPUniform;
@group(1) @binding(0) var<uniform> lens_glow: LensGlowUniform;
@group(1) @binding(1) var glow_texture: texture_2d<f32>;
@group(1) @binding(2) var glow_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // BILLBOARD BEHAVIOR: Transform the star center to clip space first
    let star_center = vec4<f32>(0.0, 0.0, 0.0, 1.0); // Model space center
    var center_clip = mvp.mvp_matrix * star_center;
    
    // Perform perspective divide to get normalized device coordinates
    let center_ndc = center_clip.xyz / center_clip.w;
    
    // Add the quad offset exactly like Java does
    // Java: offsetX = (width/screenWidth), offsetY = (height/screenHeight)
    // Both width and height have the same value in Java: (size / aspect_ratio)
    let aspect_ratio = lens_glow.screen_width / lens_glow.screen_height;
    let size_with_aspect = lens_glow.glow_size / aspect_ratio;
    
    let offset_x = size_with_aspect / lens_glow.screen_width;
    let offset_y = size_with_aspect / lens_glow.screen_height;
    
    let screen_offset = vec2<f32>(input.position.x * offset_x, input.position.y * offset_y);
    
    // Final position: star center + screen-aligned offset
    out.clip_position = vec4<f32>(
        center_ndc.x + screen_offset.x,
        center_ndc.y + screen_offset.y,
        0.001, // Force to render on top (near plane)
        1.0
    );
    
    out.tex_coord = input.tex_coord;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the original lens glow texture (star_glow.png)
    let tex_color = textureSample(glow_texture, glow_sampler, input.tex_coord);
    
    // For now, use the original texture directly without modification
    // The additive blending will handle the final color mixing
    return vec4<f32>(tex_color.rgb, tex_color.a);
}