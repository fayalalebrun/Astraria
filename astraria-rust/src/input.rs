/// Input handling system
/// Processes keyboard and mouse input for camera controls and UI interaction
use winit::event::{ElementState, MouseButton, VirtualKeyCode, WindowEvent};
// Note: KeyEvent and keyboard module don't exist in winit 0.28
use std::collections::HashMap;

use crate::AstrariaResult;

pub struct InputHandler {
    mouse_pressed: bool,
    last_mouse_pos: (f32, f32),
    mouse_sensitivity: f32,
    keys_pressed: HashMap<VirtualKeyCode, bool>,
    mouse_delta: Option<(f32, f32)>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
            mouse_sensitivity: 0.1,
            keys_pressed: HashMap::new(),
            mouse_delta: None,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) -> AstrariaResult<bool> {
        match event {
            WindowEvent::KeyboardInput { input, .. } => self.handle_keyboard_input(input),
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
        input: &winit::event::KeyboardInput,
    ) -> AstrariaResult<bool> {
        let pressed = input.state == ElementState::Pressed;

        if let Some(keycode) = input.virtual_keycode {
            // Store key state
            self.keys_pressed.insert(keycode, pressed);

            match keycode {
                VirtualKeyCode::W
                | VirtualKeyCode::S
                | VirtualKeyCode::A
                | VirtualKeyCode::D
                | VirtualKeyCode::Space
                | VirtualKeyCode::LShift => {
                    Ok(true) // Camera movement keys handled
                }
                VirtualKeyCode::E => {
                    // TODO: Send to camera (roll right)
                    Ok(true)
                }
                VirtualKeyCode::Up => {
                    // TODO: Increase camera speed
                    Ok(true)
                }
                VirtualKeyCode::Down => {
                    // TODO: Decrease camera speed
                    Ok(true)
                }
                VirtualKeyCode::Left => {
                    // TODO: Decrease simulation speed
                    Ok(true)
                }
                VirtualKeyCode::Right => {
                    // TODO: Increase simulation speed
                    Ok(true)
                }
                VirtualKeyCode::H => {
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
                self.mouse_pressed = state == ElementState::Pressed;
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

        // Store mouse delta for camera look (always, not just when mouse pressed)
        if delta_x.abs() > 0.1 || delta_y.abs() > 0.1 {
            // Only if significant movement
            self.mouse_delta = Some((
                delta_x * self.mouse_sensitivity,
                -delta_y * self.mouse_sensitivity,
            ));
        }

        self.last_mouse_pos = (x, y);
        Ok(true) // Always consume mouse movement for camera
    }

    fn handle_scroll(&mut self, delta: &winit::event::MouseScrollDelta) -> AstrariaResult<bool> {
        let scroll_amount = match delta {
            winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
            winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
        };

        // TODO: Send scroll to camera for speed adjustment
        // camera.process_scroll(scroll_amount);

        Ok(true)
    }

    pub fn update(&mut self, _delta_time: f32) {
        // TODO: Update continuous input (like held keys)
        // TODO: Update camera movement based on held keys
    }

    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: &VirtualKeyCode) -> bool {
        self.keys_pressed.get(key).copied().unwrap_or(false)
    }

    /// Get and consume mouse delta for camera look
    pub fn take_mouse_delta(&mut self) -> Option<(f32, f32)> {
        self.mouse_delta.take()
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}
