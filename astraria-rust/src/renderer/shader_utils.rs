use anyhow::Result;
/// WGSL shader preprocessing utilities
/// Uses wgsl_preprocessor crate to handle #include directives and other preprocessing
use std::path::Path;

/// Process WGSL shader source with preprocessing
/// Handles #include directives and other preprocessing features
pub fn preprocess_wgsl(source: &str, shader_path: &Path) -> Result<String> {
    // For files that don't use includes, just return the source as-is
    if !source.contains("#include") && !source.contains("//!include") {
        return Ok(source.to_string());
    }

    // Use wgsl_preprocessor to handle includes
    let shader_path_str = shader_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid shader path"))?;

    let shader_builder = wgsl_preprocessor::ShaderBuilder::new(shader_path_str)
        .map_err(|e| anyhow::anyhow!("Failed to create ShaderBuilder: {}", e))?;
    let shader_desc = shader_builder.build();

    // Convert ShaderSource to String
    let source_str = match shader_desc.source {
        wgpu::ShaderSource::Wgsl(cow) => cow.into_owned(),
        _ => return Err(anyhow::anyhow!("Unsupported shader source type")),
    };

    Ok(source_str)
}

/// Load and preprocess a WGSL shader file
pub fn load_preprocessed_wgsl(shader_path: &Path) -> Result<String> {
    let source = std::fs::read_to_string(shader_path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to read shader file {}: {}",
            shader_path.display(),
            e
        )
    })?;

    preprocess_wgsl(&source, shader_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_preprocess_simple() {
        let source = r#"
// Simple shader without includes
struct Test {
    value: f32,
};
"#;
        let path = PathBuf::from("test.wgsl");
        let result = preprocess_wgsl(source, &path).unwrap();
        assert!(result.contains("struct Test"));
    }
}
