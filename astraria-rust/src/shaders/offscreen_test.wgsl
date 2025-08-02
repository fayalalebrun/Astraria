// Simple test shader for offscreen rendering

struct CameraUniform {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
};

struct TransformUniform {
    model_matrix: mat4x4<f32>,
};

struct LightingUniform {
    light_position: vec3<f32>,
    _padding: f32,
    light_color: vec3<f32>,
    _padding2: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(0) @binding(1)
var<uniform> transform: TransformUniform;

@group(0) @binding(2)
var<uniform> lighting: LightingUniform;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Transform to world space
    let world_position = transform.model_matrix * vec4<f32>(input.position, 1.0);
    out.world_position = world_position.xyz;
    
    // Transform to clip space
    out.clip_position = camera.projection_matrix * camera.view_matrix * world_position;
    
    // Transform normal
    out.normal = (transform.model_matrix * vec4<f32>(input.normal, 0.0)).xyz;
    
    out.tex_coord = input.tex_coord;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple lighting calculation
    let normal = normalize(input.normal);
    let light_dir = normalize(lighting.light_position - input.world_position);
    let diffuse = max(dot(normal, light_dir), 0.0);
    
    // Create a bright, visible color
    let base_color = vec3<f32>(0.8, 0.4, 0.2); // Orange-ish
    let final_color = base_color * (0.3 + 0.7 * diffuse) * lighting.light_color;
    
    return vec4<f32>(final_color, 1.0);
}