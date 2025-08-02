use bytemuck::{Pod, Zeroable};
/// Physics body representation for N-body simulation
/// Ported from the original Java Body.java with Rust safety improvements
use glam::DVec3;
use std::sync::{Arc, RwLock};
// Removed serde for now - can be added back when needed

/// A celestial body in the simulation with position, velocity, and mass
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Body {
    /// Mass in kilograms
    pub mass: f64,

    /// Position in meters (x, y, z)
    pub position: DVec3,

    /// Velocity in meters per second (x, y, z)
    pub velocity: DVec3,

    /// Current acceleration in meters per second squared (x, y, z)
    pub acceleration: DVec3,

    /// Whether acceleration has been initialized for this timestep
    pub acceleration_initialized: bool,
}

impl Body {
    /// Create a new body with the given properties
    pub fn new(mass: f64, position: DVec3, velocity: DVec3) -> Self {
        Self {
            mass,
            position,
            velocity,
            acceleration: DVec3::ZERO,
            acceleration_initialized: false,
        }
    }

    /// Reset acceleration initialization flag for new timestep
    pub fn reset_acceleration(&mut self) {
        self.acceleration_initialized = false;
        self.acceleration = DVec3::ZERO;
    }

    /// Set the current acceleration and mark as initialized
    pub fn set_acceleration(&mut self, acceleration: DVec3) {
        self.acceleration = acceleration;
        self.acceleration_initialized = true;
    }

    /// Get kinetic energy of the body
    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.mass * self.velocity.length_squared()
    }

    /// Get momentum vector of the body
    pub fn momentum(&self) -> DVec3 {
        self.velocity * self.mass
    }

    /// Calculate gravitational force vector to another body
    pub fn gravitational_force_to(&self, other: &Body) -> DVec3 {
        let displacement = other.position - self.position;
        let distance_squared = displacement.length_squared();

        if distance_squared == 0.0 {
            return DVec3::ZERO;
        }

        let _distance = distance_squared.sqrt();
        let force_magnitude =
            crate::math::GRAVITATIONAL_CONSTANT * self.mass * other.mass / distance_squared;

        displacement.normalize() * force_magnitude
    }

    /// Apply gravitational acceleration from another body
    pub fn apply_gravitational_acceleration(&mut self, other: &Body) {
        if self.mass == 0.0 {
            return;
        }

        let force = self.gravitational_force_to(other);
        let acceleration = force / self.mass;
        self.acceleration += acceleration;
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new(1.0, DVec3::ZERO, DVec3::ZERO)
    }
}

/// Thread-safe wrapper for a Body that can be shared between physics and rendering threads
pub type SharedBody = Arc<RwLock<Body>>;

/// Create a new shared body
pub fn new_shared_body(body: Body) -> SharedBody {
    Arc::new(RwLock::new(body))
}

/// Reduced precision body data for rendering
/// Uses f32 for GPU compatibility and smaller memory footprint
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct RenderBody {
    pub position: [f32; 3],
    pub _padding1: f32,
    pub velocity: [f32; 3],
    pub _padding2: f32,
    pub mass: f32,
    pub radius: f32,
    pub temperature: f32,
    pub _padding3: f32,
}

impl From<&Body> for RenderBody {
    fn from(body: &Body) -> Self {
        Self {
            position: [
                body.position.x as f32,
                body.position.y as f32,
                body.position.z as f32,
            ],
            _padding1: 0.0,
            velocity: [
                body.velocity.x as f32,
                body.velocity.y as f32,
                body.velocity.z as f32,
            ],
            _padding2: 0.0,
            mass: body.mass as f32,
            radius: 1.0,         // Default radius, should be set by simulation object
            temperature: 5778.0, // Default temperature (Sun-like)
            _padding3: 0.0,
        }
    }
}

/// Collection of bodies for efficient simulation
pub struct BodyCollection {
    bodies: Vec<SharedBody>,
    pending_additions: Vec<SharedBody>,
    pending_removals: Vec<usize>,
}

