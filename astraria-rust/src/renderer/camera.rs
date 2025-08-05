/// Camera system for 3D navigation and rendering
/// Ported from the original Java Camera.java with enhanced functionality
use glam::{DMat4, DVec3, Mat4, Quat, Vec3};

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
    // Essential state
    position: DVec3,
    rotation: Quat,

    // Projection parameters
    fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,

    // Movement properties
    movement_speed: f32,
    sensitivity: f32,

    // Optional features
    locked_object_position: Option<DVec3>,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        log::info!("Creating new Camera instance");

        Self {
            position: DVec3::new(0.0, 0.0, 0.0),
            rotation: Quat::IDENTITY, // Start with identity (looking down -Z)
            fov: 45.0,
            aspect_ratio,
            near_plane: 1e3,        // 1000 meters (1 km)
            far_plane: 1e11,        // MAXVIEWDISTANCE from Java version
            movement_speed: 0.0794, // Java base movement speed
            sensitivity: 0.2,
            locked_object_position: None,
        }
    }

    /// Calculate front vector from quaternion
    fn calculate_front(&self) -> Vec3 {
        // Front is -Z direction in camera space, transformed by rotation
        self.rotation * Vec3::NEG_Z
    }

    /// Calculate up vector from quaternion
    fn calculate_up(&self) -> Vec3 {
        // Up is Y direction in camera space, transformed by rotation
        self.rotation * Vec3::Y
    }

    /// Calculate right vector from quaternion
    fn calculate_right(&self) -> Vec3 {
        // Right is X direction in camera space, transformed by rotation
        self.rotation * Vec3::X
    }

    /// Calculate view matrix on demand
    fn calculate_view_matrix(&self) -> DMat4 {
        let front = self.calculate_front();
        let up = self.calculate_up();
        let target = self.position + front.as_dvec3();
        create_view_matrix_64bit(self.position, target, up.as_dvec3())
    }

    /// Calculate projection matrix on demand
    fn calculate_projection_matrix(&self) -> DMat4 {
        create_perspective_64bit(
            self.fov.to_radians() as f64,
            self.aspect_ratio as f64,
            self.near_plane as f64,
            self.far_plane as f64,
        )
    }

    /// Apply rotation deltas in camera coordinate space
    fn apply_rotation(&mut self, yaw_delta: f32, pitch_delta: f32, roll_delta: f32) {
        // Create rotation quaternions for each axis in camera space
        // Yaw: rotation around camera's up axis (Y)
        // Pitch: rotation around camera's right axis (X)
        // Roll: rotation around camera's front axis (Z)
        let yaw_quat = Quat::from_axis_angle(Vec3::Y, yaw_delta.to_radians());
        let pitch_quat = Quat::from_axis_angle(Vec3::X, pitch_delta.to_radians());
        let roll_quat = Quat::from_axis_angle(Vec3::Z, roll_delta.to_radians());

        // Apply rotations in camera space: current * yaw * pitch * roll
        // This ensures rotations happen around the camera's local axes
        self.rotation = self.rotation * yaw_quat * pitch_quat * roll_quat;

        // Normalize to prevent drift
        self.rotation = self.rotation.normalize();
    }

    /// Get camera uniform data for GPU
    pub fn get_uniform(&self) -> CameraUniform {
        let view_matrix = self.calculate_view_matrix();
        let projection_matrix = self.calculate_projection_matrix();
        let view_projection_matrix = projection_matrix * view_matrix;
        let front = self.calculate_front();

        // Calculate fc_constant for logarithmic depth
        let log_depth_constant = 1.0;
        let fc_constant = 1.0 / (log_depth_constant * self.far_plane + 1.0).ln();

        CameraUniform {
            view_matrix: view_matrix.as_mat4().to_cols_array_2d(),
            projection_matrix: projection_matrix.as_mat4().to_cols_array_2d(),
            view_projection_matrix: view_projection_matrix.as_mat4().to_cols_array_2d(),
            camera_position: self.position.as_vec3().to_array(),
            _padding1: 0.0,
            camera_direction: front.to_array(),
            _padding2: 0.0,
            log_depth_constant: 1.0,
            far_plane_distance: self.far_plane,
            near_plane_distance: self.near_plane,
            fc_constant,
        }
    }

    /// Process mouse movement for camera rotation
    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32) {
        let yaw_delta = x_offset * self.sensitivity;
        let pitch_delta = -y_offset * self.sensitivity; // Negative for natural mouse movement

        // Apply rotations directly - no gimbal lock with quaternions!
        self.apply_rotation(yaw_delta, pitch_delta, 0.0);
    }

    /// Process scroll wheel for speed adjustment
    pub fn process_scroll(&mut self, y_offset: f32) {
        let multiplier: f32 = if y_offset > 0.0 { 1.2637 } else { 1.0 / 1.2637 };
        self.movement_speed *= multiplier.powf(y_offset.abs());
        self.movement_speed = self.movement_speed.clamp(1e-10, 1e12);
    }

    /// Update camera position based on movement
    pub fn process_movement(&mut self, movement: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        let front = self.calculate_front();
        let right = self.calculate_right();
        let up = self.calculate_up();

        match movement {
            CameraMovement::Forward => {
                self.position += front.as_dvec3() * velocity as f64;
            }
            CameraMovement::Backward => {
                self.position -= front.as_dvec3() * velocity as f64;
            }
            CameraMovement::Left => {
                self.position -= right.as_dvec3() * velocity as f64;
            }
            CameraMovement::Right => {
                self.position += right.as_dvec3() * velocity as f64;
            }
            CameraMovement::Up => {
                self.position += up.as_dvec3() * velocity as f64;
            }
            CameraMovement::Down => {
                self.position -= up.as_dvec3() * velocity as f64;
            }
            CameraMovement::RollLeft => {
                self.apply_rotation(0.0, 0.0, -90.0 * delta_time);
            }
            CameraMovement::RollRight => {
                self.apply_rotation(0.0, 0.0, 90.0 * delta_time);
            }
        }
    }

    /// Lock camera to follow a simulation object
    pub fn lock_to_object(&mut self, object_position: DVec3) {
        self.locked_object_position = Some(object_position);
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
    }

    /// Get the view matrix in 64-bit precision for matrix calculations
    pub fn view_matrix(&self) -> DMat4 {
        self.calculate_view_matrix()
    }

    /// Get the projection matrix in 64-bit precision for matrix calculations
    pub fn projection_matrix(&self) -> DMat4 {
        self.calculate_projection_matrix()
    }

    /// Get the view-projection matrix in 64-bit precision for matrix calculations
    pub fn view_projection_matrix(&self) -> DMat4 {
        self.calculate_projection_matrix() * self.calculate_view_matrix()
    }

    /// Get the view matrix converted to f32 for legacy GPU usage
    pub fn view_matrix_f32(&self) -> Mat4 {
        self.calculate_view_matrix().as_mat4()
    }

    /// Get view matrix with translation removed (rotation only) - for skybox rendering
    pub fn view_matrix_rotation_only(&self) -> DMat4 {
        use crate::renderer::precision_math::remove_translation_64bit;
        remove_translation_64bit(self.calculate_view_matrix())
    }

    /// Get the projection matrix converted to f32 for legacy GPU usage
    pub fn projection_matrix_f32(&self) -> Mat4 {
        self.calculate_projection_matrix().as_mat4()
    }

    /// Get the view-projection matrix converted to f32 for legacy GPU usage
    pub fn view_projection_matrix_f32(&self) -> Mat4 {
        (self.calculate_projection_matrix() * self.calculate_view_matrix()).as_mat4()
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
        self.calculate_front()
    }

    /// Get camera up vector
    pub fn up(&self) -> Vec3 {
        self.calculate_up()
    }

    /// Get camera right vector
    pub fn right(&self) -> Vec3 {
        self.calculate_right()
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

    pub fn set_position(&mut self, position: DVec3) {
        self.position = position;
    }

    pub fn look_at(&mut self, target: DVec3, distance: f64) {
        // Reset to default rotation (looking down -Z)
        self.rotation = Quat::IDENTITY;

        // Position camera at specified distance along default direction
        // Default direction is -Z, so we offset in +Z to look back at target
        self.position = target + DVec3::new(0.0, 0.0, distance);

        log::debug!(
            "Camera look_at: positioned at distance {:.2e} from target",
            distance
        );
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(16.0 / 9.0)
    }
}
