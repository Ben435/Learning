use gl;
use super::gl_buffer::GlBuffer;

#[derive(Debug)]
pub struct GlVertexArray {
    gl_vao: gl::types::GLuint
}

impl GlVertexArray {
    pub fn new() -> GlVertexArray {
        let mut gl_vao = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut gl_vao);
        }

        GlVertexArray{
            gl_vao,
        }
    }

    pub fn add_buffer(&mut self, buf: GlBuffer, index: gl::types::GLuint) {
        self.bind();
        buf.bind();

        unsafe {
            gl::VertexAttribPointer(index, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(index);
        }

        buf.unbind();
        self.unbind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.gl_vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