impl BodyCollection {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            pending_additions: Vec::new(),
            pending_removals: Vec::new(),
        }
    }

    /// Add a body to the collection (will be added at next update)
    pub fn add_body(&mut self, body: Body) {
        self.pending_additions.push(new_shared_body(body));
    }

    /// Remove a body by index (will be removed at next update)
    pub fn remove_body(&mut self, index: usize) {
        if index < self.bodies.len() {
            self.pending_removals.push(index);
        }
    }

    /// Process pending additions and removals
    pub fn update_collection(&mut self) {
        // Add new bodies
        self.bodies.extend(self.pending_additions.drain(..));

        // Remove bodies (sort indices in reverse order to avoid invalidation)
        self.pending_removals.sort_by(|a, b| b.cmp(a));
        for &index in &self.pending_removals {
            if index < self.bodies.len() {
                self.bodies.remove(index);
            }
        }
        self.pending_removals.clear();
    }

    /// Get read access to all bodies
    pub fn bodies(&self) -> &[SharedBody] {
        &self.bodies
    }

    /// Get number of bodies
    pub fn len(&self) -> usize {
        self.bodies.len()
    }

    /// Check if collection is empty
    pub fn is_empty(&self) -> bool {
        self.bodies.is_empty()
    }

    /// Calculate total system energy (kinetic + potential)
    pub fn total_energy(&self) -> f64 {
        let mut kinetic_energy = 0.0;
        let mut potential_energy = 0.0;

        // Calculate kinetic energy
        for body_ref in &self.bodies {
            if let Ok(body) = body_ref.read() {
                kinetic_energy += body.kinetic_energy();
            }
        }

        // Calculate potential energy
        for (i, body1_ref) in self.bodies.iter().enumerate() {
            for body2_ref in self.bodies.iter().skip(i + 1) {
                if let (Ok(body1), Ok(body2)) = (body1_ref.read(), body2_ref.read()) {
                    let distance = (body1.position - body2.position).length();
                    if distance > 0.0 {
                        potential_energy -=
                            crate::math::GRAVITATIONAL_CONSTANT * body1.mass * body2.mass
                                / distance;
                    }
                }
            }
        }

        kinetic_energy + potential_energy
    }

    /// Calculate center of mass of the system
    pub fn center_of_mass(&self) -> DVec3 {
        let mut total_mass = 0.0;
        let mut weighted_position = DVec3::ZERO;

        for body_ref in &self.bodies {
            if let Ok(body) = body_ref.read() {
                total_mass += body.mass;
                weighted_position += body.position * body.mass;
            }
        }

        if total_mass > 0.0 {
            weighted_position / total_mass
        } else {
            DVec3::ZERO
        }
    }
}

impl Default for BodyCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_creation() {
        let body = Body::new(
            1000.0,
            DVec3::new(1.0, 2.0, 3.0),
            DVec3::new(10.0, 20.0, 30.0),
        );

        assert_eq!(body.mass, 1000.0);
        assert_eq!(body.position, DVec3::new(1.0, 2.0, 3.0));
        assert_eq!(body.velocity, DVec3::new(10.0, 20.0, 30.0));
        assert!(!body.acceleration_initialized);
    }

    #[test]
    fn test_kinetic_energy() {
        let body = Body::new(2.0, DVec3::ZERO, DVec3::new(3.0, 4.0, 0.0));
        let expected_ke = 0.5 * 2.0 * (3.0 * 3.0 + 4.0 * 4.0); // 0.5 * m * v²
        assert!((body.kinetic_energy() - expected_ke).abs() < 1e-10);
    }

    #[test]
    fn test_gravitational_force() {
        let body1 = Body::new(1000.0, DVec3::ZERO, DVec3::ZERO);
        let body2 = Body::new(2000.0, DVec3::new(1.0, 0.0, 0.0), DVec3::ZERO);

        let force = body1.gravitational_force_to(&body2);

        // Force should be in positive x direction (toward body2)
        assert!(force.x > 0.0);
        assert_eq!(force.y, 0.0);
        assert_eq!(force.z, 0.0);

        // Force magnitude should equal G * m1 * m2 / r²
        let expected_magnitude = crate::math::GRAVITATIONAL_CONSTANT * 1000.0 * 2000.0 / 1.0;
        assert!((force.length() - expected_magnitude).abs() < 1e-10);
    }

    #[test]
    fn test_render_body_conversion() {
        let body = Body::new(
            1e24,
            DVec3::new(1e11, 2e11, 3e11),
            DVec3::new(1e4, 2e4, 3e4),
        );

        let render_body = RenderBody::from(&body);

        assert_eq!(render_body.position[0], 1e11 as f32);
        assert_eq!(render_body.velocity[1], 2e4 as f32);
        assert_eq!(render_body.mass, 1e24 as f32);
    }
}
