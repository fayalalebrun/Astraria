/// Camera system for 3D navigation and rendering
/// Ported from the original Java Camera.java with enhanced functionality
use glam::{DMat4, DVec3, Mat4, Quat, Vec3};
use std::collections::HashMap;
use wgpu::Queue;

use crate::renderer::precision_math::{create_perspective_64bit, create_view_matrix_64bit};

/// Camera movement directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
    RollLeft,
    RollRight,
}

// Import the consolidated CameraUniform from core.rs
use crate::renderer::core::CameraUniform;

/// 3D camera with astronomical scale support
pub struct Camera {
    // Double precision position for astronomical distances
    position: DVec3,

    // Camera orientation vectors (single precision for rendering)
    front: Vec3,
    up: Vec3,
    right: Vec3,
    world_up: Vec3,

    // Euler angles
    yaw: f32,
    pitch: f32,
    roll: f32,

    // Movement properties
    movement_speed: f32,
    sensitivity: f32,
    scrolled_amount: f32, // For Java-style speed calculation

    // Projection properties
    fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,

    // Movement state
    movement_keys: HashMap<CameraMovement, bool>,

    // Object locking for following simulation objects
    locked_object_position: Option<DVec3>,
    lock_distance: f32,

    // Uniform buffer
    uniform_buffer: Option<wgpu::Buffer>,
    bind_group: Option<wgpu::BindGroup>,

    // Logarithmic depth buffer support
    log_depth_constant: f32,

    // Matrices (cached) - 64-bit precision for all calculations
    view_matrix: DMat4,
    projection_matrix: DMat4,
    dirty: bool,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        log::info!("Creating new Camera instance");
        // Default position - will be set programmatically relative to physics bodies
        let position = DVec3::new(0.0, 0.0, 0.0);
        let world_up = Vec3::Y;
        let yaw = -90.0;
        let pitch = 0.0;
        let roll = 0.0;

        let mut camera = Self {
            position,
            front: Vec3::NEG_Z,
            up: world_up,
            right: Vec3::X,
            world_up,
            yaw,
            pitch,
            roll,
            movement_speed: 0.0794, // Java base movement speed
            sensitivity: 0.2,       // Doubled mouse sensitivity for better responsiveness
            scrolled_amount: 0.0,   // Initial scroll amount
            fov: 45.0,
            aspect_ratio,
            near_plane: 1e3, // 1000 meters (1 km) - much closer for nearby objects
            far_plane: 1e11, // MAXVIEWDISTANCE from Java version
            movement_keys: HashMap::new(),
            locked_object_position: None,
            lock_distance: 1e7, // 10 million units default lock distance
            uniform_buffer: None,
            bind_group: None,
            log_depth_constant: 1.0, // Logarithmic depth constant matching Java
            view_matrix: DMat4::IDENTITY,
            projection_matrix: DMat4::IDENTITY,
            dirty: true,
        };

