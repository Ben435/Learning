use gl;
use std::mem::size_of;
use std::ffi::c_void;
use log::error;
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

    pub fn with_capacity(size: usize) -> GlIndexBuffer {
        let mut res = GlIndexBuffer {
            buffer_id: 0,
            components: 0,
        };

        unsafe {
            gl::GenBuffers(1, &mut res.buffer_id);
            res.bind();

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                size as isize * size_of::<Index>() as isize, 
                std::ptr::null(),
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

    // TODO: Maybe make a smart pointer wrapper, so can auto-unmap on drop.
    pub fn map_buffer(&mut self, mode: gl::types::GLenum) -> *mut Index {
        unsafe {
            self.bind();
            let ptr = gl::MapBuffer(self.buffer_id, mode) as *mut Index;
            self.unbind();

            ptr
        }
    }

    pub fn unmap_buffer(&mut self) {
        unsafe {
            let res = gl::UnmapBuffer(gl::ELEMENT_ARRAY_BUFFER);

            if res != gl::TRUE {
                error!("MapBuffer didn't return GL_TRUE? '{}'", res);
                panic!("UnMapBuffer returned non-true, TODO: re-init buffer when this happens")
            }
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
