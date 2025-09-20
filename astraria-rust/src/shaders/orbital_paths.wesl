// Orbital paths shader package for smooth Catmull-Rom spline interpolation
// Uses shared uniform definitions for consistency with the rest of the renderer

import super::shared::{StandardMVPUniform, model_to_clip_coordinates, VertexInput};

struct PathControlPoint {
    position: vec3<f32>,     // Camera-relative position
    timestamp: f32,          // When this position was recorded
    velocity: vec3<f32>,     // Velocity vector
    padding: f32,            // Alignment padding
};

struct PathVertex {
    position: vec3<f32>,     // Interpolated position
    fade_factor: f32,        // 0.0 = fully faded, 1.0 = fully visible
    color: vec4<f32>,        // RGBA color with alpha
};

struct PathComputeUniforms {
    current_time: f32,
    max_trail_time: f32,
    total_segments: u32,
    subdivisions_per_segment: u32,
    camera_position: vec3<f32>,
    padding: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

// Compute shader bindings
@group(0) @binding(0) var<storage, read> control_points: array<PathControlPoint>;
@group(0) @binding(1) var<storage, read_write> output_vertices: array<PathVertex>;
@group(0) @binding(2) var<uniform> compute_uniforms: PathComputeUniforms;

// Render shader bindings (different group to avoid conflict)
@group(1) @binding(0) var<uniform> mvp: StandardMVPUniform;

// Catmull-Rom spline interpolation function
// Takes 4 control points and parameter t (0.0 to 1.0)
fn catmull_rom_interpolate(p0: vec3<f32>, p1: vec3<f32>, p2: vec3<f32>, p3: vec3<f32>, t: f32) -> vec3<f32> {
    let t2 = t * t;
    let t3 = t2 * t;
    
    // Catmull-Rom basis functions
    let a = -0.5 * p0 + 1.5 * p1 - 1.5 * p2 + 0.5 * p3;
    let b = p0 - 2.5 * p1 + 2.0 * p2 - 0.5 * p3;
    let c = -0.5 * p0 + 0.5 * p2;
    let d = p1;
    
    return a * t3 + b * t2 + c * t + d;
}

// Calculate fade factor based on age of the point
fn calculate_fade(age: f32, max_age: f32) -> f32 {
    if (age > max_age) {
        return 0.0;
    }
    
    let normalized_age = age / max_age;
    // Use smooth fade curve - more dramatic fading towards the end
    let fade = 1.0 - normalized_age * normalized_age;
    return max(fade, 0.0);
}

// Convert velocity magnitude to color (blue = slow, red = fast)
fn velocity_to_color(velocity: vec3<f32>) -> vec3<f32> {
    let speed = length(velocity);
    let normalized_speed = min(speed / 50000.0, 1.0); // 50 km/s max for color scale
    
    // Blue (slow) to red (fast) gradient
    let blue = vec3<f32>(0.3, 0.7, 1.0);   // Light blue
    let green = vec3<f32>(0.0, 1.0, 0.0);  // Green
    let red = vec3<f32>(1.0, 0.3, 0.0);    // Orange-red
    
    if (normalized_speed < 0.5) {
        return mix(blue, green, normalized_speed * 2.0);
    } else {
        return mix(green, red, (normalized_speed - 0.5) * 2.0);
    }
}

@compute @workgroup_size(64, 1, 1)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let segment_id = global_id.x;
    
    // Check bounds - need at least 4 control points for Catmull-Rom
    if (segment_id >= compute_uniforms.total_segments || segment_id + 3u >= arrayLength(&control_points)) {
        return;
    }
    
    // Load the 4 control points for this segment
    let p0 = control_points[segment_id];
    let p1 = control_points[segment_id + 1u];
    let p2 = control_points[segment_id + 2u]; 
    let p3 = control_points[segment_id + 3u];
    
    // Generate interpolated vertices for this segment
    let base_output_index = segment_id * compute_uniforms.subdivisions_per_segment;
    
    for (var i = 0u; i < compute_uniforms.subdivisions_per_segment; i++) {
        let output_index = base_output_index + i;
        
        // Skip if we would exceed output buffer bounds
        if (output_index >= arrayLength(&output_vertices)) {
            break;
        }
        
        // Interpolation parameter (0.0 to 1.0 across the segment)
        let t = f32(i) / f32(compute_uniforms.subdivisions_per_segment - 1u);
        
        // Perform Catmull-Rom interpolation
        let interpolated_pos = catmull_rom_interpolate(
            p0.position, 
            p1.position, 
            p2.position, 
            p3.position, 
            t
        );
        
        // Interpolate timestamp for age calculation
        let interpolated_timestamp = mix(p1.timestamp, p2.timestamp, t);
        let age = compute_uniforms.current_time - interpolated_timestamp;
        
        // Calculate fade factor
        let fade = calculate_fade(age, compute_uniforms.max_trail_time);
        
        // Skip points that are completely faded
        if (fade <= 0.001) {
            output_vertices[output_index].position = vec3<f32>(0.0);
            output_vertices[output_index].fade_factor = 0.0;
            output_vertices[output_index].color = vec4<f32>(0.0);
            continue;
        }
        
        // Interpolate velocity for color calculation
        let interpolated_velocity = mix(p1.velocity, p2.velocity, t);
        let base_color = velocity_to_color(interpolated_velocity);
        
        // Apply fade to alpha channel
        let final_color = vec4<f32>(base_color, fade);
        
        // Store the generated vertex
        output_vertices[output_index].position = interpolated_pos;
        output_vertices[output_index].fade_factor = fade;
        output_vertices[output_index].color = final_color;
    }
}

struct PathVertexInput {
    @location(0) position: vec3<f32>,     // Interpolated position
    @location(1) fade_factor: f32,        // Fade factor from compute shader
    @location(2) color: vec4<f32>,        // Color from compute shader
};

@vertex
fn vs_main(input: PathVertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Transform position using standard MVP with logarithmic depth
    out.clip_position = model_to_clip_coordinates(
        vec4<f32>(input.position, 1.0), 
        mvp.mvp_matrix,
        mvp.log_depth_constant,
        mvp.far_plane_distance
    );
    
    // Use color from compute shader, modulated by fade factor
    out.color = vec4<f32>(input.color.rgb, input.color.a * input.fade_factor);
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple pass-through with alpha blending for trail effect
    return input.color;
}