        camera.update_vectors();
        camera.update_matrices();
        camera
    }

    pub fn initialize_gpu_resources(&mut self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        // Create uniform buffer
        self.uniform_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create bind group
        self.bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.uniform_buffer.as_ref().unwrap().as_entire_binding(),
            }],
        }));

        bind_group_layout
    }

    /// Update camera vectors based on Euler angles
    fn update_vectors(&mut self) {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();
        let roll_rad = self.roll.to_radians();

        // Calculate front vector
        // In our coordinate system: +X right, +Y up, +Z forward
        // Yaw rotates around Y axis, pitch rotates around X axis
        self.front = Vec3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        )
        .normalize();

        // Calculate right and up vectors
        // Right vector is perpendicular to front and world up
        self.right = self.front.cross(self.world_up).normalize();
        
        // Up vector is perpendicular to right and front
        self.up = self.right.cross(self.front).normalize();
        
        // Apply roll rotation to up and right vectors
        if roll_rad.abs() > 0.001 {
            let roll_quat = Quat::from_axis_angle(self.front, roll_rad);
            self.right = roll_quat * self.right;
            self.up = roll_quat * self.up;
        }

        self.dirty = true;
    }

    /// Update view and projection matrices
    fn update_matrices(&mut self) {
        if !self.dirty {
            return;
        }
        log::debug!(
            "Camera matrices dirty, updating: yaw={:.1}°, pitch={:.1}°",
            self.yaw,
            self.pitch
        );

        // Handle object locking
        if let Some(locked_pos) = self.locked_object_position {
            // Position camera at lock distance from the locked object
            let direction = (self.position - locked_pos).normalize();
            self.position = locked_pos + direction * self.lock_distance as f64;
        }

        // Calculate view matrix using 64-bit precision (eliminates NaN at astronomical distances)
        let target = self.position + self.front.as_dvec3();
        self.view_matrix = create_view_matrix_64bit(self.position, target, self.up.as_dvec3());

        // Calculate projection matrix using 64-bit precision
        self.projection_matrix = create_perspective_64bit(
            self.fov.to_radians() as f64,
            self.aspect_ratio as f64,
            self.near_plane as f64,
            self.far_plane as f64,
        );

        self.dirty = false;
    }

    /// Update GPU uniforms
    pub fn update(&mut self, queue: &Queue) {
        self.update_matrices();

        if let Some(buffer) = &self.uniform_buffer {
            log::debug!(
                "Updating camera GPU uniforms: yaw={:.1}°, pitch={:.1}°",
                self.yaw,
                self.pitch
            );
            // Calculate fc_constant for logarithmic depth (matches Java: 1.0f/(float)Math.log(MAXVIEWDISTANCE*LOGDEPTHCONSTANT + 1))
            let fc_constant = 1.0 / (self.log_depth_constant * self.far_plane + 1.0).ln();

            // Convert 64-bit matrices to f32 for GPU (safe after 64-bit calculations)
            let view_matrix_f32 = self.view_matrix.as_mat4();
            let projection_matrix_f32 = self.projection_matrix.as_mat4();
            let view_projection_matrix_f32 = (self.projection_matrix * self.view_matrix).as_mat4();

            let uniforms = CameraUniform {
                view_matrix: view_matrix_f32.to_cols_array_2d(),
                projection_matrix: projection_matrix_f32.to_cols_array_2d(),
                view_projection_matrix: view_projection_matrix_f32.to_cols_array_2d(),
                camera_position: self.position.as_vec3().to_array(),
                _padding1: 0.0,
                camera_direction: self.front.to_array(),
                _padding2: 0.0,
                log_depth_constant: 1.0, // Logarithmic depth constant
                far_plane_distance: self.far_plane,
                near_plane_distance: self.near_plane,
                fc_constant, // FC constant for logarithmic depth calculations
            };

            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[uniforms]));
        }
    }

    /// Process keyboard input for camera movement
    pub fn process_keyboard(&mut self, movement: CameraMovement, pressed: bool) {
        self.movement_keys.insert(movement, pressed);
    }

    /// Process mouse movement for camera rotation
    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32) {
        let x_offset = x_offset * self.sensitivity;
        let y_offset = y_offset * self.sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        // Constrain pitch to avoid camera flipping
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        self.update_vectors();
    }

    /// Process scroll wheel for speed adjustment (matching Java Camera.java)
    pub fn process_scroll(&mut self, y_offset: f32) {
        // Update scrolled amount like Java implementation
        self.scrolled_amount += y_offset;

        // Apply Java's exponential speed formula: 0.0794f * pow(1.2637, scrolledAmount)
        self.movement_speed = 0.0794 * 1.2637_f32.powf(self.scrolled_amount);

        // Clamp speed to reasonable range (allowing very slow and very fast speeds)
        self.movement_speed = self.movement_speed.clamp(1e-10, 1e12);
    }

    /// Update camera position based on current movement state
    pub fn update_movement(&mut self, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        let mut position_changed = false;
        let mut rotation_changed = false;

        // Collect movements to avoid borrowing issues
        let movements: Vec<(CameraMovement, bool)> = self
            .movement_keys
            .iter()
            .map(|(&movement, &pressed)| (movement, pressed))
            .collect();

        for (movement, pressed) in &movements {
            if *pressed {
                match *movement {
                    CameraMovement::Forward => {
                        self.position += self.front.as_dvec3() * velocity as f64;
                        position_changed = true;
                    }
                    CameraMovement::Backward => {
                        self.position -= self.front.as_dvec3() * velocity as f64;
                        position_changed = true;
                    }
                    CameraMovement::Left => {
                        self.position -= self.right.as_dvec3() * velocity as f64;
                        position_changed = true;
                    }
                    CameraMovement::Right => {
                        self.position += self.right.as_dvec3() * velocity as f64;
                        position_changed = true;
                    }
                    CameraMovement::Up => {
                        self.position += self.up.as_dvec3() * velocity as f64;
                        position_changed = true;
                    }
                    CameraMovement::Down => {
                        self.position -= self.up.as_dvec3() * velocity as f64;
                        position_changed = true;
                    }
                    CameraMovement::RollLeft => {
                        self.roll -= 90.0 * delta_time; // 90 degrees per second
                        rotation_changed = true;
                        log::debug!("Roll left: new roll={:.1}°", self.roll);
                    }
                    CameraMovement::RollRight => {
                        self.roll += 90.0 * delta_time; // 90 degrees per second
                        rotation_changed = true;
                        log::debug!("Roll right: new roll={:.1}°", self.roll);
                    }
                }
            }
        }

        // Update vectors immediately if rotation changed
        if rotation_changed {
            self.update_vectors();
        }

        // Mark as dirty if anything changed
        if position_changed || rotation_changed {
            self.dirty = true;
        }
    }

    /// Lock camera to follow a simulation object
    pub fn lock_to_object(&mut self, object_position: DVec3) {
        self.locked_object_position = Some(object_position);
        self.dirty = true;
    }

    /// Unlock camera from following an object
    pub fn unlock(&mut self) {
        self.locked_object_position = None;
    }

    /// Check if camera is locked to an object
    pub fn is_locked(&self) -> bool {
        self.locked_object_position.is_some()
    }

    /// Set aspect ratio (called on window resize)
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.dirty = true;
    }

    /// Get the view matrix in 64-bit precision for matrix calculations
    pub fn view_matrix(&self) -> DMat4 {
        self.view_matrix
    }

    /// Get the projection matrix in 64-bit precision for matrix calculations
    pub fn projection_matrix(&self) -> DMat4 {
        self.projection_matrix
    }

    /// Get the view-projection matrix in 64-bit precision for matrix calculations
    pub fn view_projection_matrix(&self) -> DMat4 {
        self.projection_matrix * self.view_matrix
    }

    /// Get the view matrix converted to f32 for legacy GPU usage
    pub fn view_matrix_f32(&self) -> Mat4 {
        self.view_matrix.as_mat4()
    }

    /// Get view matrix with translation removed (rotation only) - for skybox rendering
    /// This ensures skybox is never affected by camera position, only rotation
    pub fn view_matrix_rotation_only(&self) -> DMat4 {
        use crate::renderer::precision_math::remove_translation_64bit;
        remove_translation_64bit(self.view_matrix)
    }

    /// Get the projection matrix converted to f32 for legacy GPU usage
    pub fn projection_matrix_f32(&self) -> Mat4 {
        self.projection_matrix.as_mat4()
    }

    /// Get the view-projection matrix converted to f32 for legacy GPU usage
    pub fn view_projection_matrix_f32(&self) -> Mat4 {
        (self.projection_matrix * self.view_matrix).as_mat4()
    }

    /// Get camera position
    pub fn position(&self) -> DVec3 {
        self.position
    }

    /// Position camera relative to a body at a multiple of its radius
    pub fn position_relative_to_body(
        &mut self,
        body_position: DVec3,
        body_radius: f64,
        distance_multiplier: f64,
    ) {
        let distance = body_radius * distance_multiplier;
        // Position camera at distance from body, looking at it
        self.position = body_position + DVec3::new(0.0, 0.0, distance);
        self.dirty = true;
        log::info!(
            "Positioned camera at distance {:.2e} meters from body at ({:.2e}, {:.2e}, {:.2e})",
            distance,
            body_position.x,
            body_position.y,
            body_position.z
        );
    }

    /// Get camera direction (front vector)
    pub fn direction(&self) -> Vec3 {
        self.front
    }

    /// Get camera up vector
    pub fn up(&self) -> Vec3 {
        self.up
    }

    /// Get camera right vector
    pub fn right(&self) -> Vec3 {
        self.right
    }

    /// Get camera bind group for shaders
    pub fn bind_group(&self) -> Option<&wgpu::BindGroup> {
        self.bind_group.as_ref()
    }

    /// Adjust movement speed
    pub fn change_speed(&mut self, delta: f32) {
        let multiplier = if delta > 0.0 {
            1.1f32.powf(delta)
        } else {
            0.9f32.powf(-delta)
        };
        self.movement_speed *= multiplier;
        self.movement_speed = self.movement_speed.clamp(1.0, 1e12);
    }

    /// Get reference to uniform buffer
    pub fn uniform_buffer(&self) -> Option<&wgpu::Buffer> {
        self.uniform_buffer.as_ref()
    }

    pub fn set_position(&mut self, position: DVec3) {
        self.position = position;
        self.update_vectors();
    }

    pub fn look_at(&mut self, target: DVec3) {
        // Calculate direction from camera to target
        let direction = target - self.position;
        
        // Check if target is at the same position as camera
        if direction.length_squared() < 1e-10 {
            log::warn!("Camera look_at: target is at the same position as camera, ignoring");
            return;
        }
        
        let direction = direction.normalize();
        
        // Set front directly to this direction
        self.front = direction.as_vec3().normalize();
        
        // Calculate right vector
        // Check if we're looking nearly parallel to world up
        let dot_with_up = self.front.dot(self.world_up).abs();
        
        if dot_with_up > 0.9999 {
            // Looking almost straight up or down
            // Use the previous right vector or a default one
            if self.right.length_squared() > 0.1 {
                // Keep existing right vector, just ensure it's perpendicular
                let temp_right = if dot_with_up > 0.0 {
                    // Looking up, use a vector perpendicular to up
                    Vec3::X
                } else {
                    // Looking down, use opposite
                    Vec3::NEG_X
                };
                self.right = temp_right - self.front * temp_right.dot(self.front);
                self.right = self.right.normalize();
            } else {
                // No valid previous right vector, create one
                self.right = if self.front.y > 0.0 {
                    Vec3::X
                } else {
                    Vec3::NEG_X
                };
            }
        } else {
            // Normal case: calculate right as cross product
            self.right = self.front.cross(self.world_up).normalize();
        }
        
        // Calculate up vector as perpendicular to right and front
        self.up = self.right.cross(self.front).normalize();
        
        // Calculate yaw and pitch from the front vector for consistency
        // Yaw: angle in the XZ plane (rotation around Y axis)
        // atan2 handles all quadrants correctly
        self.yaw = self.front.z.atan2(self.front.x).to_degrees();
        
        // Pitch: angle from the horizontal plane
        // Clamp to avoid numerical issues at extremes
        let clamped_y = self.front.y.clamp(-0.9999, 0.9999);
        self.pitch = clamped_y.asin().to_degrees();
        
        // Reset roll when looking at a target
        self.roll = 0.0;
        
        // Mark as dirty to force matrix updates
        self.dirty = true;
        
        log::debug!(
            "Camera look_at: position=({:.2e}, {:.2e}, {:.2e}), target=({:.2e}, {:.2e}, {:.2e}), yaw={:.1}°, pitch={:.1}°",
            self.position.x, self.position.y, self.position.z,
            target.x, target.y, target.z,
            self.yaw, self.pitch
        );
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(16.0 / 9.0)
    }
}
