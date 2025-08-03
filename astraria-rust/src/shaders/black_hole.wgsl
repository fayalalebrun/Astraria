// Black hole shader with gravitational lensing effects
// Direct port from the original Astraria GLSL blackHole shaders

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

struct TransformUniform {
    model_matrix: mat4x4<f32>,
    model_view_matrix: mat4x4<f32>,
    normal_matrix: mat3x4<f32>,  // mat3x4 for proper alignment (3 vec4s = 48 bytes)
    _padding: vec4<f32>,
};

struct BlackHoleUniform {
    hole_position: vec3<f32>,    // Black hole position in world coordinates
    _padding: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) model_view_position: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> transform: TransformUniform;

@group(1) @binding(1)
var<uniform> black_hole: BlackHoleUniform;

@group(2) @binding(0)
var skybox_texture: texture_cube<f32>;

@group(2) @binding(1)
var texture_sampler: sampler;

// Logarithmic depth buffer function (from original shader)
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
    
    // Use logarithmic depth buffer for astronomical scale support
    let world_position = transform.model_matrix * vec4<f32>(input.position, 1.0);
    out.clip_position = model_to_clip_coordinates(
        world_position,
        camera.view_projection_matrix,
        camera.log_depth_constant,
        camera.far_plane_distance
    );
    out.tex_coord = input.tex_coord;
    out.normal = input.normal;
    out.world_position = world_position.xyz;
    out.model_view_position = input.position;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate distance from center for black hole effect
    let center_distance = length(input.tex_coord - vec2<f32>(0.5, 0.5));
    
    // For gravitational lensing effect, sample skybox with distorted coordinates
    let distortion = (0.5 - center_distance) * 2.0;
    let refracted = normalize(input.normal + vec3<f32>(distortion));
    let skybox_color = textureSample(skybox_texture, texture_sampler, refracted).rgb;
    
    // Create black hole: very dark in center, lighter towards edges
    var final_color = skybox_color;
    if (center_distance < 0.2) {
        final_color = vec3<f32>(0.0, 0.0, 0.0); // Pure black for event horizon
    } else {
        // Apply lensing effect - brighten around the edge
        let lensing_factor = smoothstep(0.2, 0.5, center_distance);
        final_color = skybox_color * lensing_factor;
    }
    
    return vec4<f32>(final_color, 1.0);
}