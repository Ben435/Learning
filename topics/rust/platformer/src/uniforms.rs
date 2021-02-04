use cgmath::{SquareMatrix,Matrix4};
use crate::camera::Camera;


#[repr(C)]
#[derive(Debug,Copy,Clone,bytemuck::Pod,bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
    pub model_proj: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
            model_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_matrix().into();
    }

    pub fn update_model_proj(&mut self, model_proj: Matrix4<f32>) {
        self.model_proj = model_proj.into();
    }
}
