// Atmospheric planet shader package with Fresnel effects and scattering
// Uses shared uniform definitions for consistency

import super::shared::{StandardMVPUniform, VertexInput, model_to_clip_coordinates};

struct DirectionalLight {
    direction: vec3<f32>,  // Normalized direction from object to light (WORLD SPACE)
    _padding1: f32,
    ambient: vec3<f32>,
    _padding2: f32,
    diffuse: vec3<f32>,
    _padding3: f32,
    specular: vec3<f32>,
    _padding4: f32,
};

struct LightingUniform {
    lights: array<DirectionalLight, 8>,  // 512 bytes
    num_lights: i32,                     // 4 bytes
    _padding: array<vec4<f32>, 16>,      // 256 bytes (should be more than enough)
};

struct AtmosphereUniform {
    atmosphere_color_mod: vec4<f32>,
    overglow: f32,
    use_ambient_texture: i32,
    _padding: vec2<f32>,
};

// Custom vertex output for atmospheric effects (extends base VertexOutput)
struct AtmosphereVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) pixel_pos: vec3<f32>,        // Fragment position in camera space
    @location(1) pixel_normal: vec3<f32>,     // Fragment surface normal
    @location(2) tex_coords: vec2<f32>,       // Texture coordinates
    @location(3) angle_incidence: f32,        // Light incidence angle
    @location(4) atmosphere_color: vec4<f32>, // Computed atmosphere color
};

@group(0) @binding(0) var<uniform> mvp: StandardMVPUniform;

@group(1) @binding(0) var<uniform> lighting: LightingUniform;
@group(1) @binding(1) var<uniform> atmosphere: AtmosphereUniform;

@group(2) @binding(0) var ambient_texture: texture_2d<f32>;
@group(2) @binding(1) var diffuse_texture: texture_2d<f32>;
@group(2) @binding(2) var atmosphere_gradient_texture: texture_2d<f32>;
@group(2) @binding(3) var texture_sampler: sampler;

// Constants matching the original Java implementation
const PI: f32 = 3.14159265;
const TRANSITION_WIDTH: f32 = 0.1;  // How prominent the atmosphere is
const FRESNEL_EXPONENT: f32 = 20.0;

// Calculate directional light contribution
fn calc_directional_light(
    light: DirectionalLight,
    normal: vec3<f32>,
    frag_pos: vec3<f32>,
    view_dir: vec3<f32>,
    tex_coords: vec2<f32>,
    min_diff: ptr<function, f32>
) -> vec4<f32> {
    // Transform world space light direction to view space
    // Use the same normal matrix approach as in the vertex shader
    let normal_matrix = mat3x3<f32>(
        mvp.mv_matrix[0].xyz,
        mvp.mv_matrix[1].xyz,
        mvp.mv_matrix[2].xyz
    );
    let light_dir = normalize(normal_matrix * light.direction);
    
    // Diffuse shading
    let diff_before = dot(normal, light_dir);
    let diff = max(diff_before, 0.0);
    
    // Track minimum diffuse for ambient texture blending
    if (diff_before < *min_diff) {
        *min_diff = diff_before;
    }
    
    // Sample textures
    let diffuse_sample = textureSample(diffuse_texture, texture_sampler, tex_coords);
    
    // Combine lighting components
    let ambient = vec4<f32>(light.ambient, 1.0) * diffuse_sample;
    let diffuse_contrib = vec4<f32>(light.diffuse, 1.0) * diff * diffuse_sample;
    
    return ambient + diffuse_contrib;
}

