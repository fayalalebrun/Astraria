// Simple default shader compatible with basic uniform structure
// This is a temporary version for testing the new shader architecture

struct CameraUniform {
    view_projection: mat4x4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) normal: vec3<f32>,
};

@group(0) @binding(0) var<uniform> camera: CameraUniform;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    out.clip_position = camera.view_projection * vec4<f32>(input.position, 1.0);
    out.tex_coord = input.tex_coord;
    out.normal = input.normal;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple shading based on normal (for testing)
    let normal = normalize(input.normal);
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diffuse = max(dot(normal, light_dir), 0.2);
    
    return vec4<f32>(diffuse * vec3<f32>(0.8, 0.6, 0.4), 1.0);
}