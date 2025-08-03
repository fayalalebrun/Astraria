// Debug version of atmospheric shader - just shows a simple colored sphere
// This tests if the rendering pipeline works at all

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
    normal_matrix: mat3x4<f32>,  // Stored as 3 vec4 for alignment but only use vec3 part
};

struct PointLight {
    position: vec3<f32>,
    _padding1: f32,
    ambient: vec3<f32>,
    _padding2: f32,
    diffuse: vec3<f32>,
    _padding3: f32,
    specular: vec3<f32>,
    _padding4: f32,
};

struct LightingUniform {
    lights: array<PointLight, 8>,
    num_lights: i32,
    _padding: vec3<i32>,
};

struct AtmosphereUniform {
    star_position: vec3<f32>,
    _padding1: f32,
    planet_position: vec3<f32>,
    _padding2: f32,
    atmosphere_color_mod: vec4<f32>,
    overglow: f32,
    use_ambient_texture: i32,
    _padding3: vec2<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) light_pos: vec3<f32>,        // Sun position in camera space
    @location(1) planet_pos: vec3<f32>,       // Planet center in camera space
    @location(2) pixel_pos: vec3<f32>,        // Fragment position in camera space
    @location(3) pixel_normal: vec3<f32>,     // Fragment surface normal
    @location(4) tex_coords: vec2<f32>,       // Texture coordinates
    @location(5) angle_incidence: f32,        // Light incidence angle
    @location(6) atmosphere_color: vec4<f32>, // Computed atmosphere color
    @location(7) light_direction: vec3<f32>,  // Light direction in camera space
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> transform: TransformUniform;

@group(2) @binding(0)
var<uniform> lighting: LightingUniform;
@group(2) @binding(1)
var<uniform> atmosphere: AtmosphereUniform;

@group(3) @binding(0)
var ambient_texture: texture_2d<f32>;
@group(3) @binding(1)
var diffuse_texture: texture_2d<f32>;
@group(3) @binding(2)
var atmosphere_gradient_texture: texture_2d<f32>;
@group(3) @binding(3)
var texture_sampler: sampler;

// Constants matching the original Java implementation
const PI: f32 = 3.14159265;
const TRANSITION_WIDTH: f32 = 0.1;  // How prominent the atmosphere is
const FRESNEL_EXPONENT: f32 = 20.0;

// Step 7: Add point light function from full shader
fn calc_point_light(
    light: PointLight,
    normal: vec3<f32>,
    frag_pos: vec3<f32>,
    view_dir: vec3<f32>,
    tex_coords: vec2<f32>,
    min_diff: ptr<function, f32>
) -> vec4<f32> {
    let light_dir = normalize(light.position - frag_pos);
    
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

// Step 6: Add logarithmic depth buffer transformation (matching Java implementation)
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
    
    // Extract the 3x3 part from the mat3x4 for normal transformation
    let normal_matrix_3x3 = mat3x3<f32>(
        transform.normal_matrix[0].xyz,
        transform.normal_matrix[1].xyz,
        transform.normal_matrix[2].xyz
    );
    
    // Calculate all fields properly like the full shader
    out.light_pos = (camera.view_matrix * vec4<f32>(atmosphere.star_position, 1.0)).xyz;
    out.planet_pos = (camera.view_matrix * vec4<f32>(atmosphere.planet_position, 1.0)).xyz;
    out.pixel_pos = (transform.model_view_matrix * vec4<f32>(input.position, 1.0)).xyz;
    out.pixel_normal = normalize(normal_matrix_3x3 * input.normal);
    out.tex_coords = input.tex_coord;
    out.light_direction = normalize(out.light_pos - out.planet_pos);
    
    // Calculate view direction
    let view_dir = normalize(-out.pixel_pos);
    
    // Calculate light incidence angle (EXACTLY like original Java implementation)
    let dot_prod = clamp(dot(out.light_direction, out.pixel_normal), -1.0, 1.0);
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
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Step 7: Use point light function approach like the full shader
    let norm = normalize(input.pixel_normal);
    let view_dir = normalize(-input.pixel_pos);
    
    var result = vec4<f32>(0.0);
    var min_diff = 1.0;
    
    // Calculate lighting from all point lights using the function approach
    for (var i = 0; i < lighting.num_lights; i++) {
        result += calc_point_light(
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