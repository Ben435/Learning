use gl;
use std::mem::size_of;
use std::ffi::c_void;
use super::renderable::Vertex;

#[derive(Debug)]
pub struct GlBuffer {
    pub buffer_id: gl::types::GLuint,
    pub components: usize,
}

impl GlBuffer {
    pub fn new(data: &[Vertex]) -> GlBuffer {
        let mut res = GlBuffer{
            buffer_id: 0,
            components: data.len(),
        };

        unsafe {
            gl::GenBuffers(1, &mut res.buffer_id);
            res.bind();

            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (data.len() * size_of::<Vertex>()) as isize, 
                data.as_ptr() as *mut c_void,
                gl::STATIC_DRAW
            );

            res.unbind();
        }

        res
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for GlBuffer {
    fn drop(&mut self) {
        unsafe {
            // Silently ignores dropping bound buffers, so don't worry about it
            gl::DeleteBuffers(1, &mut self.buffer_id);
        }
    }
}
