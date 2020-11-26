use cgmath::{SquareMatrix,Matrix4};
use crate::camera::Camera;


#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub struct Uniforms {
    view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into()
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_matrix().into();
    }
}
