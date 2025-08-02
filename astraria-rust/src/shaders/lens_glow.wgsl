// Lens glow shader for stellar lens flare effects
// Direct port from the original Astraria GLSL lensGlow shaders

struct CameraUniform {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
};

struct TransformUniform {
    model_matrix: mat4x4<f32>,
};

struct LensGlowUniform {
    screen_dimensions: vec2<f32>,   // Screen width and height
    glow_size: vec2<f32>,          // Width and height of the glow effect  
    star_position: vec3<f32>,      // Star position in world coordinates
    _padding1: f32,
    camera_direction: vec3<f32>,   // Camera forward direction
    _padding2: f32,
    temperature: f32,              // Star temperature for spectrum mapping
    _padding3: vec3<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,  // Billboard quad positions (-1 to 1)
    @location(1) tex_coord: vec2<f32>, // Texture coordinates
    @location(2) normal: vec3<f32>,    // Not used but needed for vertex format
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) dot_view: f32,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(0) @binding(1)
var<uniform> transform: TransformUniform;

@group(0) @binding(2)
var<uniform> lens_glow: LensGlowUniform;

@group(1) @binding(0)
var glow_texture: texture_2d<f32>;

@group(1) @binding(1)
var spectrum_texture: texture_2d<f32>;

@group(1) @binding(2)
var texture_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // For debugging, just render a simple billboard centered at origin
    out.clip_position = camera.projection_matrix * camera.view_matrix * transform.model_matrix * vec4<f32>(input.position, 1.0);
    out.tex_coord = input.tex_coord;
    
    // Always pass dot view test for debugging
    out.dot_view = 1.0;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the glow texture
    let glow_sample = textureSample(glow_texture, texture_sampler, input.tex_coord);
    
    // Map temperature to spectrum texture coordinates
    // Temperature range: 800K to 30000K (like original)
    let u = (lens_glow.temperature - 800.0) / 29200.0;
    let v = 1.0 - glow_sample.r + 0.125;
    
    // Sample temperature-based color from spectrum texture
    let spectrum_color = textureSample(spectrum_texture, texture_sampler, vec2<f32>(u, v));
    
    // Use glow texture red channel as alpha
    let alpha = glow_sample.r;
    
    return vec4<f32>(spectrum_color.rgb, alpha);
}