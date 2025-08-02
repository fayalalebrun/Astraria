use std::mem;

// Copy the struct definitions from our shader file to check sizes
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct PointLight {
    position: [f32; 3],
    _padding1: f32,
    ambient: [f32; 3],
    _padding2: f32,
    diffuse: [f32; 3],
    _padding3: f32,
    specular: [f32; 3],
    _padding4: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct LightingUniform {
    lights: [PointLight; 8], // 8 * 64 = 512 bytes
    num_lights: i32,         // 4 bytes
    _padding: [i32; 3],      // 12 bytes (vec3<i32> in WGSL requires 16-byte alignment)
}

// Test alternative struct alignments
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct LightingUniformAligned {
    lights: [PointLight; 8], // 8 * 64 = 512 bytes
    num_lights: i32,         // 4 bytes
    _padding: [i32; 4],      // 16 bytes - try forcing 16-byte alignment with extra field
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct TransformUniformOld {
    model_matrix: [[f32; 4]; 4],      // 64 bytes
    model_view_matrix: [[f32; 4]; 4], // 64 bytes
    normal_matrix: [[f32; 4]; 3],     // 48 bytes (stored as 3 vec4 for alignment)
    _padding: [f32; 4],               // 16 bytes (total = 192 bytes)
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct TransformUniformNew {
    model_matrix: [[f32; 4]; 4],      // 64 bytes
    model_view_matrix: [[f32; 4]; 4], // 64 bytes
    normal_matrix: [[f32; 3]; 3],     // 36 bytes (mat3x3 stored as 3 vec3)
    _padding: [f32; 3],               // 12 bytes for alignment (total = 144 bytes)
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct AtmosphereUniform {
    star_position: [f32; 3],
    _padding1: f32,
    planet_position: [f32; 3],
    _padding2: f32,
    atmosphere_color_mod: [f32; 4],
    overglow: f32,
    use_ambient_texture: i32,
    _padding3: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct LensGlowUniform {
    screen_dimensions: [f32; 2], // Screen width and height  (8 bytes)
    glow_size: [f32; 2],         // Width and height of the glow effect (8 bytes) = 16 bytes so far
    star_position: [f32; 3],     // Star position in world coordinates (12 bytes)
    _padding1: f32,              // (4 bytes) = 16 bytes, total 32 bytes
    camera_direction: [f32; 3],  // Camera forward direction (12 bytes)
    _padding2: f32,              // (4 bytes) = 16 bytes, total 48 bytes
    temperature: f32,            // Star temperature for spectrum mapping (4 bytes)
    _padding3: [f32; 7],         // (28 bytes) = 32 bytes, total 80 bytes
}

fn main() {
    println!("PointLight size: {}", mem::size_of::<PointLight>());
    println!(
        "LightingUniform size: {}",
        mem::size_of::<LightingUniform>()
    );
    println!(
        "LightingUniformAligned size: {}",
        mem::size_of::<LightingUniformAligned>()
    );
    println!(
        "TransformUniformOld size: {}",
        mem::size_of::<TransformUniformOld>()
    );
    println!(
        "TransformUniformNew size: {}",
        mem::size_of::<TransformUniformNew>()
    );
    println!(
        "AtmosphereUniform size: {}",
        mem::size_of::<AtmosphereUniform>()
    );
    println!(
        "LensGlowUniform size: {}",
        mem::size_of::<LensGlowUniform>()
    );

    println!("\nTransform struct size change:");
    println!(
        "Old (mat3x4): {} bytes",
        mem::size_of::<TransformUniformOld>()
    );
    println!(
        "New (mat3x3): {} bytes",
        mem::size_of::<TransformUniformNew>()
    );
    println!(
        "Difference: {} bytes",
        mem::size_of::<TransformUniformOld>() as i32 - mem::size_of::<TransformUniformNew>() as i32
    );
}
