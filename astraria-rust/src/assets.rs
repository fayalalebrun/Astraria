use image::{DynamicImage, GenericImageView};
/// Asset loading and management system
/// Replaces LibGDX AssetManager with Rust-native implementation
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use wgpu::{util::DeviceExt, Buffer, Device, Queue, Texture, TextureView};

use crate::{graphics::Vertex, AstrariaError, AstrariaResult};

pub struct AssetManager {
    textures: HashMap<String, Arc<TextureAsset>>,
    models: HashMap<String, Arc<ModelAsset>>,
    cubemaps: HashMap<String, Arc<CubemapAsset>>,
    // Asset lifecycle tracking
    texture_handles: HashMap<String, Vec<Weak<TextureAsset>>>,
    model_handles: HashMap<String, Vec<Weak<ModelAsset>>>,
    cubemap_handles: HashMap<String, Vec<Weak<CubemapAsset>>>,
}

pub struct TextureAsset {
    pub texture: Texture,
    pub view: TextureView,
    pub width: u32,
    pub height: u32,
}

pub struct CubemapAsset {
    pub texture: Texture,
    pub view: TextureView,
    pub size: u32,
}

pub struct ModelAsset {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub num_indices: u32,
    pub num_vertices: u32,
    pub material_name: Option<String>,
}

impl AssetManager {
    pub async fn new() -> AstrariaResult<Self> {
        Ok(Self {
            textures: HashMap::new(),
            models: HashMap::new(),
            cubemaps: HashMap::new(),
            texture_handles: HashMap::new(),
            model_handles: HashMap::new(),
            cubemap_handles: HashMap::new(),
        })
    }

    /// Load a default white texture for testing
    pub fn create_default_texture(
        &mut self,
        device: &Device,
        queue: &Queue,
    ) -> AstrariaResult<Arc<TextureAsset>> {
        if let Some(texture) = self.textures.get("default_white") {
            return Ok(Arc::clone(texture));
        }

        // Create a simple 1x1 white texture
        let texture_size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Default White Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Write white pixel data
        let white_pixel = [255u8, 255u8, 255u8, 255u8];
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &white_pixel,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let texture_asset = TextureAsset {
            texture,
            view,
            width: 1,
            height: 1,
        };

        let texture_arc = Arc::new(texture_asset);
        self.textures
            .insert("default_white".to_string(), Arc::clone(&texture_arc));
        Ok(texture_arc)
    }

    pub async fn load_texture(
        &mut self,
        device: &Device,
        queue: &Queue,
        path: &str,
    ) -> AstrariaResult<Arc<TextureAsset>> {
        if let Some(texture) = self.textures.get(path) {
            return Ok(Arc::clone(texture));
        }

        // Load image from file
        let img = image::open(path).map_err(|e| {
            AstrariaError::AssetLoading(format!("Failed to load image {}: {}", path, e))
        })?;

        let texture_asset = Self::create_texture_from_image(device, queue, &img, Some(path))?;
        let texture_arc = Arc::new(texture_asset);

        self.textures
            .insert(path.to_string(), Arc::clone(&texture_arc));
        Ok(texture_arc)
    }

