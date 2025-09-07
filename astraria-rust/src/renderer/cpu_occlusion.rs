/// CPU-based occlusion testing using simple sphere raytracing
/// Much more reliable than GPU queries and perfect for spherical objects!
/// STATELESS DESIGN: No stored state, all data passed as parameters
use glam::DVec3;

/// Simple sphere for occlusion testing
#[derive(Debug, Clone)]
pub struct Sphere {
    pub position: DVec3,
    pub radius: f64,
}

/// Stateless CPU-based occlusion testing system
/// All functions are pure - no state stored!
pub struct CpuOcclusionSystem;

impl CpuOcclusionSystem {
    /// Create a new CPU occlusion system (stateless, so just empty)
    pub fn new() -> Self {
        Self
    }

    /// Test if a star at world_position is occluded by any sphere (returns immediately!)
    /// All data passed as parameters - no stored state!
    pub fn is_star_visible(
        camera_position: DVec3,
        star_position: DVec3,
        occluding_spheres: &[Sphere],
    ) -> bool {
        // Calculate ray from camera to star
        let ray_direction = (star_position - camera_position).normalize();
        let star_distance = (star_position - camera_position).length();

        log::info!(
            "OCCLUSION DEBUG: Ray direction: ({:.3}, {:.3}, {:.3}), star distance: {:.2e}",
            ray_direction.x,
            ray_direction.y,
            ray_direction.z,
            star_distance
        );

        // Test against all occluding spheres
        for (i, sphere) in occluding_spheres.iter().enumerate() {
            let sphere_distance = (sphere.position - camera_position).length();
            log::info!(
                "OCCLUSION DEBUG: Testing sphere {} at ({:.2e}, {:.2e}, {:.2e}) radius {:.2e}, distance from camera: {:.2e}",
                i,
                sphere.position.x,
                sphere.position.y,
                sphere.position.z,
                sphere.radius,
                sphere_distance
            );

            if Self::ray_intersects_sphere(camera_position, ray_direction, sphere, star_position) {
                // Star is occluded by this sphere
                log::info!("OCCLUSION DEBUG: *** STAR OCCLUDED by sphere {} ***", i);
                return false;
            } else {
                log::info!("OCCLUSION DEBUG: Sphere {} does not occlude star", i);
            }
        }

        // Star is visible
        log::info!("OCCLUSION DEBUG: Star is VISIBLE (no occlusion)");
        true
    }

    /// Get visibility as float (1.0 = visible, 0.0 = occluded) - for compatibility
    pub fn get_star_visibility(
        camera_position: DVec3,
        star_position: DVec3,
        occluding_spheres: &[Sphere],
    ) -> f32 {
        if Self::is_star_visible(camera_position, star_position, occluding_spheres) {
            1.0
        } else {
            0.0
        }
    }

    /// Ray-sphere intersection test optimized for occlusion
    /// Returns true if the sphere occludes the star from the camera
    fn ray_intersects_sphere(
        ray_origin: DVec3,
        ray_direction: DVec3,
        sphere: &Sphere,
        star_position: DVec3,
    ) -> bool {
        // Vector from ray origin to sphere center
        let oc = ray_origin - sphere.position;

        // Quadratic equation coefficients for ray-sphere intersection
        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * oc.dot(ray_direction);
        let c = oc.dot(oc) - sphere.radius * sphere.radius;

        let discriminant = b * b - 4.0 * a * c;

        log::info!(
            "OCCLUSION DEBUG: Ray-sphere math: a={:.3}, b={:.3}, c={:.3}, discriminant={:.3}",
            a,
            b,
            c,
            discriminant
        );

        // No intersection if discriminant is negative
        if discriminant < 0.0 {
            log::info!("OCCLUSION DEBUG: No intersection (discriminant < 0)");
            return false;
        }

        // Calculate intersection distances
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        // Check if either intersection point is between camera and star
        let star_distance = (star_position - ray_origin).length();

        log::info!(
            "OCCLUSION DEBUG: Intersection distances: t1={:.3}, t2={:.3}, star_distance={:.2e}",
            t1,
            t2,
            star_distance
        );

        let occluded = (t1 > 0.0 && t1 < star_distance) || (t2 > 0.0 && t2 < star_distance);

        log::info!(
            "OCCLUSION DEBUG: Occlusion check: t1_valid={}, t2_valid={}, result={}",
            t1 > 0.0 && t1 < star_distance,
            t2 > 0.0 && t2 < star_distance,
            occluded
        );

        // If either intersection is in front of camera and before the star, it's occluded
        occluded
    }
}

impl Default for CpuOcclusionSystem {
    fn default() -> Self {
        Self::new()
    }
}
