/// Input handling system
/// Processes keyboard and mouse input for camera controls and UI interaction
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
// Note: KeyEvent and keyboard module don't exist in winit 0.28
use std::collections::HashMap;

use crate::AstrariaResult;

pub struct InputHandler {
    mouse_pressed: bool,
    last_mouse_pos: (f32, f32),
    _mouse_sensitivity: f32,
    keys_pressed: HashMap<KeyCode, bool>,
    mouse_delta: Option<(f32, f32)>,
    scroll_delta: Option<f32>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
            _mouse_sensitivity: 0.1,
            keys_pressed: HashMap::new(),
            mouse_delta: None,
            scroll_delta: None,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) -> AstrariaResult<bool> {
        match event {
            WindowEvent::KeyboardInput { event, .. } => self.handle_keyboard_input(event),
            WindowEvent::MouseInput { state, button, .. } => {
                self.handle_mouse_input(*state, *button)
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.handle_mouse_movement(position.x as f32, position.y as f32)
            }
            WindowEvent::MouseWheel { delta, .. } => self.handle_scroll(delta),
            _ => Ok(false),
        }
    }

    fn handle_keyboard_input(
        &mut self,
        event: &winit::event::KeyEvent,
    ) -> AstrariaResult<bool> {
        let pressed = event.state == ElementState::Pressed;

        if let PhysicalKey::Code(keycode) = event.physical_key {
            // Store key state
            self.keys_pressed.insert(keycode, pressed);

            match keycode {
                KeyCode::KeyW
                | KeyCode::KeyS
                | KeyCode::KeyA
                | KeyCode::KeyD
                | KeyCode::Space
                | KeyCode::ShiftLeft => {
                    Ok(true) // Camera movement keys handled
                }
                KeyCode::KeyE => {
                    // Roll right (handled in renderer)
                    Ok(true)
                }
                KeyCode::KeyQ => {
                    // Roll left (handled in renderer)
                    Ok(true)
                }
                KeyCode::ArrowUp => {
                    if pressed {
                        // Increase camera speed (simulate scroll up)
                        self.scroll_delta = Some(1.0);
                    }
                    Ok(true)
                }
                KeyCode::ArrowDown => {
                    if pressed {
                        // Decrease camera speed (simulate scroll down)
                        self.scroll_delta = Some(-1.0);
                    }
                    Ok(true)
                }
                KeyCode::ArrowLeft => {
                    // TODO: Decrease simulation speed
                    Ok(true)
                }
                KeyCode::ArrowRight => {
                    // TODO: Increase simulation speed
                    Ok(true)
                }
                KeyCode::KeyH => {
                    if pressed {
                        // TODO: Toggle UI visibility
                    }
                    Ok(true)
                }
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    fn handle_mouse_input(
        &mut self,
        state: ElementState,
        button: MouseButton,
    ) -> AstrariaResult<bool> {
        match button {
            MouseButton::Right => {
                let pressed = state == ElementState::Pressed;
                log::debug!(
                    "Right mouse button: {} (was: {})",
                    if pressed { "PRESSED" } else { "RELEASED" },
                    self.mouse_pressed
                );
                self.mouse_pressed = pressed;
                Ok(true)
            }
            MouseButton::Left => {
                // TODO: Handle object selection
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_mouse_movement(&mut self, x: f32, y: f32) -> AstrariaResult<bool> {
        let delta_x = x - self.last_mouse_pos.0;
        let delta_y = y - self.last_mouse_pos.1;

        // Only store mouse delta for camera look when right mouse button is pressed (like Java)
        if self.mouse_pressed && (delta_x.abs() > 0.1 || delta_y.abs() > 0.1) {
            // Pass raw pixel deltas to camera (Java behavior - no scaling here)
            let mouse_delta = (-delta_x, delta_y);
            log::debug!(
                "Mouse movement: delta=({:.2}, {:.2}) -> camera_delta=({:.2}, {:.2})",
                delta_x,
                delta_y,
                mouse_delta.0,
                mouse_delta.1
            );
            self.mouse_delta = Some(mouse_delta);
        }

        self.last_mouse_pos = (x, y);
        Ok(self.mouse_pressed) // Only consume if right mouse is pressed
    }

    fn handle_scroll(&mut self, delta: &winit::event::MouseScrollDelta) -> AstrariaResult<bool> {
        let scroll_amount = match delta {
            winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
            winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
        };

        // Store scroll amount to be processed by camera later
        self.scroll_delta = Some(scroll_amount);

        Ok(true)
    }

    pub fn update(&mut self, _delta_time: f32) {
        // TODO: Update continuous input (like held keys)
        // TODO: Update camera movement based on held keys
    }

    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: &KeyCode) -> bool {
        self.keys_pressed.get(key).copied().unwrap_or(false)
    }

    /// Get and consume mouse delta for camera look
    pub fn take_mouse_delta(&mut self) -> Option<(f32, f32)> {
        self.mouse_delta.take()
    }

    /// Get and consume scroll delta for camera speed adjustment
    pub fn take_scroll_delta(&mut self) -> Option<f32> {
        self.scroll_delta.take()
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}
