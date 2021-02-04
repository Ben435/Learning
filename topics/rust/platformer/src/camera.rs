use cgmath::{Point3,Vector3,Matrix4};


pub const FORWARD: Vector3<f32> = Vector3{ x: 0.0, y: 0.0, z: -1.0 };

pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub eye: Point3<f32>,
    pub up: Vector3<f32>,
    width: f32,
    height: f32,
    half_width: f32,
    half_height: f32,
    znear: f32,
    zfar: f32
}

impl Camera {
    pub fn new(eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>, aspect: f32, znear: f32, zfar: f32) -> Camera {
        let width = aspect;
        let height = 1.0;
        Camera {
            eye,
            up,
            width,
            height,
            half_width: width/2.0,
            half_height: height/2.0,
            znear,
            zfar
        }
    }

    pub fn build_view_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at(self.eye, self.eye + FORWARD, self.up);
        let projection = cgmath::ortho(-self.half_width, self.half_width, -self.half_height, self.half_height, self.znear, self.zfar);
        
        return OPENGL_TO_WGPU_MATRIX * projection * view;
    }
}
