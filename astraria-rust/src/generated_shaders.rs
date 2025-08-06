// Generated shader modules from wgsl_to_wgpu
// This module includes all the type-safe shader bindings generated at build time

// Include the generated shader modules from the build directory
include!(concat!(env!("OUT_DIR"), "/shaders/mod.rs"));

// Re-export commonly used types from the generated modules
pub mod common {
    // Re-export uniform types from default shader (they're identical across shaders)
    pub use super::default::{DirectionalLight, LightingUniforms, StandardMVPUniform};

    // Re-export vertex input types
    pub use super::default::VertexInput;
}
