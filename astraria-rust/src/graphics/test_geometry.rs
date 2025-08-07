/// Test geometry for verifying the rendering pipeline
use crate::generated_shaders::common::{SkyboxVertexInput, VertexInput};

/// Create a simple colored cube for testing
pub fn create_test_cube() -> (Vec<VertexInput>, Vec<u32>) {
    let vertices = vec![
        // Front face
        VertexInput {
            position: glam::Vec3::new(-1.0, -1.0, 1.0),
            tex_coord: glam::Vec2::new(0.0, 0.0),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, -1.0, 1.0),
            tex_coord: glam::Vec2::new(1.0, 0.0),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, 1.0, 1.0),
            tex_coord: glam::Vec2::new(1.0, 1.0),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
        },
        VertexInput {
            position: glam::Vec3::new(-1.0, 1.0, 1.0),
            tex_coord: glam::Vec2::new(0.0, 1.0),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
        },
        // Back face
        VertexInput {
            position: glam::Vec3::new(-1.0, -1.0, -1.0),
            tex_coord: glam::Vec2::new(1.0, 0.0),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
        },
        VertexInput {
            position: glam::Vec3::new(-1.0, 1.0, -1.0),
            tex_coord: glam::Vec2::new(1.0, 1.0),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, 1.0, -1.0),
            tex_coord: glam::Vec2::new(0.0, 1.0),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, -1.0, -1.0),
            tex_coord: glam::Vec2::new(0.0, 0.0),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
        },
        // Top face
        VertexInput {
            position: glam::Vec3::new(-1.0, 1.0, -1.0),
            tex_coord: glam::Vec2::new(0.0, 1.0),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(-1.0, 1.0, 1.0),
            tex_coord: glam::Vec2::new(0.0, 0.0),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, 1.0, 1.0),
            tex_coord: glam::Vec2::new(1.0, 0.0),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, 1.0, -1.0),
            tex_coord: glam::Vec2::new(1.0, 1.0),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
        },
        // Bottom face
        VertexInput {
            position: glam::Vec3::new(-1.0, -1.0, -1.0),
            tex_coord: glam::Vec2::new(1.0, 1.0),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, -1.0, -1.0),
            tex_coord: glam::Vec2::new(0.0, 1.0),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, -1.0, 1.0),
            tex_coord: glam::Vec2::new(0.0, 0.0),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(-1.0, -1.0, 1.0),
            tex_coord: glam::Vec2::new(1.0, 0.0),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
        },
        // Right face
        VertexInput {
            position: glam::Vec3::new(1.0, -1.0, -1.0),
            tex_coord: glam::Vec2::new(1.0, 0.0),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, 1.0, -1.0),
            tex_coord: glam::Vec2::new(1.0, 1.0),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, 1.0, 1.0),
            tex_coord: glam::Vec2::new(0.0, 1.0),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(1.0, -1.0, 1.0),
            tex_coord: glam::Vec2::new(0.0, 0.0),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
        },
        // Left face
        VertexInput {
            position: glam::Vec3::new(-1.0, -1.0, -1.0),
            tex_coord: glam::Vec2::new(0.0, 0.0),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(-1.0, -1.0, 1.0),
            tex_coord: glam::Vec2::new(1.0, 0.0),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(-1.0, 1.0, 1.0),
            tex_coord: glam::Vec2::new(1.0, 1.0),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
        },
        VertexInput {
            position: glam::Vec3::new(-1.0, 1.0, -1.0),
            tex_coord: glam::Vec2::new(0.0, 1.0),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
        },
    ];

    let indices = vec![
        0, 1, 2, 2, 3, 0, // front
        4, 5, 6, 6, 7, 4, // back
        8, 9, 10, 10, 11, 8, // top
        12, 13, 14, 14, 15, 12, // bottom
        16, 17, 18, 18, 19, 16, // right
        20, 21, 22, 22, 23, 20, // left
    ];

    (vertices, indices)
}

/// Create a cube for skybox rendering using the correct SkyboxVertexInput type
pub fn create_skybox_cube() -> (Vec<SkyboxVertexInput>, Vec<u32>) {
    let size = 1.0;

    let vertices = vec![
        // Front face (z = +size) - 2 triangles
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, -size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, -size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, -size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, size, size),
        },
        // Back face (z = -size) - 2 triangles
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, -size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, -size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, -size, -size),
        },
        // Left face (x = -size) - 2 triangles
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, -size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, -size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, -size, size),
        },
        // Right face (x = +size) - 2 triangles
        SkyboxVertexInput {
            position: glam::Vec3::new(size, -size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, -size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, -size, -size),
        },
        // Top face (y = +size) - 2 triangles
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, size, -size),
        },
        // Bottom face (y = -size) - 2 triangles
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, -size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, -size, -size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, -size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(size, -size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, -size, size),
        },
        SkyboxVertexInput {
            position: glam::Vec3::new(-size, -size, -size),
        },
    ];

    // Sequential indices for triangle list
    let indices: Vec<u32> = (0..vertices.len() as u32).collect();
    (vertices, indices)
}

