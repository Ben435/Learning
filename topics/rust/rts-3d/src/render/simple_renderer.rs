use gl;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ptr;

use super::renderable::{Renderable};

// const MAX_VERTICES: usize = 10_000;
// const MAX_VBO_SIZE: usize = MAX_VERTICES * size_of::<Vertex>();
// const MAX_IBO_SIZE: usize = 20_000;

pub struct SimpleRenderer<'a, T : Renderable> {
    queue: VecDeque<&'a T>,
    vao: gl::types::GLuint,
    phantom: PhantomData<T>,
}

impl <'a, T: Renderable> SimpleRenderer<'a, T> {

    pub fn new() -> SimpleRenderer<'a, T> {
        let mut res = SimpleRenderer::<T>{
            vao: 0,
            queue: VecDeque::new(),
            phantom: PhantomData,
        };

        res.init();

        res
    }

    fn init(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
        }
    }

    pub fn begin(&mut self) {}
    pub fn end(&mut self) {}

    /// Copy to mem_buffer, will send to GPU in `present()` call.
    pub fn submit(&mut self, renderable: &'a T) {
        self.queue.push_back(renderable);
    }

    pub fn present(&mut self) {
        unsafe {
            while let Some(r) = self.queue.pop_front() {
                r.get_vao().bind();
                let ebo = r.get_ebo();
                ebo.bind();
                gl::DrawElements(gl::TRIANGLES, ebo.components as i32, gl::UNSIGNED_SHORT, ptr::null());
            }
        };
    }
}

impl <'a, T: Renderable> Drop for SimpleRenderer<'a, T> {
    fn drop(&mut self) {

    }
}