    fn create_texture_from_image(
        device: &Device,
        queue: &Queue,
        img: &DynamicImage,
        label: Option<&str>,
    ) -> AstrariaResult<TextureAsset> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(TextureAsset {
            texture,
            view,
            width: dimensions.0,
            height: dimensions.1,
        })
    }

    pub async fn load_scenario(&self, path: &str) -> AstrariaResult<String> {
        use std::fs;

        // Try to load from assets/examples/ directory first
        let full_path = format!("assets/examples/{}", path);

        match fs::read_to_string(&full_path) {
            Ok(content) => {
                log::info!("Loaded scenario file: {}", full_path);
                Ok(content)
            }
            Err(_) => {
                // Try direct path if not found in examples
                match fs::read_to_string(path) {
                    Ok(content) => {
                        log::info!("Loaded scenario file: {}", path);
                        Ok(content)
                    }
                    Err(e) => {
                        log::error!("Failed to load scenario file '{}': {}", path, e);
                        Err(crate::AstrariaError::AssetLoading(format!(
                            "Failed to load scenario file '{}': {}",
                            path, e
                        )))
                    }
                }
            }
        }
    }

    pub async fn load_model(
        &mut self,
        device: &Device,
        path: &str,
    ) -> AstrariaResult<Arc<ModelAsset>> {
        if let Some(model) = self.models.get(path) {
            return Ok(Arc::clone(model));
        }

        log::info!("Loading OBJ model: {}", path);

        // Load OBJ file using tobj
        let (models, _materials) = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ignore_points: true,
                ignore_lines: true,
            },
        )
        .map_err(|e| AstrariaError::AssetLoading(format!("Failed to load OBJ {}: {}", path, e)))?;

        if models.is_empty() {
            return Err(AstrariaError::AssetLoading(format!(
                "No models found in OBJ file: {}",
                path
            )));
        }

        // Use the first model for now (TODO: support multiple meshes)
        let model = &models[0];
        let mesh = &model.mesh;

        // Convert to our vertex format using indices
        let mut vertices = Vec::new();
        let positions = &mesh.positions;
        let normals = &mesh.normals;
        let texcoords = &mesh.texcoords;

        // Build vertices based on the indices
        for &index in &mesh.indices {
            let pos_idx = (index as usize) * 3;
            let tex_idx = (index as usize) * 2;

            let vertex = Vertex {
                position: if pos_idx + 2 < positions.len() {
                    [
                        positions[pos_idx],
                        positions[pos_idx + 1],
                        positions[pos_idx + 2],
                    ]
                } else {
                    [0.0, 0.0, 0.0]
                },
                tex_coord: if tex_idx + 1 < texcoords.len() {
                    [texcoords[tex_idx], texcoords[tex_idx + 1]]
                } else {
                    [0.0, 0.0]
                },
                normal: if pos_idx + 2 < normals.len() {
                    [normals[pos_idx], normals[pos_idx + 1], normals[pos_idx + 2]]
                } else {
                    [0.0, 1.0, 0.0] // Default up normal
                },
            };
            vertices.push(vertex);
        }

        // Create simple sequential indices since we've already expanded vertices
        let indices: Vec<u32> = (0..vertices.len() as u32).collect();

        // Create vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Vertex Buffer", path)),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create index buffer with our sequential indices
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Index Buffer", path)),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let model_asset = ModelAsset {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
            num_vertices: vertices.len() as u32,
            material_name: model.mesh.material_id.map(|id| format!("material_{}", id)),
        };

        log::info!(
            "Loaded OBJ model: {} vertices, {} indices",
            model_asset.num_vertices,
            model_asset.num_indices
        );

        let model_arc = Arc::new(model_asset);
        self.models.insert(path.to_string(), Arc::clone(&model_arc));
        Ok(model_arc)
    }

    /// Get a loaded model by path
    pub fn get_model(&self, path: &str) -> Option<&ModelAsset> {
        self.models.get(path).map(|arc| arc.as_ref())
    }

    /// Get a shared reference to a loaded model
    pub fn get_model_handle(&self, path: &str) -> Option<Arc<ModelAsset>> {
        self.models.get(path).map(Arc::clone)
    }

    /// Get a shared reference to a loaded texture
    pub fn get_texture_handle(&self, path: &str) -> Option<Arc<TextureAsset>> {
        self.textures.get(path).map(Arc::clone)
    }

    /// Clean up unused assets (remove assets with no active handles)
    pub fn cleanup_unused_assets(&mut self) {
        // Clean up unused textures
        let mut unused_textures = Vec::new();
        for (path, texture_arc) in &self.textures {
            if Arc::strong_count(texture_arc) == 1 {
                unused_textures.push(path.clone());
            }
        }
        for path in unused_textures {
            self.textures.remove(&path);
            self.texture_handles.remove(&path);
            log::debug!("Cleaned up unused texture: {}", path);
        }

        // Clean up unused models
        let mut unused_models = Vec::new();
        for (path, model_arc) in &self.models {
            if Arc::strong_count(model_arc) == 1 {
                unused_models.push(path.clone());
            }
        }
        for path in unused_models {
            self.models.remove(&path);
            self.model_handles.remove(&path);
            log::debug!("Cleaned up unused model: {}", path);
        }

        // Clean up unused cubemaps
        let mut unused_cubemaps = Vec::new();
        for (name, cubemap_arc) in &self.cubemaps {
            if Arc::strong_count(cubemap_arc) == 1 {
                unused_cubemaps.push(name.clone());
            }
        }
        for name in unused_cubemaps {
            self.cubemaps.remove(&name);
            self.cubemap_handles.remove(&name);
            log::debug!("Cleaned up unused cubemap: {}", name);
        }
    }

    /// Load a cubemap from 6 face images
    pub async fn load_cubemap(
        &mut self,
        device: &Device,
        queue: &Queue,
        name: &str,
        face_paths: &[&str; 6], // +X, -X, +Y, -Y, +Z, -Z
    ) -> AstrariaResult<Arc<CubemapAsset>> {
        if let Some(cubemap) = self.cubemaps.get(name) {
            return Ok(Arc::clone(cubemap));
        }

        // Load all 6 face images
        let mut face_images = Vec::with_capacity(6);
        let mut cubemap_size = 0u32;

        for (i, path) in face_paths.iter().enumerate() {
            let img = image::open(path).map_err(|e| {
                AstrariaError::AssetLoading(format!("Failed to load cubemap face {}: {}", path, e))
            })?;

            let rgba = img.to_rgba8();
            let (width, height) = img.dimensions();

            // Ensure all faces are square and same size
            if width != height {
                return Err(AstrariaError::AssetLoading(format!(
                    "Cubemap face {} is not square: {}x{}",
                    path, width, height
                )));
            }

            if i == 0 {
                cubemap_size = width;
            } else if width != cubemap_size {
                return Err(AstrariaError::AssetLoading(format!(
                    "Cubemap face {} size mismatch: expected {}, got {}",
                    path, cubemap_size, width
                )));
            }

            face_images.push(rgba);
        }

        // Create cubemap texture
        let texture_size = wgpu::Extent3d {
            width: cubemap_size,
            height: cubemap_size,
            depth_or_array_layers: 6,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("Cubemap: {}", name)),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Upload each face
        for (i, face_data) in face_images.iter().enumerate() {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: i as u32,
                    },
                },
                face_data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * cubemap_size),
                    rows_per_image: Some(cubemap_size),
                },
                wgpu::Extent3d {
                    width: cubemap_size,
                    height: cubemap_size,
                    depth_or_array_layers: 1,
                },
            );
        }

        // Create cube texture view
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("Cubemap View: {}", name)),
            format: None,
            dimension: Some(wgpu::TextureViewDimension::Cube),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
            usage: Some(wgpu::TextureUsages::TEXTURE_BINDING),
        });

        let cubemap_asset = CubemapAsset {
            texture,
            view,
            size: cubemap_size,
        };

        let cubemap_arc = Arc::new(cubemap_asset);
        self.cubemaps
            .insert(name.to_string(), Arc::clone(&cubemap_arc));

        log::info!("Loaded cubemap '{}' with size {}", name, cubemap_size);
        Ok(cubemap_arc)
    }

    /// Get a loaded cubemap by name
    pub fn get_cubemap(&self, name: &str) -> Option<&CubemapAsset> {
        self.cubemaps.get(name).map(|arc| arc.as_ref())
    }

    /// Get a shared reference to a loaded cubemap
    pub fn get_cubemap_handle(&self, name: &str) -> Option<Arc<CubemapAsset>> {
        self.cubemaps.get(name).map(Arc::clone)
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize, usize) {
        (self.textures.len(), self.models.len(), self.cubemaps.len())
    }
}
