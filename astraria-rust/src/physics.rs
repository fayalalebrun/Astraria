/// Physics simulation system
/// Ported from the original Java N-body simulation with enhanced threading
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::{
    AstrariaError, AstrariaResult,
    math::{Body, BodyCollection, GRAVITATIONAL_CONSTANT},
};

/// Velocity-Verlet integration algorithm for N-body simulation
/// Ported from the original VelocityVerlet.java
pub struct VelocityVerlet {
    bodies: Arc<RwLock<BodyCollection>>,
    simulation_speed: Arc<RwLock<f32>>,
    terminate_flag: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

impl Default for VelocityVerlet {
    fn default() -> Self {
        Self::new()
    }
}

impl VelocityVerlet {
    pub fn new() -> Self {
        Self {
            bodies: Arc::new(RwLock::new(BodyCollection::new())),
            simulation_speed: Arc::new(RwLock::new(1.0)),
            terminate_flag: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    pub fn start_simulation(&mut self) -> AstrariaResult<()> {
        if self.thread_handle.is_some() {
            return Err(AstrariaError::Physics(
                "Simulation already running".to_string(),
            ));
        }

        let bodies = Arc::clone(&self.bodies);
        let simulation_speed = Arc::clone(&self.simulation_speed);
        let terminate_flag = Arc::clone(&self.terminate_flag);

        let handle = thread::spawn(move || {
            let mut last_time = Instant::now();

            while !terminate_flag.load(Ordering::Relaxed) {
                let current_time = Instant::now();
                let mut delta_time = current_time.duration_since(last_time).as_secs_f64();
                last_time = current_time;

                // Apply simulation speed multiplier
                if let Ok(speed) = simulation_speed.read() {
                    delta_time *= *speed as f64;
                }

                // Limit delta time to prevent numerical instability
                delta_time = delta_time.min(0.1);

                // Run the integration step
                if let Err(e) = Self::integration_step(&bodies, delta_time) {
                    log::error!("Physics integration error: {e}");
                    break;
                }

                // Sleep briefly to avoid maxing out CPU
                thread::sleep(Duration::from_millis(1));
            }

            log::info!("Physics simulation thread terminated");
        });

        self.thread_handle = Some(handle);
        Ok(())
    }

    fn integration_step(
        bodies: &Arc<RwLock<BodyCollection>>,
        delta_time: f64,
    ) -> AstrariaResult<()> {
        let bodies_guard = bodies
            .read()
            .map_err(|_| AstrariaError::Physics("Failed to acquire read lock".to_string()))?;

        let body_refs = bodies_guard.bodies();
        if body_refs.is_empty() {
            return Ok(());
        }

        // Phase 1: Update positions and calculate new accelerations
        for (i, body_ref) in body_refs.iter().enumerate() {
            let mut body = body_ref.write().map_err(|_| {
                AstrariaError::Physics("Failed to acquire body write lock".to_string())
            })?;

            // Reset acceleration for this timestep
            if !body.acceleration_initialized {
                body.reset_acceleration();

                // Calculate gravitational acceleration from all other bodies
                for (j, other_body_ref) in body_refs.iter().enumerate() {
                    if i != j {
                        let other_body = other_body_ref.read().map_err(|_| {
                            AstrariaError::Physics(
                                "Failed to acquire other body read lock".to_string(),
                            )
                        })?;

                        let displacement = other_body.position - body.position;
                        let distance_squared = displacement.length_squared();

                        if distance_squared > 0.0 {
                            let distance = distance_squared.sqrt();
                            let force_magnitude = GRAVITATIONAL_CONSTANT * other_body.mass
                                / (distance_squared * distance);

                            body.acceleration += displacement * force_magnitude;
                        }
                    }
                }

                body.acceleration_initialized = true;
            }

            // Update position using Velocity-Verlet: x(t+dt) = x(t) + v(t)*dt + 0.5*a(t)*dt²
            body.position = body.position
                + body.velocity * delta_time
                + body.acceleration * (0.5 * delta_time * delta_time);
        }

        // Phase 2: Calculate new accelerations at new positions
        let mut new_accelerations = Vec::with_capacity(body_refs.len());

        for (i, body_ref) in body_refs.iter().enumerate() {
            let body = body_ref.read().map_err(|_| {
                AstrariaError::Physics("Failed to acquire body read lock".to_string())
            })?;

            let mut acceleration = glam::DVec3::ZERO;

            // Calculate acceleration from all other bodies
            for (j, other_body_ref) in body_refs.iter().enumerate() {
                if i != j {
                    let other_body = other_body_ref.read().map_err(|_| {
                        AstrariaError::Physics("Failed to acquire other body read lock".to_string())
                    })?;

                    let displacement = other_body.position - body.position;
                    let distance_squared = displacement.length_squared();

                    if distance_squared > 0.0 {
                        let distance = distance_squared.sqrt();
                        let force_magnitude = GRAVITATIONAL_CONSTANT * other_body.mass
                            / (distance_squared * distance);

                        acceleration += displacement * force_magnitude;
                    }
                }
            }

            new_accelerations.push(acceleration);
        }

        // Phase 3: Update velocities using average of old and new accelerations
        for (body_ref, new_acceleration) in body_refs.iter().zip(new_accelerations.iter()) {
            let mut body = body_ref.write().map_err(|_| {
                AstrariaError::Physics("Failed to acquire body write lock".to_string())
            })?;

            // Update velocity: v(t+dt) = v(t) + 0.5*(a(t) + a(t+dt))*dt
            body.velocity =
                body.velocity + (body.acceleration + *new_acceleration) * (0.5 * delta_time);

            // Store new acceleration for next timestep
            body.acceleration = *new_acceleration;
            body.acceleration_initialized = false; // Reset for next iteration
        }

        Ok(())
    }

    pub fn stop_simulation(&mut self) {
        self.terminate_flag.store(true, Ordering::Relaxed);

        if let Some(handle) = self.thread_handle.take() {
            if let Err(e) = handle.join() {
                log::error!("Failed to join physics thread: {e:?}");
            }
        }
    }

    pub fn add_body(&self, body: Body) -> AstrariaResult<()> {
        let mut bodies = self
            .bodies
            .write()
            .map_err(|_| AstrariaError::Physics("Failed to acquire write lock".to_string()))?;

        bodies.add_body(body);
        Ok(())
    }

    pub fn get_bodies(&self) -> AstrariaResult<Vec<Body>> {
        let bodies = self
            .bodies
            .read()
            .map_err(|_| AstrariaError::Physics("Failed to acquire read lock".to_string()))?;

        let mut result = Vec::new();

        for body_ref in bodies.bodies() {
            let body = body_ref.read().map_err(|_| {
                AstrariaError::Physics("Failed to acquire body read lock".to_string())
            })?;
            result.push(body.clone());
        }

        Ok(result)
    }

    pub fn set_simulation_speed(&self, speed: f32) -> AstrariaResult<()> {
        let mut sim_speed = self
            .simulation_speed
            .write()
            .map_err(|_| AstrariaError::Physics("Failed to acquire write lock".to_string()))?;

        *sim_speed = speed.max(0.0); // Ensure non-negative speed
        Ok(())
    }

    pub fn get_simulation_speed(&self) -> AstrariaResult<f32> {
        let sim_speed = self
            .simulation_speed
            .read()
            .map_err(|_| AstrariaError::Physics("Failed to acquire read lock".to_string()))?;

        Ok(*sim_speed)
    }
}

impl Drop for VelocityVerlet {
    fn drop(&mut self) {
        self.stop_simulation();
    }
}

/// Main physics simulation coordinator
pub struct PhysicsSimulation {
    algorithm: VelocityVerlet,
}

impl PhysicsSimulation {
    pub fn new() -> Self {
        Self {
            algorithm: VelocityVerlet::new(),
        }
    }

    pub fn start(&mut self) -> AstrariaResult<()> {
        self.algorithm.start_simulation()
    }

    pub fn shutdown(&mut self) {
        self.algorithm.stop_simulation();
    }

    pub fn update(&mut self, _delta_time: f32) -> AstrariaResult<()> {
        // The actual physics runs on a separate thread
        // This could be used for interpolation or other per-frame updates
        Ok(())
    }

    pub fn add_body(&self, body: Body) -> AstrariaResult<()> {
        self.algorithm.add_body(body)
    }

    pub fn get_bodies(&self) -> AstrariaResult<Vec<Body>> {
        self.algorithm.get_bodies()
    }

    pub fn set_simulation_speed(&self, speed: f32) -> AstrariaResult<()> {
        self.algorithm.set_simulation_speed(speed)
    }

    pub fn get_simulation_speed(&self) -> AstrariaResult<f32> {
        self.algorithm.get_simulation_speed()
    }

    pub fn load_scenario(&mut self, scenario_data: String) -> AstrariaResult<()> {
        use crate::math::Body;
        use crate::scenario::ScenarioParser;

        // Parse the scenario file
        let scenario = ScenarioParser::parse(&scenario_data)?;

        if scenario.bodies.is_empty() {
            log::warn!("No bodies found in scenario, creating test scenario");
            return self.create_test_scenario();
        }

        log::info!("Loading scenario with {} bodies", scenario.bodies.len());

        // Clear existing bodies
        {
            let mut bodies = self.algorithm.bodies.write().map_err(|_| {
                AstrariaError::Physics("Failed to acquire write lock on bodies".to_string())
            })?;
            *bodies = crate::math::BodyCollection::new();
        }

        // Add bodies from scenario
        for scenario_body in scenario.bodies {
            let body = Body::new_with_properties(
                scenario_body.mass,
                scenario_body.position,
                scenario_body.velocity,
                scenario_body.name.clone(),
                scenario_body.body_type,
                scenario_body.orbit_color,
                scenario_body.rotation_params,
            );

            log::info!(
                "Adding body: {} (mass: {:.2e} kg)",
                scenario_body.name,
                scenario_body.mass
            );
            self.add_body(body)?;
        }

        // Update the body collection to move bodies from pending_additions to the main bodies vector
        {
            let mut bodies = self.algorithm.bodies.write().map_err(|_| {
                AstrariaError::Physics("Failed to acquire write lock on bodies".to_string())
            })?;
            bodies.update_collection();
        }

        // Start the simulation
        self.start()?;

        log::info!("Scenario loaded successfully");
        Ok(())
    }

    fn create_test_scenario(&mut self) -> AstrariaResult<()> {
        use crate::math::{AU_TO_METERS, SOLAR_MASS};
        use glam::DVec3;

        // Create a simple Sun-Earth system for testing
        let sun = Body::new(SOLAR_MASS, DVec3::ZERO, DVec3::ZERO);

        let earth = Body::new(
            5.972e24, // Earth mass in kg
            DVec3::new(AU_TO_METERS, 0.0, 0.0),
            DVec3::new(0.0, 29780.0, 0.0), // Earth orbital velocity in m/s
        );

        self.add_body(sun)?;
        self.add_body(earth)?;

        // Start the simulation
        self.start()?;

        log::info!("Created test Sun-Earth scenario");
        Ok(())
    }
}

impl Default for PhysicsSimulation {
    fn default() -> Self {
        Self::new()
    }
}
