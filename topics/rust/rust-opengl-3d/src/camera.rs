use cgmath::{Vector3,Point3,vec3,Zero,Matrix4};
use cgmath::prelude::*;

// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
#[derive(PartialEq, Clone, Copy)]
pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

// Default camera values
const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
    // Camera Attributes
    pub position: Point3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub world_up: Vector3<f32>,
    // Euler Angles
    pub yaw: f32,
    pub pitch: f32,
    // Camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        let mut camera = Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            front: vec3(0.0, 0.0, -1.0),
            up: Vector3::zero(), // initialized later
            right: Vector3::zero(), // initialized later
            world_up: Vector3::unit_y(),
            yaw: YAW,
            pitch: PITCH,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVTY,
            zoom: ZOOM,
        };
        camera.update_camera_vectors();

        camera
    }
}

impl Camera {
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        
        if direction == CameraMovement::FORWARD {
            self.position += self.front * velocity;
        }

        if direction == CameraMovement::BACKWARD {
            self.position += -(self.front * velocity);
        }

        if direction == CameraMovement::RIGHT {
            self.position += self.right * velocity;
        }

        if direction == CameraMovement::LEFT {
            self.position += -(self.right * velocity);
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_movements(&mut self, xoffset: f32, yoffset: f32, constrain_pitch: bool) {
        let xoffset = xoffset * self.mouse_sensitivity;
        let yoffset = yoffset * self.mouse_sensitivity;

        self.yaw += xoffset;

        if constrain_pitch {
            self.pitch = (self.pitch + yoffset).min(89.0).max(-89.0);
        } else {
            self.pitch += yoffset;
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom = (self.zoom - yoffset).max(1.0).min(45.0);
        }
    }

    pub fn update_camera_vectors(&mut self) {
        let front = vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );

        self.front = front.normalize();

        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}
