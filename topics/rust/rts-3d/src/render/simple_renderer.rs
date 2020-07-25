use gl;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ptr;
use cgmath::{vec2,Vector2,ortho,Matrix4};

use super::renderable::{Renderable};

// const MAX_VERTICES: usize = 10_000;
// const MAX_VBO_SIZE: usize = MAX_VERTICES * size_of::<Vertex>();
// const MAX_IBO_SIZE: usize = 20_000;

pub struct SimpleRenderer<'a, T : Renderable> {
    queue: VecDeque<&'a T>,
    phantom: PhantomData<T>,
    pub light_pos: Vector2<f32>,
}

impl <'a, T: Renderable> SimpleRenderer<'a, T> {

    pub fn new() -> SimpleRenderer<'a, T> {
        let mut res = SimpleRenderer::<T>{
            queue: VecDeque::new(),
            phantom: PhantomData,
            light_pos: vec2(0.0, 0.0),
        };

        res.init();

        res
    }

    fn init(&mut self) {}

    pub fn begin(&mut self) {}
    pub fn end(&mut self) {}

    /// Copy to mem_buffer, will send to GPU in `present()` call.
    pub fn submit(&mut self, renderable: &'a T) {
        self.queue.push_back(renderable);
    }

    pub fn present(&mut self) {
        let pr_matrix = ortho(0.0, 16.0, 0.0, 9.0, -1.0, 1.0);
        unsafe {
            while let Some(r) = self.queue.pop_front() {
                r.get_vao().bind();
                let ebo = r.get_ebo();
                ebo.bind();
                let shader = r.get_shader();
                shader.enable();
                shader.set_uniform_mat4("pr_matrix".to_string(), pr_matrix);
                let pos = *r.get_position();
                // let scale = *r.get_size();
                shader.set_uniform_mat4(
                    "ml_matrix".to_string(),
                    Matrix4::from_translation(pos) * Matrix4::from_scale(0.9)
                );

                shader.set_uniform_2f("light_pos".to_string(), self.light_pos);

                gl::DrawElements(gl::TRIANGLES, ebo.components as i32, gl::UNSIGNED_SHORT, ptr::null());
            }
        };
    }
}
