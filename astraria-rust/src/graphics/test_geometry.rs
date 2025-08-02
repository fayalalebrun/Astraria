/// Test geometry for verifying the rendering pipeline
use crate::graphics::Vertex;

/// Create a simple colored cube for testing
pub fn create_test_cube() -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        // Front face
        Vertex {
            position: [-1.0, -1.0, 1.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0, 1.0],
            tex_coord: [1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            tex_coord: [1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0, 1.0],
            tex_coord: [0.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        // Back face
        Vertex {
            position: [-1.0, -1.0, -1.0],
            tex_coord: [1.0, 0.0],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [-1.0, 1.0, -1.0],
            tex_coord: [1.0, 1.0],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0, -1.0],
            tex_coord: [0.0, 1.0],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [1.0, -1.0, -1.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, 0.0, -1.0],
        },
        // Top face
        Vertex {
            position: [-1.0, 1.0, -1.0],
            tex_coord: [0.0, 1.0],
            normal: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [-1.0, 1.0, 1.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            tex_coord: [1.0, 0.0],
            normal: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, -1.0],
            tex_coord: [1.0, 1.0],
            normal: [0.0, 1.0, 0.0],
        },
        // Bottom face
        Vertex {
            position: [-1.0, -1.0, -1.0],
            tex_coord: [1.0, 1.0],
            normal: [0.0, -1.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0, -1.0],
            tex_coord: [0.0, 1.0],
            normal: [0.0, -1.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0, 1.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, -1.0, 0.0],
        },
        Vertex {
            position: [-1.0, -1.0, 1.0],
            tex_coord: [1.0, 0.0],
            normal: [0.0, -1.0, 0.0],
        },
        // Right face
        Vertex {
            position: [1.0, -1.0, -1.0],
            tex_coord: [1.0, 0.0],
            normal: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, -1.0],
            tex_coord: [1.0, 1.0],
            normal: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            tex_coord: [0.0, 1.0],
            normal: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0, 1.0],
            tex_coord: [0.0, 0.0],
            normal: [1.0, 0.0, 0.0],
        },
        // Left face
        Vertex {
            position: [-1.0, -1.0, -1.0],
            tex_coord: [0.0, 0.0],
            normal: [-1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-1.0, -1.0, 1.0],
            tex_coord: [1.0, 0.0],
            normal: [-1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-1.0, 1.0, 1.0],
            tex_coord: [1.0, 1.0],
            normal: [-1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-1.0, 1.0, -1.0],
            tex_coord: [0.0, 1.0],
            normal: [-1.0, 0.0, 0.0],
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

/// Create a simple triangle for initial testing
pub fn create_test_triangle() -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        Vertex {
            position: [0.0, 0.5, 0.0],
            tex_coord: [0.5, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            tex_coord: [0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            tex_coord: [1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
    ];

    let indices = vec![0, 1, 2];

    (vertices, indices)
}

/// Create a skybox cube for background rendering
/// Uses exact vertices from original Astraria skybox implementation
pub fn create_skybox_cube() -> (Vec<[f32; 3]>, Vec<u32>) {
    // Direct port from Astraria Skybox.java - vertices as triangles, not indexed
    let vertices = vec![
        // Face 1 (front)
        [-10.0, 10.0, -10.0],
        [-10.0, -10.0, -10.0],
        [10.0, -10.0, -10.0],
        [10.0, -10.0, -10.0],
        [10.0, 10.0, -10.0],
        [-10.0, 10.0, -10.0],
        // Face 2 (left)
        [-10.0, -10.0, 10.0],
        [-10.0, -10.0, -10.0],
        [-10.0, 10.0, -10.0],
        [-10.0, 10.0, -10.0],
        [-10.0, 10.0, 10.0],
        [-10.0, -10.0, 10.0],
        // Face 3 (right)
        [10.0, -10.0, -10.0],
        [10.0, -10.0, 10.0],
        [10.0, 10.0, 10.0],
        [10.0, 10.0, 10.0],
        [10.0, 10.0, -10.0],
        [10.0, -10.0, -10.0],
        // Face 4 (back)
        [-10.0, -10.0, 10.0],
        [-10.0, 10.0, 10.0],
        [10.0, 10.0, 10.0],
        [10.0, 10.0, 10.0],
        [10.0, -10.0, 10.0],
        [-10.0, -10.0, 10.0],
        // Face 5 (top)
        [-10.0, 10.0, -10.0],
        [10.0, 10.0, -10.0],
        [10.0, 10.0, 10.0],
        [10.0, 10.0, 10.0],
        [-10.0, 10.0, 10.0],
        [-10.0, 10.0, -10.0],
        // Face 6 (bottom)
        [-10.0, -10.0, -10.0],
        [-10.0, -10.0, 10.0],
        [10.0, -10.0, -10.0],
        [10.0, -10.0, -10.0],
        [-10.0, -10.0, 10.0],
        [10.0, -10.0, 10.0],
    ];

    // Sequential indices since we're using triangle list directly
    let indices: Vec<u32> = (0..vertices.len() as u32).collect();

    (vertices, indices)
}
