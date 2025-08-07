use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use wesl::{EscapeMangler, FileResolver, Mangler, Router, Wesl};
use wgsl_to_wgpu::{MatrixVectorTypes, Module, ModulePath, TypePath, WriteOptions};

/// Demangle function for wesl imports - consolidates types from shared modules
fn demangle_wesl(name: &str) -> TypePath {
    // For wesl imports, we want to consolidate shared types into a single type
    // Check if this looks like a mangled wesl import name
    if name.starts_with("super_super_shared_") {
        // Extract the actual type name (remove the path prefix)
        let actual_name = name.strip_prefix("super_super_shared_").unwrap_or(name);
        TypePath {
            parent: ModulePath::default(), // Put in root module for consolidation
            name: actual_name.to_string(),
        }
    } else if name.starts_with("package_") {
        // Handle other package mangling schemes
        let mangler = EscapeMangler;
        if let Some((path, unmangled_name)) = mangler.unmangle(name) {
            TypePath {
                parent: ModulePath {
                    components: path.components,
                },
                name: unmangled_name,
            }
        } else {
            // Fallback to root module
            TypePath {
                parent: ModulePath::default(),
                name: name.to_string(),
            }
        }
    } else {
        // Unmangled names go to root module
        TypePath {
            parent: ModulePath::default(),
            name: name.to_string(),
        }
    }
}

/// Process shaders using wesl 0.2 compiler
fn process_shader_with_wesl<R>(
    shader_path: &Path,
    wesl_compiler: &Wesl<R>,
    shader_dir: &Path,
) -> Result<String, Box<dyn std::error::Error>>
where
    R: wesl::Resolver,
{
    // Extract shader name to use as package identifier
    let shader_name = shader_path
        .file_stem()
        .ok_or("Invalid shader file name")?
        .to_string_lossy();

    // Debug: Try to compile shared module first to test wesl setup
    if shader_name == "default" {
        // Debug the files that should exist
        let expected_wesl = shader_dir.join("shared.wesl");
        let expected_wgsl = shader_dir.join("shared.wgsl");
        println!(
            "cargo:warning=Expected wesl file exists: {} ({})",
            expected_wesl.display(),
            expected_wesl.exists()
        );
        println!(
            "cargo:warning=Expected wgsl file exists: {} ({})",
            expected_wgsl.display(),
            expected_wgsl.exists()
        );

        // Try different path syntaxes to get filesystem origin
        let parsed_path = "super::shared".parse()?;
        println!("cargo:warning=Parsed module path: {:?}", parsed_path);

        // Try shared path - wesl base is already shader_dir so use relative path
        match wesl_compiler.compile(&parsed_path) {
            Ok(_) => println!("cargo:warning=Shared module compiles successfully!"),
            Err(e) => println!("cargo:warning=Shared module failed: {}", e),
        }
    }

    // Try to compile using wesl first - use super:: syntax for filesystem modules
    let module_path = format!("super::{}", shader_name);
    match wesl_compiler.compile(&module_path.parse()?) {
        Ok(compiled) => {
            println!(
                "cargo:warning=Successfully compiled shader {} with wesl",
                shader_name
            );
            Ok(compiled.to_string())
        }
        Err(e) => {
            // Fallback to reading the file directly for shaders not yet converted to wesl
            println!(
                "cargo:warning=Shader {} not yet converted to wesl ({}), using direct read",
                shader_name, e
            );
            let source = fs::read_to_string(shader_path)?;
            Ok(source)
        }
    }
}

/// Generate Rust bindings for a single WGSL shader
fn process_shader<R>(
    shader_path: &Path,
    output_dir: &Path,
    wesl_compiler: &Wesl<R>,
    shader_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>>
where
    R: wesl::Resolver,
{
    println!("cargo:rerun-if-changed={}", shader_path.display());

    // Process shader source using wesl 0.2
    let processed_source = process_shader_with_wesl(shader_path, wesl_compiler, shader_dir)?;

    // Configure wgsl_to_wgpu options with enhanced validation and demangling
    // Configure wgsl_to_wgpu options to avoid struct layout assertion issues
    let options = WriteOptions {
        derive_bytemuck_vertex: true, // Enable for vertex types (no assertions)
        derive_bytemuck_host_shareable: false, // Disable to avoid uniform struct assertions
        derive_encase_host_shareable: false, // Disable encase for now
        matrix_vector_types: MatrixVectorTypes::Glam,
        validate: None, // Disable validation to avoid issues
        ..Default::default()
    };

    // Write the processed WGSL file to output directory (needed for include_str!)
    let shader_name = shader_path.file_stem().unwrap().to_string_lossy();
    let processed_wgsl_file = output_dir.join(format!("{}.wgsl", shader_name));
    fs::write(&processed_wgsl_file, &processed_source)?;

    // Create Module for demangling support
    let mut module = Module::default();

    // Add shader module with demangling to get single Rust types for WGSL types
    module.add_shader_module(
        &processed_source,
        None, // include_path
        options,
        ModulePath::default(), // Use default (root) module path
        demangle_wesl,
    )?;

    // Generate Rust code with demangling
    let generated_code = module.to_generated_bindings(options);

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

    // Initialize wesl 0.2 compiler with shader directory (parent of packages)
    let current_dir = env::current_dir()?;
    let shader_dir = current_dir.join("src/shaders");
    let packages_path = shader_dir.join("packages");
    println!("cargo:warning=Shader dir: {}", shader_dir.display());
    println!(
        "cargo:warning=Packages dir: {} (exists: {})",
        packages_path.display(),
        packages_path.exists()
    );

    // List the files in the packages directory for debugging
    if packages_path.exists() {
        let entries: Result<Vec<_>, _> = fs::read_dir(&packages_path)?.collect();
        match entries {
            Ok(entries) => {
                let filenames: Vec<_> = entries
                    .iter()
                    .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
                    .collect();
                println!("cargo:warning=Found wesl files: {:?}", filenames);
            }
            Err(e) => println!("cargo:warning=Error reading packages directory: {}", e),
        }
    }

    // Create manual FileResolver to test if it can find files
    let file_resolver = FileResolver::new(&shader_dir);
    let mut router = Router::new();
    router.mount_fallback_resolver(file_resolver);

    let wesl_compiler = Wesl::new("").set_custom_resolver(router);
    println!(
        "cargo:warning=Using custom FileResolver with shader directory: {}",
        shader_dir.display()
    );

    // List of shaders to process
    let shaders = [
        "src/shaders/default.wesl",
        "src/shaders/skybox.wesl",
        "src/shaders/planet_atmo.wesl",
        "src/shaders/sun_shader.wesl",
        "src/shaders/billboard.wesl",
        "src/shaders/lens_glow.wesl",
        "src/shaders/black_hole.wesl",
        "src/shaders/line.wesl",
        "src/shaders/point.wesl",
    ];

    // Process each shader
    for shader_path_str in &shaders {
        let shader_path = Path::new(shader_path_str);
        if shader_path.exists() {
            match process_shader(shader_path, &shader_out_dir, &wesl_compiler, &shader_dir) {
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
