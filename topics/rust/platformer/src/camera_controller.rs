use crate::camera::Camera;
use winit::event::{
    WindowEvent,
    KeyboardInput,
    VirtualKeyCode,
    ElementState,
};
use cgmath::InnerSpace;

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
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
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
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

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();

        if !(self.is_forward_pressed && self.is_backward_pressed) {
            if self.is_forward_pressed {
                camera.eye += forward_norm * self.speed;
            }
            if self.is_backward_pressed {
                camera.eye -= forward_norm * self.speed;
            }
        }
        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if !(self.is_left_pressed && self.is_right_pressed) {
            if self.is_right_pressed {
                camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
            }

            if self.is_left_pressed {
                camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
            }
        }
    }
}
