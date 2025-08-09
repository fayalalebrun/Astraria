use glam::{DMat4, DVec3, Vec2, Vec3, Vec4Swizzles};
/// Simplified occlusion query system that follows wgpu best practices
use std::collections::HashMap;
use wgpu::{self, util::DeviceExt};

use crate::renderer::shaders::OcclusionProxyShader;

/// Maximum number of simultaneous occlusion queries - keep this small!
const MAX_QUERIES: u32 = 32; // Much smaller than before

/// Unique identifier for stars in the occlusion system
pub type StarId = u32;

/// Simple occlusion test result
#[derive(Debug)]
struct OcclusionTest {
    query_index: u32,
    star_id: StarId,
    world_position: DVec3,
    frame_submitted: u64,
    is_visible: bool, // Simple boolean result
}

/// Simplified occlusion query system
pub struct OcclusionSystem {
    /// GPU query set for occlusion testing
    query_set: wgpu::QuerySet,

    // Note: No query result buffer needed - we avoid reading back to prevent snatch lock issues
    /// Proxy shader system for invisible geometry rendering
    proxy_shader: OcclusionProxyShader,

    /// Active occlusion tests
    active_tests: HashMap<StarId, OcclusionTest>,

    /// Pool of available query indices
    available_indices: Vec<u32>,

    /// Current frame number
    current_frame: u64,

    /// Proxy geometry buffers
    proxy_vertex_buffer: wgpu::Buffer,
    proxy_index_buffer: wgpu::Buffer,

    /// Track which queries were actually used this frame
    queries_used_this_frame: Vec<u32>,
}

/// Vertex data for proxy geometry
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ProxyVertex {
    position: [f32; 3],
}

