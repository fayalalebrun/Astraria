/// High-precision matrix calculations for astronomical scale rendering
///
/// This module provides 64-bit precision matrix calculations to prevent the NaN issues
/// that occur when using f32 matrices at astronomical distances. All calculations are
/// performed in 64-bit precision and only converted to f32 at the final step for GPU usage.
use glam::{DMat4, DQuat, DVec3, Mat4, Vec3};

use super::camera::Camera;

/// Calculate a complete MVP matrix using 64-bit precision throughout
///
/// This is the unified matrix calculation function that handles all rendering cases:
/// - Basic object rendering (when light_pos is None)
/// - Atmospheric/planetary rendering (when light_pos is Some)
/// - Skybox rendering (when is_skybox is true)
///
/// All matrix calculations are performed in 64-bit precision to handle astronomical
/// distances without precision loss, then converted to f32 for GPU usage.
///
/// # Arguments
/// * `camera` - Camera reference providing position, direction, up vector, and projection matrix
/// * `object_pos` - Object position in world coordinates (64-bit precision)
/// * `object_scale` - Object scale factor
/// * `is_skybox` - Whether this is for skybox rendering (removes translation)
/// * `light_pos` - Light position in world coordinates (None for basic rendering, Some for atmospheric effects)
///
/// # Returns
/// Tuple of (MVP matrix, camera-relative transform, light direction in camera space)
pub fn calculate_mvp_matrix_64bit_with_atmosphere(
    camera: &Camera,
    object_pos: DVec3,
    object_scale: DVec3,
    is_skybox: bool,
    light_pos: Option<DVec3>,
) -> (Mat4, Mat4, Vec3) {
    // Use camera's existing view matrix methods
    let final_view_matrix = if is_skybox {
        camera.view_matrix_rotation_only() // Camera already provides rotation-only matrix for skybox
    } else {
        camera.view_matrix() // Camera provides the full view matrix
    };

    // Calculate model matrix in 64-bit precision
    let model_matrix = create_model_matrix_64bit(object_pos, object_scale);

    // Compute final MVP in 64-bit precision
    let mvp_matrix_64 = camera.projection_matrix() * final_view_matrix * model_matrix;

    // Calculate proper camera-relative transform for atmospheric effects
    // This transforms model-space vertices to camera-relative space where camera is at origin
    // Only translation is needed - scale is handled by the model matrix
    let camera_relative_object_pos = object_pos - camera.position();
    let camera_relative_transform_64 = DMat4::from_translation(camera_relative_object_pos);

    // Calculate light direction in camera space (normalized, avoids large coordinates)
    let light_direction_camera_space = if let Some(light_world_pos) = light_pos {
        // Transform light position to camera space, then calculate direction to object
        let view_matrix = camera.view_matrix(); // Use camera's view matrix
        let light_camera_space = (view_matrix * light_world_pos.extend(1.0)).truncate();
        let object_camera_space = (view_matrix * object_pos.extend(1.0)).truncate();
        (light_camera_space - object_camera_space)
            .normalize()
            .as_vec3()
    } else {
        Vec3::new(0.0, 0.0, -1.0) // Default light direction
    };

    // Convert to f32 for GPU (safe after 64-bit calculations)
    (
        mvp_matrix_64.as_mat4(),
        camera_relative_transform_64.as_mat4(),
        light_direction_camera_space,
    )
}

/// Create a view matrix using 64-bit precision
///
/// This is equivalent to Mat4::look_at_rh but with 64-bit precision to handle
/// astronomical distances without NaN issues.
///
/// # Arguments
/// * `eye` - Camera position (64-bit precision)
/// * `center` - Look-at target position (64-bit precision)  
/// * `up` - Up direction vector (64-bit precision)
///
/// # Returns
/// View matrix in 64-bit precision
pub fn create_view_matrix_64bit(eye: DVec3, center: DVec3, up: DVec3) -> DMat4 {
    let f = (center - eye).normalize();
    let s = f.cross(up).normalize();
    let u = s.cross(f);

    DMat4::from_cols(
        DVec3::new(s.x, u.x, -f.x).extend(0.0),
        DVec3::new(s.y, u.y, -f.y).extend(0.0),
        DVec3::new(s.z, u.z, -f.z).extend(0.0),
        DVec3::new(-s.dot(eye), -u.dot(eye), f.dot(eye)).extend(1.0),
    )
}

