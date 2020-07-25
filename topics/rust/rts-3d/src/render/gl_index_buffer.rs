use gl;
use std::mem::size_of;
use std::ffi::c_void;
use super::renderable::Index;

#[derive(Debug)]
pub struct GlIndexBuffer {
    pub buffer_id: gl::types::GLuint,
    pub components: usize,
}

impl GlIndexBuffer {
    pub fn new(data: &[Index]) -> GlIndexBuffer {
        let mut res = GlIndexBuffer{
            buffer_id: 0,
            components: data.len(),
        };

        unsafe {
            gl::GenBuffers(1, &mut res.buffer_id);
            res.bind();

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                (data.len() * size_of::<Index>()) as isize, 
                data.as_ptr() as *mut c_void,
                gl::STATIC_DRAW
            );

            res.unbind();
        }

        res
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.buffer_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for GlIndexBuffer {
    fn drop(&mut self) {
        unsafe {
            // Silently ignores dropping bound buffers, so don't worry about it
            gl::DeleteBuffers(1, &mut self.buffer_id);
        }
    }
}
