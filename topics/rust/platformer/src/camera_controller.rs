use crate::camera::{Camera, FORWARD};
use winit::event::{
    WindowEvent,
    KeyboardInput,
    VirtualKeyCode,
    ElementState,
};
use cgmath::InnerSpace;
use std::time::Duration;

pub struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false
        }
    }

    pub fn update_camera(&self, camera: &mut Camera, delta_frame: Duration) {
        let up_norm = camera.up.normalize();
        let relative_speed = delta_frame.as_secs_f32() * self.speed;

        if !(self.is_up_pressed && self.is_down_pressed) {
            if self.is_up_pressed {
                camera.eye += up_norm * relative_speed;
            }
            if self.is_down_pressed {
                camera.eye -= up_norm * relative_speed;
            }
        }

        let forward = FORWARD;
        let right = forward.cross(camera.up);

        if !(self.is_left_pressed && self.is_right_pressed) {
            if self.is_right_pressed {
                camera.eye = camera.eye + (right * relative_speed);
            }

            if self.is_left_pressed {
                camera.eye = camera.eye  + (-right * relative_speed);
            }
        }
    }
}
