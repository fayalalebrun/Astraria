// Default shader package - basic 3D object rendering  
// Uses shared uniform definitions for consistency
import super::shared::{VertexInput};

// For now, inline the needed definitions until wesl import resolution is fixed
struct StandardMVPUniform {
    mvp_matrix: mat4x4<f32>,        // 0-63 (64 bytes)
    camera_position: vec3<f32>,     // 64-75 (12 bytes)
    _padding1: f32,                 // 76-79 (4 bytes padding)  
    camera_direction: vec3<f32>,    // 80-91 (12 bytes)
    _padding2: f32,                 // 92-95 (4 bytes padding)
    log_depth_constant: f32,        // 96-99 (4 bytes)
    far_plane_distance: f32,        // 100-103 (4 bytes)
    near_plane_distance: f32,       // 104-107 (4 bytes)
    fc_constant: f32,               // 108-111 (4 bytes)
    mv_matrix: mat4x4<f32>,         // 112-175 (64 bytes)
};


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
};

// WebGPU-compatible logarithmic depth buffer function
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

// MVP uniform binding
@group(0) @binding(0) var<uniform> mvp: StandardMVPUniform;

// Directional light structure (for astronomical scale lighting)
struct DirectionalLight {
    direction: vec3<f32>,  // Normalized direction from object to light (WORLD SPACE, 12 bytes)
    _padding1: f32,        // Align to 16 bytes (4 bytes, total: 16)
    ambient: vec3<f32>,    // 12 bytes
    _padding2: f32,        // Align to 16 bytes (4 bytes, total: 32) 
    diffuse: vec3<f32>,    // 12 bytes
    _padding3: f32,        // Align to 16 bytes (4 bytes, total: 48)
    specular: vec3<f32>,   // 12 bytes  
    _padding4: f32,        // Align to 16 bytes (4 bytes, total: 64)
}

// Lighting uniforms - properly aligned for WGSL uniform buffer  
struct LightingUniforms {
    lights: array<DirectionalLight, 8>,  // 8 * 64 = 512 bytes
    num_lights: i32,                     // 4 bytes
    // Use vec4<f32> for proper 16-byte alignment in uniform buffer
    _padding: array<vec4<f32>, 10>,      // 10 * 16 = 160 bytes (total 512+4+160=676, round to 688 = 43*16)
}

// Bind groups
// Note: mvp uniform is imported from shared package at @group(0) @binding(0)
@group(1) @binding(0) var<uniform> lighting: LightingUniforms;
@group(2) @binding(0) var diffuse_texture: texture_2d<f32>;
@group(2) @binding(1) var diffuse_sampler: sampler;

// Vertex shader
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Use pre-computed MVP matrix (calculated with 64-bit precision on CPU)
    let vertex_position = vec4<f32>(input.position, 1.0);
    
    // Use vertex position as approximate world position (simplified)
    out.world_position = input.position;
    
    // Use vertex normal as world normal (simplified - no transformation)
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

// Calculate directional light contribution
fn calc_directional_light(light: DirectionalLight, normal: vec3<f32>, frag_pos: vec3<f32>, view_dir: vec3<f32>) -> vec3<f32> {
    // Transform world space light direction to view space
    let normal_matrix = mat3x3<f32>(
        mvp.mv_matrix[0].xyz,
        mvp.mv_matrix[1].xyz,
        mvp.mv_matrix[2].xyz
    );
    let light_dir = normalize(normal_matrix * light.direction);
    
    // Diffuse shading
    let diff = max(dot(normal, light_dir), 0.0);
    
    // Specular shading (Blinn-Phong)
    let halfway_dir = normalize(light_dir + view_dir);
    let spec = pow(max(dot(normal, halfway_dir), 0.0), 32.0);
    
    // Combine results (no attenuation for directional lights)
    let ambient = light.ambient;
    let diffuse = light.diffuse * diff;
    let specular = light.specular * spec;
    
    return ambient + diffuse + specular;
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
        result += calc_directional_light(lighting.lights[i], normal, input.world_position, view_dir);
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