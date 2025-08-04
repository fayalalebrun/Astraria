// Default shader for basic 3D object rendering
// Refactored to use standardized MVP matrix approach with 64-bit precision calculations

// Standardized MVP uniform structure (shared across all shaders)
struct StandardMVPUniform {
    mvp_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
    _padding1: f32,
    camera_direction: vec3<f32>,
    _padding2: f32,
    log_depth_constant: f32,
    far_plane_distance: f32,
    near_plane_distance: f32,
    fc_constant: f32,
};

@group(0) @binding(0)
var<uniform> mvp: StandardMVPUniform;

// Point light structure (matches original Java implementation)
struct PointLight {
    position: vec3<f32>,
    ambient: vec3<f32>,
    diffuse: vec3<f32>,
    specular: vec3<f32>,
}

// Lighting uniforms
struct LightingUniforms {
    lights: array<PointLight, 8>,
    num_lights: i32,
}

// Vertex input
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) normal: vec3<f32>,
}

// Vertex output / Fragment input
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
}

// Bind groups
// Note: StandardMVPUniform is declared above at @group(0) @binding(0)
@group(1) @binding(0) var<uniform> lighting: LightingUniforms;
@group(2) @binding(0) var diffuse_texture: texture_2d<f32>;
@group(2) @binding(1) var diffuse_sampler: sampler;

// WebGPU-compatible logarithmic depth buffer function
// Based on Outerra's optimized implementation for [0,1] depth range
fn model_to_clip_coordinates(
    position: vec4<f32>,
    mvp_matrix: mat4x4<f32>,
    depth_constant: f32,
    far_plane_distance: f32
) -> vec4<f32> {
    var clip = mvp_matrix * position;
    
    // WebGPU logarithmic depth: maps to [0,1] range instead of OpenGL's [-1,1]
    // Formula: z = log2(max(1e-6, 1.0 + w)) * Fcoef
    // where Fcoef = 1.0 / log2(farplane + 1.0) for [0,1] range
    let fcoef = 1.0 / log2(far_plane_distance + 1.0);
    clip.z = log2(max(1e-6, 1.0 + clip.w)) * fcoef * clip.w;
    
    return clip;
}

// Vertex shader
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Use pre-computed MVP matrix (calculated with 64-bit precision on CPU)
    let vertex_position = vec4<f32>(input.position, 1.0);
    
    // For world position, we need to extract just the model transform part
    // Since MVP = P * V * M, we approximate world position for lighting
    // Note: This is a simplification - for precise lighting, we'd need separate model matrix
    out.world_position = input.position; // Approximate for basic lighting
    
    // Transform normal (for basic lighting, use vertex normal as-is)
    // Note: For precise lighting, we'd need a separate normal matrix
    out.world_normal = normalize(input.normal);
    
    // Use logarithmic depth buffer with pre-computed MVP matrix
    out.clip_position = model_to_clip_coordinates(
        vertex_position,
        mvp.mvp_matrix,
        mvp.log_depth_constant,
        mvp.far_plane_distance
    );
    
    out.tex_coord = input.tex_coord;
    
    return out;
}

// Calculate point light contribution (ported from original GLSL)
fn calc_point_light(light: PointLight, normal: vec3<f32>, frag_pos: vec3<f32>, view_dir: vec3<f32>) -> vec3<f32> {
    let light_dir = normalize(light.position - frag_pos);
    
    // Diffuse shading
    let diff = max(dot(normal, light_dir), 0.0);
    
    // Specular shading (Blinn-Phong)
    let halfway_dir = normalize(light_dir + view_dir);
    let spec = pow(max(dot(normal, halfway_dir), 0.0), 32.0);
    
    // Attenuation (basic distance-based for now)
    let distance = length(light.position - frag_pos);
    let attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * distance * distance);
    
    // Combine results
    let ambient = light.ambient;
    let diffuse = light.diffuse * diff;
    let specular = light.specular * spec;
    
    return (ambient + diffuse + specular) * attenuation;
}

// Fragment shader
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample diffuse texture
    let tex_color = textureSample(diffuse_texture, diffuse_sampler, input.tex_coord);
    
    // Normalize inputs
    let normal = normalize(input.world_normal);
    let view_dir = normalize(mvp.camera_position - input.world_position);
    
    // Calculate lighting from all point lights
    var result = vec3<f32>(0.0);
    
    for (var i = 0; i < lighting.num_lights; i++) {
        if (i >= 8) { break; } // Safety check
        result += calc_point_light(lighting.lights[i], normal, input.world_position, view_dir);
    }
    
    // If no lights, use ambient lighting
    if (lighting.num_lights == 0) {
        result = vec3<f32>(0.3, 0.6, 0.9); // Nice blue ambient for testing
    }
    
    // Apply lighting to texture color with some base coloring
    let base_color = vec3<f32>(0.8, 0.4, 0.2); // Orange base color
    let final_color = (tex_color.rgb * base_color) * result;
    
    return vec4<f32>(final_color, tex_color.a);
}