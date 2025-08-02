pub mod billboard_shader;
pub mod black_hole_shader;
/// Shader system - each shader type has its own struct for specialized rendering
/// Based on the Java implementation where each shader type manages its own rendering logic
pub mod default_shader;
pub mod lens_glow_shader;
pub mod line_shader;
pub mod planet_atmo_shader;
pub mod point_shader;
pub mod skybox_shader;
pub mod sun_shader;

pub use billboard_shader::BillboardShader;
pub use black_hole_shader::BlackHoleShader;
pub use default_shader::DefaultShader;
pub use lens_glow_shader::LensGlowShader;
pub use line_shader::LineShader;
pub use planet_atmo_shader::PlanetAtmoShader;
pub use point_shader::PointShader;
pub use skybox_shader::SkyboxShader;
pub use sun_shader::SunShader;

// Legacy compatibility types for the old pipeline system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Default,
    Skybox,
    PlanetAtmosphere,
    Star,
    Billboard,
    LensGlow,
    BlackHole,
    Line,
    Point,
}

// Placeholder ShaderManager for compatibility
pub struct ShaderManager {
    // Placeholder - the new system uses individual shader structs instead
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {}
    }

    // Compatibility methods for the old pipeline system
    pub fn get_shader(&self, _shader_type: ShaderType) -> Option<&wgpu::ShaderModule> {
        None // Placeholder - individual shader structs manage their own modules now
    }

    pub async fn load_shaders(&mut self, _device: &wgpu::Device) -> crate::AstrariaResult<()> {
        Ok(()) // Placeholder - individual shader structs load themselves now
    }

    pub fn get_shader_source(&self, _shader_type: ShaderType) -> Option<&str> {
        None // Placeholder
    }
}
