use gl;
use std::mem::size_of;
use std::ffi::c_void;
use super::renderable::Index;

#[derive(Debug)]
pub struct GlIndexBuffer {
    gl_ibo: gl::types::GLuint,
    pub components: usize,
}

impl GlIndexBuffer {
    pub fn new(data: &[Index]) -> GlIndexBuffer {
        let mut res = GlIndexBuffer{
            gl_ibo: 0,
            components: data.len(),
        };

        unsafe {
            gl::GenBuffers(1, &mut res.gl_ibo);
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
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gl_ibo);
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
            gl::DeleteBuffers(1, &mut self.gl_ibo);
        }
    }
}