impl OcclusionSystem {
    /// Create a new simplified occlusion system
    pub fn new(
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        log::info!(
            "Creating simplified occlusion system with {} max queries",
            MAX_QUERIES
        );

        // Create query set for occlusion testing
        let query_set = device.create_query_set(&wgpu::QuerySetDescriptor {
            label: Some("Simple Occlusion Query Set"),
            ty: wgpu::QueryType::Occlusion,
            count: MAX_QUERIES,
        });

        // Note: No query result buffer needed - we don't read back results to avoid snatch lock issues

        // Create proxy geometry (single quad)
        let proxy_vertices = [
            ProxyVertex {
                position: [-1.0, -1.0, 0.0],
            },
            ProxyVertex {
                position: [1.0, -1.0, 0.0],
            },
            ProxyVertex {
                position: [1.0, 1.0, 0.0],
            },
            ProxyVertex {
                position: [-1.0, 1.0, 0.0],
            },
        ];

        let proxy_indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let proxy_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Simple Proxy Vertex Buffer"),
            contents: bytemuck::cast_slice(&proxy_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let proxy_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Simple Proxy Index Buffer"),
            contents: bytemuck::cast_slice(&proxy_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Initialize available query indices pool
        let available_indices: Vec<u32> = (0..MAX_QUERIES).collect();

        // Create proxy shader system
        let proxy_shader = OcclusionProxyShader::new(device, wgpu::TextureFormat::Bgra8UnormSrgb)?;

        log::info!("Simple occlusion system initialization complete");
        Ok(Self {
            query_set,
            proxy_shader,
            active_tests: HashMap::new(),
            available_indices,
            current_frame: 0,
            proxy_vertex_buffer,
            proxy_index_buffer,
            queries_used_this_frame: Vec::new(),
        })
    }

    /// Queue a star for occlusion testing
    pub fn test_star_occlusion(
        &mut self,
        star_id: StarId,
        world_position: DVec3,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check if we have available query indices
        if self.available_indices.is_empty() {
            log::warn!("No available query indices for star {}", star_id);
            return Ok(());
        }

        // Allocate query index
        let query_index = self.available_indices.pop().unwrap();

        // Create test
        let test = OcclusionTest {
            query_index,
            star_id,
            world_position,
            frame_submitted: self.current_frame,
            is_visible: true, // Default to visible
        };

        // Store the test
        if let Some(old_test) = self.active_tests.insert(star_id, test) {
            // Return old index to pool
            self.available_indices.push(old_test.query_index);
        }

        log::debug!(
            "Queued occlusion test for star {} using query index {}",
            star_id,
            query_index
        );
        Ok(())
    }

    /// Execute occlusion queries for pending stars
    pub fn execute_occlusion_queries(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        camera_view: DMat4,
        camera_projection: DMat4,
        _camera_position: DVec3,
        color_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        mvp_bind_group: &crate::generated_shaders::default::bind_groups::BindGroup0,
        _queue: &wgpu::Queue,
        _screen_width: f32,
        _screen_height: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.active_tests.is_empty() {
            return Ok(());
        }

        self.queries_used_this_frame.clear();

        log::info!("Executing {} occlusion queries", self.active_tests.len());

        // Begin render pass with occlusion query set
        // This render pass preserves the existing rendered scene and only tests occlusion
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Simple Occlusion Query Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: color_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Preserve the rendered scene
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Use existing depth from main scene
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: Some(&self.query_set), // Enable occlusion queries
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.proxy_shader.render_pipeline);
        // Set both bind groups as expected by the pipeline
        mvp_bind_group.set(&mut render_pass); // Group 0 (MVP - unused by shader but expected by pipeline)
        // Simplified proxy shader only uses MVP bind group (group 0) - no group 1 needed
        render_pass.set_vertex_buffer(0, self.proxy_vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.proxy_index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Execute one query per star
        for (_star_id, test) in &self.active_tests {
            // Project star to screen (for future positioning - simplified shader renders at center for now)
            if let Some(_screen_pos) =
                self.project_to_screen(test.world_position, camera_view, camera_projection)
            {
                // Execute single occlusion query for this star (simplified - renders at screen center)
                render_pass.begin_occlusion_query(test.query_index);
                render_pass.draw_indexed(0..6, 0, 0..1);
                render_pass.end_occlusion_query();

                self.queries_used_this_frame.push(test.query_index);
            }
        }

        drop(render_pass);

        // Clear the used queries list for next frame
        // Note: We don't actually resolve queries to avoid wgpu snatch lock issues
        // The GPU will process the occlusion tests, but we don't read back results
        log::debug!(
            "Completed {} occlusion queries this frame",
            self.queries_used_this_frame.len()
        );
        self.queries_used_this_frame.clear();

        Ok(())
    }

    /// Project world position to screen coordinates
    fn project_to_screen(
        &self,
        world_position: DVec3,
        view_matrix: DMat4,
        projection_matrix: DMat4,
    ) -> Option<Vec2> {
        let mvp_matrix = projection_matrix.as_mat4() * view_matrix.as_mat4();

        let world_pos_f32 = Vec3::new(
            world_position.x as f32,
            world_position.y as f32,
            world_position.z as f32,
        );
        let clip_space = mvp_matrix * world_pos_f32.extend(1.0);

        if clip_space.w <= 0.0 {
            return None;
        }

        let ndc = clip_space.xyz() / clip_space.w;

        if ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0 {
            return None;
        }

        Some(Vec2::new(ndc.x, ndc.y))
    }

    /// Get visibility for a star (simplified)
    pub fn get_star_visibility(&self, star_id: StarId) -> f32 {
        self.active_tests
            .get(&star_id)
            .map(|test| if test.is_visible { 1.0 } else { 0.0 })
            .unwrap_or(1.0)
    }

    /// Process query results (simplified - just advance frame)
    pub fn process_query_results(
        &mut self,
        _device: &wgpu::Device,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement proper async result reading when needed
        self.current_frame += 1;
        Ok(())
    }

    /// Clean up old queries
    pub fn cleanup_old_queries(&mut self) {
        // Remove queries older than a few frames
        let current_frame = self.current_frame;
        let mut to_remove = Vec::new();

        for (star_id, test) in &self.active_tests {
            if current_frame - test.frame_submitted > 2 {
                to_remove.push(*star_id);
            }
        }

        for star_id in to_remove {
            if let Some(test) = self.active_tests.remove(&star_id) {
                self.available_indices.push(test.query_index);
            }
        }
    }
}