/// Create a perspective projection matrix using 64-bit precision
///
/// # Arguments
/// * `fov_y_radians` - Field of view in radians (Y axis)
/// * `aspect_ratio` - Aspect ratio (width/height)
/// * `z_near` - Near clipping plane distance
/// * `z_far` - Far clipping plane distance
///
/// # Returns
/// Perspective projection matrix in 64-bit precision
pub fn create_perspective_64bit(
    fov_y_radians: f64,
    aspect_ratio: f64,
    z_near: f64,
    z_far: f64,
) -> DMat4 {
    let f = 1.0 / (fov_y_radians * 0.5).tan();
    let range_inv = 1.0 / (z_near - z_far);

    DMat4::from_cols(
        glam::DVec4::new(f / aspect_ratio, 0.0, 0.0, 0.0),
        glam::DVec4::new(0.0, f, 0.0, 0.0),
        glam::DVec4::new(0.0, 0.0, (z_near + z_far) * range_inv, -1.0),
        glam::DVec4::new(0.0, 0.0, z_near * z_far * range_inv * 2.0, 0.0),
    )
}

/// Create a model matrix from position and scale using 64-bit precision
///
/// # Arguments
/// * `position` - Object position in world coordinates (64-bit precision)
/// * `scale` - Object scale factors (64-bit precision)
///
/// # Returns
/// Model matrix in 64-bit precision
pub fn create_model_matrix_64bit(position: DVec3, scale: DVec3) -> DMat4 {
    DMat4::from_scale_rotation_translation(scale, DQuat::IDENTITY, position)
}

/// Create a model matrix with rotation from position, rotation, and scale using 64-bit precision
///
/// # Arguments
/// * `position` - Object position in world coordinates (64-bit precision)
/// * `rotation` - Object rotation quaternion
/// * `scale` - Object scale factors (64-bit precision)
///
/// # Returns
/// Model matrix in 64-bit precision
pub fn create_model_matrix_with_rotation_64bit(
    position: DVec3,
    rotation: DQuat,
    scale: DVec3,
) -> DMat4 {
    DMat4::from_scale_rotation_translation(scale, rotation, position)
}

/// Convert astronomical distances to human-readable format for debugging
///
/// # Arguments
/// * `distance` - Distance in meters
///
/// # Returns
/// Human-readable string representation
pub fn format_astronomical_distance(distance: f64) -> String {
    const AU: f64 = 149_597_870_700.0; // 1 Astronomical Unit in meters
    const LIGHT_YEAR: f64 = 9_460_730_472_580_800.0; // 1 Light Year in meters

    if distance.abs() < 1_000.0 {
        format!("{:.1} m", distance)
    } else if distance.abs() < 1_000_000.0 {
        format!("{:.1} km", distance / 1_000.0)
    } else if distance.abs() < AU {
        format!("{:.1} Mm", distance / 1_000_000.0)
    } else if distance.abs() < LIGHT_YEAR {
        format!("{:.3} AU", distance / AU)
    } else {
        format!("{:.3} ly", distance / LIGHT_YEAR)
    }
}

/// Validate that a matrix contains no NaN or infinite values
///
/// # Arguments
/// * `matrix` - Matrix to validate
///
/// # Returns
/// true if matrix is valid, false if it contains NaN or infinite values
pub fn validate_matrix(matrix: &Mat4) -> bool {
    for col in matrix.to_cols_array() {
        if !col.is_finite() {
            return false;
        }
    }
    true
}

/// Validate that a 64-bit matrix contains no NaN or infinite values
///
/// # Arguments
/// * `matrix` - Matrix to validate
///
/// # Returns
/// true if matrix is valid, false if it contains NaN or infinite values
pub fn validate_matrix_64bit(matrix: &DMat4) -> bool {
    for col in matrix.to_cols_array() {
        if !col.is_finite() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_matrix_64bit() {
        let eye = DVec3::new(1e9, 1e9, 1e9); // 1 billion meters
        let center = DVec3::new(0.0, 0.0, 0.0);
        let up = DVec3::new(0.0, 1.0, 0.0);

        let view_matrix = create_view_matrix_64bit(eye, center, up);
        assert!(validate_matrix_64bit(&view_matrix));
    }

    #[test]
    fn test_mvp_calculation_astronomical_scale() {
        // Create a camera at astronomical position
        let mut camera = Camera::new(16.0 / 9.0);
        camera.set_position(DVec3::new(3.85799e8, 7.96229e8, -1.86112e7)); // Actual astronomical position

        let object_pos = DVec3::new(0.0, 0.0, 0.0); // Sun at origin
        let object_scale = DVec3::new(6.96e8, 6.96e8, 6.96e8); // Sun radius

        let (mvp, _, _) = calculate_mvp_matrix_64bit_with_atmosphere(
            &camera,
            object_pos,
            object_scale,
            false,
            None,
        );

        // Ensure no NaN values in result
        assert!(validate_matrix(&mvp));
    }

    #[test]
    fn test_format_astronomical_distance() {
        assert_eq!(format_astronomical_distance(100.0), "100.0 m");
        assert_eq!(format_astronomical_distance(1500.0), "1.5 km");
        assert_eq!(format_astronomical_distance(1_500_000.0), "1.5 Mm");

        let au = 149_597_870_700.0;
        assert_eq!(format_astronomical_distance(au), "1.000 AU");
    }
}
