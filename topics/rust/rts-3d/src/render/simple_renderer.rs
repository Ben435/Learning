use gl;
use log::error;
use cgmath;
use std::mem::size_of;

use super::renderable::Renderable;

const MAX_VERTICES: usize = 10_000;

pub struct SimpleRenderer<'a, T : Renderable> {
    // gl_buffer: gl::
    gl_vbo: gl::types::GLuint,
    mem_vbo: [&'a T; MAX_VERTICES],
    gl_buffer: *mut T,
}

impl <'a, T: Renderable> SimpleRenderer<'a, T> {

    pub fn new(init: &'a T) -> SimpleRenderer<'a, T> {
        let mut res = SimpleRenderer::<'a, T>{
            gl_vbo: 0,
            mem_vbo: [init; MAX_VERTICES * size_of::<T>()],
            gl_buffer: std::ptr::null_mut() as *mut T,
        };

        res.init();

        res
    }

    pub fn init(&mut self) {
        unsafe {
            gl::GenBuffers(1, &mut self.gl_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of::<cgmath::Vector3<f32>>() as isize,
                std::ptr::null(),   // Will overwrite.
                gl::DYNAMIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn begin(&mut self) {
        self.gl_buffer = unsafe {
            gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut T
        };
    }

    pub fn submit(&mut self, renderable: &'a T) {

    }

    pub fn end(&mut self) {
        let res = unsafe {
            gl::UnmapBuffer(gl::ARRAY_BUFFER)
        };

        if res != gl::TRUE {
            error!("MapBuffer didn't return GL_TRUE? '{}'", res);
            panic!("UnMapBuffer returned non-true, TODO: re-init buffer when this happens")
        }
    }

    pub fn present(&mut self) {
        
    }
}

impl <'a, T: Renderable + Sized> Drop for SimpleRenderer<'a, T> {
    fn drop(&mut self) {

    }
}
