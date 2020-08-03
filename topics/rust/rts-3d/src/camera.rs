use cgmath::{Vector3,Point3,vec3,Zero,Matrix4,Deg,perspective};
use cgmath::prelude::*;

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
    pub viewport_height: u32,
    pub viewport_width: u32,
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
            viewport_height: 600,
            viewport_width: 800,
        };
        camera.update_camera_vectors();

        camera
    }
}

impl Camera {
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        perspective(Deg(self.zoom), self.viewport_width as f32 / self.viewport_height as f32, 0.1, 100.0)
    }

    pub fn transform(&mut self, transform: Matrix4<f32>) {
        self.position = transform.transform_point(self.position);

        self.update_camera_vectors();
    }

    pub fn update_scroll(&mut self, yoffset: f32) {
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
