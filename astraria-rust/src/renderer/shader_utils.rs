use anyhow::Result;
/// WGSL shader preprocessing utilities
/// Lightweight preprocessor that handles //!include directives
use std::path::Path;
use std::collections::HashSet;

/// Lightweight WGSL preprocessor that handles //!include directives
pub struct LightweightPreprocessor {
    processed_files: HashSet<std::path::PathBuf>,
}

impl Default for LightweightPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl LightweightPreprocessor {
    pub fn new() -> Self {
        Self {
            processed_files: HashSet::new(),
        }
    }

    pub fn process_shader(&mut self, shader_path: &Path, source: &str) -> Result<String> {
        // Prevent infinite recursion
        let canonical_path = shader_path.canonicalize()
            .unwrap_or_else(|_| shader_path.to_path_buf());
        
        if self.processed_files.contains(&canonical_path) {
            return Ok(String::new()); // Already processed, return empty
        }
        self.processed_files.insert(canonical_path);

        let mut processed = String::new();
        
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("//!include") {
                // Extract the include path
                let include_path = trimmed
                    .strip_prefix("//!include")
                    .ok_or_else(|| anyhow::anyhow!("Invalid include directive: {}", trimmed))?
                    .trim()
                    .trim_matches('"');

                // Resolve the include path relative to current shader
                let base_dir = shader_path.parent().unwrap_or(Path::new("."));
                let mut full_include_path = base_dir.join(include_path);
                
                // If file doesn't exist, try relative to project root
                if !full_include_path.exists() && include_path.starts_with("src/") {
                    full_include_path = Path::new(".").join(include_path);
                }

                // Read and process the included file
                let include_source = std::fs::read_to_string(&full_include_path)
                    .map_err(|e| anyhow::anyhow!("Failed to read include file {:?}: {}", full_include_path, e))?;

                let processed_include = self.process_shader(&full_include_path, &include_source)?;
                processed.push_str(&processed_include);
                processed.push('\n');
            } else {
                processed.push_str(line);
                processed.push('\n');
            }
        }

        Ok(processed)
    }
}

/// Process WGSL shader source with preprocessing
/// Handles //!include directives
pub fn preprocess_wgsl(source: &str, shader_path: &Path) -> Result<String> {
    // For files that don't use includes, just return the source as-is
    if !source.contains("//!include") {
        return Ok(source.to_string());
    }

    // Use our lightweight preprocessor
    let mut preprocessor = LightweightPreprocessor::new();
    preprocessor.process_shader(shader_path, source)
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