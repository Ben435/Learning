use gl;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ptr;
use cgmath::{vec3,Vector3};

use super::renderable::{Renderable};
use crate::camera::Camera;

// const MAX_VERTICES: usize = 10_000;
// const MAX_VBO_SIZE: usize = MAX_VERTICES * size_of::<Vertex>();
// const MAX_IBO_SIZE: usize = 20_000;

/// Simple Renderer. 
/// Use `SimpleRenderer::begin()` to open a context, submit renderables to the context, then present.
/// This "manager" + "context" pattern helps guide the borrow checker, while still persisting the parent manager.
pub struct SimpleRenderer<T : Renderable> {
    phantom: PhantomData<T>,
    pub light_diffuse_dir: Vector3<f32>,
    pub light_diffuse: Vector3<f32>,
}

pub struct SimpleRenderContext<'a, T : Renderable> {
    queue: VecDeque<&'a T>,
    renderer: &'a SimpleRenderer<T>,
}

impl <'a, T: Renderable> SimpleRenderer<T> {
    pub fn new() -> SimpleRenderer<T> {
        let mut res = SimpleRenderer::<T>{
            phantom: PhantomData,
            light_diffuse_dir: vec3(-0.2, -1.0, -0.3),
            light_diffuse: vec3(0.5, 0.5, 0.5),
        };

        res.init();

        res
    }

    fn init(&mut self) {}

    /// Begin new render context
    /// Encapsulates lifetime around render queue
    /// TODO: Optimization available by carrying buffer from VecDeque between frames.
    pub fn begin(&'a self) -> SimpleRenderContext<'a, T> {
        SimpleRenderContext{
            queue: VecDeque::new(),
            renderer: self,
        }
    }
}

impl <'a, T : Renderable> SimpleRenderContext<'a, T> {
    pub fn submit(&mut self, renderable: &'a T) {
        self.queue.push_back(renderable);
    }

    pub fn present(&mut self, camera: &Camera) {
        let vw_matrix = camera.get_view_matrix();
        let pr_matrix = camera.get_projection_matrix();
        unsafe {
            while let Some(r) = self.queue.pop_front() {
                r.get_vao().bind();
                let ebo = r.get_ebo();
                ebo.bind();
                let shader = r.get_shader();
                shader.enable();
                shader.set_uniform_mat4("vw_matrix".to_string(), &vw_matrix);
                shader.set_uniform_mat4("pr_matrix".to_string(), &pr_matrix);
                shader.set_uniform_mat4("ml_matrix".to_string(), &r.get_transform());

                shader.set_uniform_3f("light_dir".to_string(), &self.renderer.light_diffuse_dir);
                shader.set_uniform_3f("light_diffuse".to_string(), &self.renderer.light_diffuse);

                gl::DrawElements(gl::TRIANGLES, ebo.components as i32, gl::UNSIGNED_SHORT, ptr::null());
            }
        };
    }
}
