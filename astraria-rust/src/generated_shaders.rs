// Generated shader modules from wgsl_to_wgpu
// This module includes all the type-safe shader bindings generated at build time

// Include the generated shader modules from the build directory
include!(concat!(env!("OUT_DIR"), "/shaders/mod.rs"));

// Re-export commonly used types from the generated modules
pub mod common {
    // Re-export demangled/consolidated vertex input types
    pub use super::default::VertexInput;

    // Re-export specialized vertex input types
    pub use super::billboard::VertexInput as BillboardVertexInput;
    pub use super::skybox::VertexInput as SkyboxVertexInput;

    // Re-export bind group types for convenience
    pub use super::default::bind_groups::*;
}
