pub mod body;
pub mod units;

pub use body::*;
pub use units::*;

use glam::{DVec3, Mat4, Quat, Vec3};

pub const PI: f64 = std::f64::consts::PI;

/// Convert between coordinate systems and perform mathematical operations
/// specific to astronomical simulations
pub trait AstronomicalMath {
    /// Convert position from simulation units to rendering coordinates
    fn to_render_coords(&self) -> Vec3;

    /// Calculate distance in appropriate units for display
    fn distance_to(&self, other: &Self) -> f64;
}

impl AstronomicalMath for DVec3 {
    fn to_render_coords(&self) -> Vec3 {
        // Convert double precision to single precision for rendering
        // Scale down for rendering if needed
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    fn distance_to(&self, other: &Self) -> f64 {
        (*self - *other).length()
    }
}

/// Mathematical utilities for 3D transformations and physics calculations
pub struct MathUtils;

impl MathUtils {
    /// Create a transformation matrix from position and rotation
    pub fn transform_matrix(position: Vec3, rotation: Quat, scale: f32) -> Mat4 {
        Mat4::from_scale_rotation_translation(Vec3::splat(scale), rotation, position)
    }

    /// Calculate gravitational acceleration between two bodies
    pub fn gravitational_acceleration(mass: f64, distance_vector: DVec3) -> DVec3 {
        let distance = distance_vector.length();
        if distance == 0.0 {
            return DVec3::ZERO;
        }

        let force_magnitude = GRAVITATIONAL_CONSTANT * mass / (distance * distance * distance);
        distance_vector * force_magnitude
    }

    /// Square function for common physics calculations
    #[inline]
    pub fn sq(x: f64) -> f64 {
        x * x
    }

    /// Cube function for common physics calculations
    #[inline]
    pub fn cb(x: f64) -> f64 {
        x * x * x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astronomical_math() {
        let pos1 = DVec3::new(0.0, 0.0, 0.0);
        let pos2 = DVec3::new(AU_TO_METERS, 0.0, 0.0);

        let distance = pos1.distance_to(&pos2);
        assert!((distance - AU_TO_METERS).abs() < 1e-6);

        let render_coords = pos1.to_render_coords();
        assert_eq!(render_coords, Vec3::ZERO);
    }

    #[test]
    fn test_math_utils() {
        assert_eq!(MathUtils::sq(3.0), 9.0);
        assert_eq!(MathUtils::cb(2.0), 8.0);

        let matrix = MathUtils::transform_matrix(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, 1.0);

        // Test that the matrix contains the correct translation
        let translation = matrix.w_axis.truncate();
        assert!((translation - Vec3::new(1.0, 2.0, 3.0)).length() < 1e-6);
    }
}
