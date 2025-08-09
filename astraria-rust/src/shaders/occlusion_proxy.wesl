/// Invisible proxy geometry shader for GPU occlusion queries
/// Renders tiny transparent quads that only affect depth testing for visibility detection

// Import shared types and utilities
import shared;

/// Proxy geometry uniform for positioning quads in screen space
struct ProxyUniform {
    screen_position: vec2<f32>,  // NDC coordinates [-1, 1]
    quad_size: f32,              // Size in pixels (typically 2.0)
    screen_width: f32,           // Screen width for pixel-to-NDC conversion
    screen_height: f32,          // Screen height for pixel-to-NDC conversion
}

@group(0) @binding(0)
var<uniform> mvp: shared::StandardMVPUniform;

@group(1) @binding(0)
var<uniform> proxy: ProxyUniform;

/// Vertex shader input
struct VertexInput {
    @location(0) position: vec3<f32>,
}

/// Vertex shader output / Fragment shader input
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

/// Vertex shader: Position proxy quad at specified screen coordinates
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Convert quad size from pixels to NDC
    let pixel_to_ndc_x = 2.0 / proxy.screen_width;
    let pixel_to_ndc_y = 2.0 / proxy.screen_height;
    let quad_size_ndc_x = proxy.quad_size * pixel_to_ndc_x;
    let quad_size_ndc_y = proxy.quad_size * pixel_to_ndc_y;
    
    // Scale input vertex position by quad size
    let scaled_position = vec2<f32>(
        input.position.x * quad_size_ndc_x,
        input.position.y * quad_size_ndc_y
    );
    
    // Position quad at target screen coordinates
    let final_position = proxy.screen_position + scaled_position;
    
    // Output in clip space coordinates
    // For proxy geometry, we want to render at the near plane to ensure proper depth testing
    // Use a small positive Z value to be just in front of near plane
    out.clip_position = vec4<f32>(final_position.x, final_position.y, 0.001, 1.0);
    
    return out;
}

/// Fragment shader: Output transparent pixel (invisible but participates in depth test)
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Completely transparent - this geometry is invisible but still triggers depth test
    // The occlusion query counts how many fragments pass all tests (including depth)
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}