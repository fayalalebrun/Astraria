// Sun shader for stellar temperature rendering (800K-30000K)
// Direct port from the original Astraria GLSL sunShader

// Camera uniform (matches core.rs CameraUniform exactly)
struct CameraUniform {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    view_projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
    _padding1: f32,
    camera_direction: vec3<f32>,
    _padding2: f32,
    log_depth_constant: f32,
    far_plane_distance: f32,
    near_plane_distance: f32,
    fc_constant: f32,
};

// Transform uniform
struct TransformUniform {
    model_matrix: mat4x4<f32>,
    model_view_matrix: mat4x4<f32>,
    normal_matrix: mat3x3<f32>,
};

// Sun-specific uniforms
struct SunUniform {
    temperature: f32,           // Star temperature in Kelvin (800-30000)
    _padding1: f32,
    camera_to_sun_direction: vec3<f32>, // Direction from camera to sun
    _padding2: f32,
    sun_position: vec3<f32>,    // Sun position in world coordinates
    _padding3: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) frag_pos: vec3<f32>,
    @location(3) model_view_pos: vec3<f32>,
};

// Bind groups
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> transform: TransformUniform;

@group(2) @binding(0)
var<uniform> sun: SunUniform;

@group(3) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(3) @binding(1)
var sun_gradient_texture: texture_2d<f32>;
@group(3) @binding(2)
var texture_sampler: sampler;

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
    
    // Use logarithmic depth transformation
    let world_position = transform.model_matrix * vec4<f32>(input.position, 1.0);
    out.clip_position = model_to_clip_coordinates(
        world_position,
        camera.view_projection_matrix,
        camera.log_depth_constant,
        camera.far_plane_distance
    );
    out.tex_coords = input.tex_coord;
    out.normal = input.normal;
    out.frag_pos = world_position.xyz;
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