@vertex
fn vs_main(input: VertexInput) -> AtmosphereVertexOutput {
    var out: AtmosphereVertexOutput;
    
    // Use pre-computed MVP matrix (calculated with 64-bit precision on CPU)
    let vertex_position = vec4<f32>(input.position, 1.0);
    out.clip_position = model_to_clip_coordinates(
        vertex_position,
        mvp.mvp_matrix,
        mvp.log_depth_constant,
        mvp.far_plane_distance
    );
    
    // Transform vertex to view space (camera space) - matching Java implementation
    // In view space, the camera is at origin (0,0,0)
    out.pixel_pos = (mvp.mv_matrix * vertex_position).xyz;
    
    // Transform normal to view space using normal matrix (transpose of inverse of modelView)
    // For now, assuming uniform scale and no shear, we can use the upper 3x3 of modelView
    let normal_matrix = mat3x3<f32>(
        mvp.mv_matrix[0].xyz,
        mvp.mv_matrix[1].xyz,
        mvp.mv_matrix[2].xyz
    );
    out.pixel_normal = normalize(normal_matrix * input.normal);
    out.tex_coords = input.tex_coord;
    
    // Calculate view direction in camera-relative space
    let view_dir = normalize(-out.pixel_pos);
    
    // Transform world space light direction to view space for atmospheric calculations
    let light_direction_view = normalize(normal_matrix * lighting.lights[0].direction);
    
    // Calculate light incidence angle (EXACTLY like original Java implementation)
    let dot_prod = clamp(dot(light_direction_view, out.pixel_normal), -1.0, 1.0);
    out.angle_incidence = acos(dot_prod) / PI;
    
    // Calculate atmospheric shading factor (terminator transition) - EXACTLY like original
    let shade_factor = 0.1 * (1.0 - out.angle_incidence) + 
                      0.9 * (1.0 - (clamp(out.angle_incidence, 0.5, 0.5 + TRANSITION_WIDTH) - 0.5) / TRANSITION_WIDTH);
    
    // Calculate Fresnel effect (atmosphere visibility from viewing angle) - EXACTLY like original
    let dot_prod2 = clamp(dot(out.pixel_normal, view_dir), -1.0, 1.0);
    let angle_to_viewer = sin(acos(dot_prod2));
    let perspective_factor = 0.3 + 
                           0.2 * pow(angle_to_viewer, FRESNEL_EXPONENT) + 
                           0.5 * pow(angle_to_viewer, FRESNEL_EXPONENT * 20.0);
    
    // Combine atmospheric effects - EXACTLY like original
    out.atmosphere_color = vec4<f32>(perspective_factor * shade_factor);
    
    return out;
}

@fragment
fn fs_main(input: AtmosphereVertexOutput) -> @location(0) vec4<f32> {
    // Step 7: Use point light function approach like the full shader
    let norm = normalize(input.pixel_normal);
    let view_dir = normalize(-input.pixel_pos);
    
    var result = vec4<f32>(0.0);
    var min_diff = 1.0;
    
    // Calculate lighting from all directional lights
    for (var i = 0; i < lighting.num_lights; i++) {
        result += calc_directional_light(
            lighting.lights[i],
            norm,
            input.pixel_pos,
            view_dir,
            input.tex_coords,
            &min_diff
        );
    }
    
    // Apply ambient texture blending for shadowed areas (from full shader)
    let ambient_sample = textureSample(ambient_texture, texture_sampler, input.tex_coords);
    
    if (min_diff < 0.0 && atmosphere.use_ambient_texture == 1) {
        var adjusted_min_diff = min_diff;
        if (adjusted_min_diff < -0.25) {
            adjusted_min_diff = -0.25;
        }
        
        adjusted_min_diff = adjusted_min_diff * -4.0;
        adjusted_min_diff = 1.0 - adjusted_min_diff;
        
        result = mix(ambient_sample, result, adjusted_min_diff);
    }
    
    let surface_color = result.xyz;
    
    // Final atmospheric blending - EXACTLY like original Java implementation
    
    // Sample atmosphere gradient based on light incidence angle
    let gradient_coords = vec2<f32>(input.angle_incidence, 0.5);
    let atmosphere_gradient = textureSample(atmosphere_gradient_texture, texture_sampler, gradient_coords);
    
    // Calculate final atmosphere contribution exactly like the original
    var atmosphere_contrib = input.atmosphere_color * atmosphere_gradient * 1.4;
    atmosphere_contrib = atmosphere_contrib * atmosphere.atmosphere_color_mod;
    
    // Blend atmosphere with surface color using alpha blending (exactly like original)
    let final_color = atmosphere_contrib.a * atmosphere_contrib.rgb + 
                     (1.0 - atmosphere_contrib.a) * surface_color;
    
    return vec4<f32>(final_color, 1.0);
}