use bytemuck::{Pod, Zeroable};
/// Camera system for 3D navigation and rendering
/// Ported from the original Java Camera.java with enhanced functionality
use glam::{DVec3, Mat4, Quat, Vec3};
use std::collections::HashMap;
use wgpu::Queue;

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

/// Camera uniform buffer data for shaders
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniforms {
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub view_projection_matrix: [[f32; 4]; 4],
    pub camera_position: [f32; 3],
    pub _padding1: f32,
    pub camera_direction: [f32; 3],
    pub _padding2: f32,
    pub log_depth_constant: f32,
    pub far_plane_distance: f32,
    pub near_plane_distance: f32,
    pub _padding3: f32,
}

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

    // Matrices (cached)
    view_matrix: Mat4,
    projection_matrix: Mat4,
    view_projection_matrix: Mat4,
    dirty: bool,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        let position = DVec3::new(0.0, 0.0, 5.0); // Start further back
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
            movement_speed: 2.5, // Slower speed for testing scene
            sensitivity: 0.1,
            fov: 45.0,
            aspect_ratio,
            near_plane: 0.1,
            far_plane: 1e11, // 100 billion units for astronomical scale
            movement_keys: HashMap::new(),
            locked_object_position: None,
            lock_distance: 1e7, // 10 million units default lock distance
            uniform_buffer: None,
            bind_group: None,
            view_matrix: Mat4::IDENTITY,
            projection_matrix: Mat4::IDENTITY,
            view_projection_matrix: Mat4::IDENTITY,
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
            size: std::mem::size_of::<CameraUniforms>() as wgpu::BufferAddress,
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
        self.front = Vec3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        )
        .normalize();

        // Calculate right and up vectors with roll support
        let basic_right = self.front.cross(self.world_up).normalize();
        let basic_up = basic_right.cross(self.front).normalize();

        // Apply roll rotation
        let roll_quat = Quat::from_axis_angle(self.front, roll_rad);
        self.right = roll_quat * basic_right;
        self.up = roll_quat * basic_up;

        self.dirty = true;
    }

    /// Update view and projection matrices
    fn update_matrices(&mut self) {
        if !self.dirty {
            return;
        }

        // Handle object locking
        if let Some(locked_pos) = self.locked_object_position {
            // Position camera at lock distance from the locked object
            let direction = (self.position - locked_pos).normalize();
            self.position = locked_pos + direction * self.lock_distance as f64;
        }

        // Convert double precision position to single precision for rendering
        let render_position = self.position.as_vec3();

        // Calculate view matrix
        self.view_matrix = Mat4::look_at_rh(render_position, render_position + self.front, self.up);

        // Calculate projection matrix with logarithmic depth support
        self.projection_matrix = Mat4::perspective_rh(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.near_plane,
            self.far_plane,
        );

        // Calculate combined view-projection matrix
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;

        self.dirty = false;
    }

    /// Update GPU uniforms
    pub fn update(&mut self, queue: &Queue) {
        self.update_matrices();

        if let Some(buffer) = &self.uniform_buffer {
            let uniforms = CameraUniforms {
                view_matrix: self.view_matrix.to_cols_array_2d(),
                projection_matrix: self.projection_matrix.to_cols_array_2d(),
                view_projection_matrix: self.view_projection_matrix.to_cols_array_2d(),
                camera_position: self.position.as_vec3().to_array(),
                _padding1: 0.0,
                camera_direction: self.front.to_array(),
                _padding2: 0.0,
                log_depth_constant: 1.0, // Logarithmic depth constant
                far_plane_distance: self.far_plane,
                near_plane_distance: self.near_plane,
                _padding3: 0.0,
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

    /// Process scroll wheel for speed adjustment
    pub fn process_scroll(&mut self, y_offset: f32) {
        // Adjust movement speed based on scroll
        let speed_multiplier = if y_offset > 0.0 { 1.1 } else { 0.9 };
        self.movement_speed *= speed_multiplier;

        // Clamp speed to reasonable range
        self.movement_speed = self.movement_speed.clamp(1.0, 1e12);
    }

    /// Update camera position based on current movement state
    pub fn update_movement(&mut self, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        let mut needs_vector_update = false;

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
                    }
                    CameraMovement::Backward => {
                        self.position -= self.front.as_dvec3() * velocity as f64;
                    }
                    CameraMovement::Left => {
                        self.position -= self.right.as_dvec3() * velocity as f64;
                    }
                    CameraMovement::Right => {
                        self.position += self.right.as_dvec3() * velocity as f64;
                    }
                    CameraMovement::Up => {
                        self.position += self.up.as_dvec3() * velocity as f64;
                    }
                    CameraMovement::Down => {
                        self.position -= self.up.as_dvec3() * velocity as f64;
                    }
                    CameraMovement::RollLeft => {
                        self.roll -= 90.0 * delta_time; // 90 degrees per second
                        needs_vector_update = true;
                    }
                    CameraMovement::RollRight => {
                        self.roll += 90.0 * delta_time; // 90 degrees per second
                        needs_vector_update = true;
                    }
                }
            }
        }

        if needs_vector_update {
            self.update_vectors();
        }

        // Always mark as dirty if any movement keys were processed
        if !movements.is_empty() {
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

    /// Get the view-projection matrix for rendering
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.view_projection_matrix
    }

    /// Get camera position
    pub fn position(&self) -> DVec3 {
        self.position
    }

    /// Get camera direction
    pub fn direction(&self) -> Vec3 {
        self.front
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
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(16.0 / 9.0)
    }
}
