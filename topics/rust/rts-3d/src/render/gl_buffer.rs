use gl;
use std::mem::size_of;
use std::ffi::c_void;
use log::error;
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

    pub fn with_capacity(size: usize) -> GlBuffer {
        let mut res = GlBuffer {
            buffer_id: 0,
            components: 0,
        };

        unsafe {
            gl::GenBuffers(1, &mut res.buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, res.buffer_id);

            gl::BufferData(
                gl::ARRAY_BUFFER, 
                size as isize * size_of::<Vertex>() as isize, 
                std::ptr::null(),
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
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

    // TODO: Maybe make a smart pointer wrapper, so can auto-unmap on drop.
    pub fn map_buffer(&mut self, mode: gl::types::GLenum) -> *mut Vertex {
        unsafe {
            self.bind();
            let ptr = gl::MapBuffer(self.buffer_id, mode) as *mut Vertex;
            self.unbind();

            ptr
        }
    }

    pub fn unmap_buffer(&mut self) {
        unsafe {
            let res = gl::UnmapBuffer(gl::ARRAY_BUFFER);

            if res != gl::TRUE {
                error!("MapBuffer didn't return GL_TRUE? '{}'", res);
                panic!("UnMapBuffer returned non-true, TODO: re-init buffer when this happens")
            }
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
