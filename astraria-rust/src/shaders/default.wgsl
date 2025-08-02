// Default shader for basic 3D object rendering
// Ported from the original GLSL default shader with logarithmic depth buffer support

// Camera uniforms
struct CameraUniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    view_projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
    camera_direction: vec3<f32>,
    log_depth_constant: f32,
    far_plane_distance: f32,
    near_plane_distance: f32,
}

// Transform uniforms
struct TransformUniforms {
    model_matrix: mat4x4<f32>,
    normal_matrix: mat3x3<f32>,
}

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
@group(0) @binding(0) var<uniform> camera: CameraUniforms;
@group(1) @binding(0) var<uniform> transform: TransformUniforms;
@group(2) @binding(0) var<uniform> lighting: LightingUniforms;
@group(3) @binding(0) var diffuse_texture: texture_2d<f32>;
@group(3) @binding(1) var diffuse_sampler: sampler;

// Logarithmic depth buffer function (ported from original GLSL)
fn model_to_clip_coordinates(
    position: vec4<f32>,
    mvp_matrix: mat4x4<f32>,
    depth_constant: f32,
    far_plane_distance: f32
) -> vec4<f32> {
    var clip = mvp_matrix * position;
    clip.z = ((2.0 * log(depth_constant * clip.z + 1.0) / 
              log(depth_constant * far_plane_distance + 1.0)) - 1.0) * clip.w;
    return clip;
}

// Vertex shader
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Transform to world space
    let world_position = transform.model_matrix * vec4<f32>(input.position, 1.0);
    out.world_position = world_position.xyz;
    
    // Transform normal to world space
    out.world_normal = normalize(transform.normal_matrix * input.normal);
    
    // For testing, use simple transformation without logarithmic depth
    out.clip_position = camera.view_projection_matrix * world_position;
    
    // TODO: Re-enable logarithmic depth transformation later
    // out.clip_position = model_to_clip_coordinates(
    //     world_position,
    //     camera.view_projection_matrix,
    //     camera.log_depth_constant,
    //     camera.far_plane_distance
    // );
    
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
    let view_dir = normalize(camera.camera_position - input.world_position);
    
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