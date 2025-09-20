/// Simple orbital path rendering system - Java Astraria style
/// Uses fixed-size ring buffers and basic line rendering with adaptive sampling
use std::collections::VecDeque;
use glam::DVec3;
use wgpu::{Buffer, util::DeviceExt};

/// Simple orbital path trail for a single celestial body
/// Directly ported from Java Orbit.java with ring buffer approach
#[derive(Debug)]
pub struct OrbitTrail {
    /// Ring buffer of world positions (x, y, z) stored as f64 for precision
    positions: VecDeque<DVec3>,
    
    /// Maximum number of trail points (like Java MAX_POINTS = 500)
    max_points: usize,
    
    /// Last recorded position for adaptive sampling
    last_position: DVec3,
    
    /// Minimum distance before adding new point (like Java segmentLength)
    segment_length: f64,
    
    /// Trail color for this object
    color: [f32; 4],
    
    /// GPU vertex buffer for rendering
    vertex_buffer: Option<Buffer>,
    
    /// Current number of vertices to draw
    vertex_count: usize,
    
    /// Whether trail needs GPU buffer update
    needs_update: bool,
}

impl OrbitTrail {
    /// Create new orbit trail (like Java Orbit constructor)
    pub fn new(color: [f32; 4]) -> Self {
        Self {
            positions: VecDeque::with_capacity(500),
            max_points: 500, // Same as Java MAX_POINTS
            last_position: DVec3::ZERO,
            segment_length: 5_000_000.0, // 5000 km, same as Java
            color,
            vertex_buffer: None,
            vertex_count: 0,
            needs_update: true,
        }
    }

    /// Add position if object moved far enough (like Java prepare() method)
    pub fn update_position(&mut self, world_position: DVec3) {
        // Check if we should add a new point (adaptive sampling)
        let distance_moved = (world_position - self.last_position).length();
        
        if distance_moved > self.segment_length || self.positions.is_empty() {
            // Add new position to ring buffer
            self.positions.push_back(world_position);
            self.last_position = world_position;
            log::debug!("Added orbital trail point at ({:.2e}, {:.2e}, {:.2e}), distance={:.2e}, total_points={}", 
                       world_position.x, world_position.y, world_position.z, distance_moved, self.positions.len());
            
            // Remove oldest points if over capacity (ring buffer behavior)
            while self.positions.len() > self.max_points {
                self.positions.pop_front();
                log::debug!("Removed oldest orbital trail point, now have {} points", self.positions.len());
            }
            
            self.needs_update = true;
        } else {
            log::trace!("Skipped orbital trail point update: distance_moved={:.2e} < segment_length={:.2e}", 
                       distance_moved, self.segment_length);
        }
    }

    /// Convert world positions to camera-relative vertices for GPU (like Java prepare())
    pub fn update_gpu_buffer(&mut self, device: &wgpu::Device, camera_position: DVec3) {
        if !self.needs_update || self.positions.len() < 2 {
            log::debug!("Skipping GPU buffer update: needs_update={}, positions.len()={}", 
                       self.needs_update, self.positions.len());
            return;
        }

        log::debug!("Updating orbital trail GPU buffer: {} trail points, camera at ({:.2e}, {:.2e}, {:.2e})",
                   self.positions.len(), camera_position.x, camera_position.y, camera_position.z);

        // Convert world positions to camera-relative f32 vertices for existing line shader
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(self.positions.len());
        
        for (i, world_pos) in self.positions.iter().enumerate() {
            // Make camera-relative for floating point precision (like Java)
            let relative_pos = (*world_pos - camera_position).as_vec3();
            vertices.push([relative_pos.x, relative_pos.y, relative_pos.z]);
            if i < 3 || i >= self.positions.len() - 3 {
                log::debug!("Trail vertex {}: world=({:.2e}, {:.2e}, {:.2e}), relative=({:.2e}, {:.2e}, {:.2e})",
                           i, world_pos.x, world_pos.y, world_pos.z, 
                           relative_pos.x, relative_pos.y, relative_pos.z);
            }
        }

        // Create or update GPU buffer compatible with existing VertexInput
        if vertices.len() >= 2 { // Need at least 2 points for a line
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Orbit Trail Vertices"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            
            self.vertex_buffer = Some(buffer);
            self.vertex_count = vertices.len();
            log::debug!("Created orbital trail GPU buffer with {} vertices", vertices.len());
        }

        self.needs_update = false;
    }

    /// Get vertex buffer for rendering with existing LineShader
    pub fn get_vertex_buffer(&self) -> Option<&Buffer> {
        self.vertex_buffer.as_ref()
    }

    /// Get number of vertices to draw
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count as u32
    }

    /// Get trail color
    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    /// Get current trail length (number of points)
    pub fn trail_length(&self) -> usize {
        self.positions.len()
    }

    /// Check if trail has enough points to render
    pub fn is_renderable(&self) -> bool {
        self.vertex_count >= 2 && self.vertex_buffer.is_some()
    }

    /// Check if trail needs GPU buffer update
    pub fn needs_update(&self) -> bool {
        self.needs_update
    }

    /// Clear all trail points
    pub fn clear(&mut self) {
        self.positions.clear();
        self.vertex_buffer = None;
        self.vertex_count = 0;
        self.needs_update = true;
        self.last_position = DVec3::ZERO;
    }

    /// Configure trail parameters
    pub fn set_config(&mut self, max_points: usize, segment_length: f64, color: [f32; 4]) {
        self.max_points = max_points;
        self.segment_length = segment_length;
        self.color = color;
        
        // Trim if necessary
        while self.positions.len() > max_points {
            self.positions.pop_front();
        }
        self.needs_update = true;
    }
}

// Note: We don't need SimpleOrbitalRenderer anymore!  
// The existing LineShader system handles all rendering:
//
// Usage example:
// 1. Create OrbitTrail for each body: trail = OrbitTrail::new(color)
// 2. Update positions: trail.update_position(body.position) 
// 3. Update GPU buffers: trail.update_gpu_buffer(device, camera_pos)
// 4. Render with existing LineShader:
//    - Use trail.get_vertex_buffer() as vertex buffer
//    - Set line uniform color to trail.color()
//    - Draw trail.vertex_count() vertices with LineList topology
//
// This leverages the existing shader system without duplication!