/// Create a simple triangle for initial testing
pub fn create_test_triangle() -> (Vec<VertexInput>, Vec<u32>) {
    let vertices = vec![
        VertexInput {
            position: glam::Vec3::new(0.0, 0.5, 0.0),
            tex_coord: glam::Vec2::new(0.5, 1.0),
            normal: glam::Vec3::Z,
        },
        VertexInput {
            position: glam::Vec3::new(-0.5, -0.5, 0.0),
            tex_coord: glam::Vec2::new(0.0, 0.0),
            normal: glam::Vec3::Z,
        },
        VertexInput {
            position: glam::Vec3::new(0.5, -0.5, 0.0),
            tex_coord: glam::Vec2::new(1.0, 0.0),
            normal: glam::Vec3::Z,
        },
    ];
    let indices = vec![0, 1, 2];
    (vertices, indices)
}

/// Create a UV sphere for spherical objects (planets, stars, etc.)
pub fn create_test_sphere(radius: f32, stacks: u32, slices: u32) -> (Vec<VertexInput>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices
    for i in 0..=stacks {
        let phi = std::f32::consts::PI * i as f32 / stacks as f32;
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();

        for j in 0..=slices {
            let theta = 2.0 * std::f32::consts::PI * j as f32 / slices as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            let x = radius * sin_phi * cos_theta;
            let y = radius * cos_phi;
            let z = radius * sin_phi * sin_theta;

            let u = j as f32 / slices as f32;
            let v = i as f32 / stacks as f32;

            vertices.push(VertexInput {
                position: glam::Vec3::new(x, y, z),
                tex_coord: glam::Vec2::new(u, v),
                normal: glam::Vec3::new(x / radius, y / radius, z / radius), // Normalized position for sphere normals
            });
        }
    }

    // Generate indices
    for i in 0..stacks {
        for j in 0..slices {
            let first = i * (slices + 1) + j;
            let second = first + slices + 1;

            // First triangle
            indices.push(first);
            indices.push(second);
            indices.push(first + 1);

            // Second triangle
            indices.push(second);
            indices.push(second + 1);
            indices.push(first + 1);
        }
    }

    (vertices, indices)
}

/// Create a quad for billboard and UI rendering
pub fn create_test_quad() -> (Vec<VertexInput>, Vec<u32>) {
    let vertices = vec![
        VertexInput {
            position: glam::Vec3::new(-1.0, -1.0, 0.0),
            tex_coord: glam::Vec2::new(0.0, 0.0),
            normal: glam::Vec3::Z,
        },
        VertexInput {
            position: glam::Vec3::new(1.0, -1.0, 0.0),
            tex_coord: glam::Vec2::new(1.0, 0.0),
            normal: glam::Vec3::Z,
        },
        VertexInput {
            position: glam::Vec3::new(1.0, 1.0, 0.0),
            tex_coord: glam::Vec2::new(1.0, 1.0),
            normal: glam::Vec3::Z,
        },
        VertexInput {
            position: glam::Vec3::new(-1.0, 1.0, 0.0),
            tex_coord: glam::Vec2::new(0.0, 1.0),
            normal: glam::Vec3::Z,
        },
    ];
    let indices = vec![0, 1, 2, 2, 3, 0];
    (vertices, indices)
}

/// Create a simple line for orbital path rendering
pub fn create_test_line() -> (Vec<VertexInput>, Vec<u32>) {
    let vertices = vec![
        VertexInput {
            position: glam::Vec3::new(-0.5, -0.5, 0.0),
            tex_coord: glam::Vec2::ZERO,
            normal: glam::Vec3::Z,
        },
        VertexInput {
            position: glam::Vec3::new(0.5, 0.5, 0.0),
            tex_coord: glam::Vec2::ONE,
            normal: glam::Vec3::Z,
        },
    ];
    let indices = vec![0, 1];
    (vertices, indices)
}

/// Create a single point for distant object rendering
pub fn create_test_point() -> (Vec<VertexInput>, Vec<u32>) {
    let vertices = vec![VertexInput {
        position: glam::Vec3::ZERO,
        tex_coord: glam::Vec2::new(0.5, 0.5),
        normal: glam::Vec3::Z,
    }];
    let indices = vec![0];
    (vertices, indices)
}
