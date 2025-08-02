use bytemuck::{Pod, Zeroable};
use glam::Vec3;
/// Lighting system management
/// Ported from the original LightSourceManager.java
use wgpu::{Buffer, Device, Queue};

use crate::{physics::PhysicsSimulation, AstrariaResult};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct PointLight {
    pub position: [f32; 3],
    pub _padding1: f32,
    pub ambient: [f32; 3],
    pub _padding2: f32,
    pub diffuse: [f32; 3],
    pub _padding3: f32,
    pub specular: [f32; 3],
    pub _padding4: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct LightingUniforms {
    pub lights: [PointLight; 8],
    pub num_lights: i32,
    pub _padding: [f32; 3],
}

pub struct LightManager {
    lights: Vec<PointLight>,
    uniform_buffer: Option<Buffer>,
    max_lights: usize,
}

impl LightManager {
    pub fn new(_device: &Device) -> AstrariaResult<Self> {
        Ok(Self {
            lights: Vec::new(),
            uniform_buffer: None,
            max_lights: 8,
        })
    }

    pub fn add_light(&mut self, position: Vec3, ambient: Vec3, diffuse: Vec3, specular: Vec3) {
        if self.lights.len() < self.max_lights {
            let light = PointLight {
                position: position.to_array(),
                _padding1: 0.0,
                ambient: ambient.to_array(),
                _padding2: 0.0,
                diffuse: diffuse.to_array(),
                _padding3: 0.0,
                specular: specular.to_array(),
                _padding4: 0.0,
            };
            self.lights.push(light);
        }
    }

    pub fn update(&mut self, _queue: &Queue, _physics: &PhysicsSimulation) -> AstrariaResult<()> {
        // TODO: Update light positions based on simulation objects
        Ok(())
    }
}
