use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use wgsl_to_wgpu::{MatrixVectorTypes, WriteOptions, create_shader_module};

/// Process //!include directives in WGSL files
fn preprocess_wgsl_includes(
    source: &str,
    shader_path: &Path,
    processed: &mut HashSet<PathBuf>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = String::new();
    let shader_dir = shader_path.parent().unwrap_or(Path::new(""));

    for line in source.lines() {
        if let Some(include_path) = line.strip_prefix("//!include ") {
            // Resolve include path relative to shader directory
            let include_file = if include_path.starts_with("src/") {
                // Absolute path from project root
                PathBuf::from(include_path)
            } else {
                // Relative path
                shader_dir.join(include_path)
            };

            let canonical_path = include_file
                .canonicalize()
                .unwrap_or_else(|_| include_file.clone());

            // Prevent circular includes
            if processed.contains(&canonical_path) {
                continue;
            }
            processed.insert(canonical_path.clone());

            // Read and recursively process the included file
            let include_content = fs::read_to_string(&canonical_path).map_err(|e| {
                format!(
                    "Failed to read include file '{}': {}",
                    canonical_path.display(),
                    e
                )
            })?;

            let processed_include =
                preprocess_wgsl_includes(&include_content, &canonical_path, processed)?;
            result.push_str(&processed_include);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    Ok(result)
}

/// Generate Rust bindings for a single WGSL shader
fn process_shader(shader_path: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={}", shader_path.display());

    // Read and preprocess the shader source
    let source = fs::read_to_string(shader_path)?;
    let mut processed_files = HashSet::new();
    let processed_source = preprocess_wgsl_includes(&source, shader_path, &mut processed_files)?;

    // Configure wgsl_to_wgpu options
    let options = WriteOptions {
        derive_bytemuck_vertex: true,
        derive_bytemuck_host_shareable: true,
        matrix_vector_types: MatrixVectorTypes::Glam,
        ..Default::default()
    };

    // Write the processed WGSL file to output directory (needed for include_str!)
    let shader_name = shader_path.file_stem().unwrap().to_string_lossy();
    let processed_wgsl_file = output_dir.join(format!("{}.wgsl", shader_name));
    fs::write(&processed_wgsl_file, &processed_source)?;

    // Generate Rust code
    let generated_code =
        create_shader_module(&processed_source, &format!("{}.wgsl", shader_name), options)?;

    // Write generated code to output file
    let output_file = output_dir.join(format!("{}.rs", shader_name));
    let mut file = File::create(&output_file)?;
    file.write_all(generated_code.as_bytes())?;

    println!(
        "Generated shader bindings: {} -> {}",
        shader_path.display(),
        output_file.display()
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/shaders");
    println!("cargo:rerun-if-changed=src/renderer/uniforms");

    let out_dir = env::var("OUT_DIR")?;
    let shader_out_dir = Path::new(&out_dir).join("shaders");

    // Create output directory
    fs::create_dir_all(&shader_out_dir)?;

    // List of shaders to process
    let shaders = [
        "src/shaders/default.wgsl",
        "src/shaders/skybox.wgsl",
        "src/shaders/planet_atmo.wgsl",
        "src/shaders/sun_shader.wgsl",
        "src/shaders/billboard.wgsl",
        "src/shaders/lens_glow.wgsl",
        "src/shaders/black_hole.wgsl",
        "src/shaders/line.wgsl",
        "src/shaders/point.wgsl",
    ];

    // Process each shader
    for shader_path_str in &shaders {
        let shader_path = Path::new(shader_path_str);
        if shader_path.exists() {
            match process_shader(shader_path, &shader_out_dir) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to process shader {}: {}",
                        shader_path.display(),
                        e
                    );
                    // Continue processing other shaders
                }
            }
        } else {
            eprintln!("Warning: Shader file not found: {}", shader_path.display());
        }
    }

    // Generate a mod.rs file to include all generated shaders
    let mod_file = shader_out_dir.join("mod.rs");
    let mut mod_content = String::from("// Generated shader modules\n\n");

    for shader_path_str in &shaders {
        let shader_path = Path::new(shader_path_str);
        if shader_path.exists() {
            let shader_name = shader_path.file_stem().unwrap().to_string_lossy();
            mod_content.push_str(&format!("pub mod {};\n", shader_name));
        }
    }

    fs::write(mod_file, mod_content)?;

    println!("Shader processing complete. Generated modules available in $OUT_DIR/shaders/");
    Ok(())
}
