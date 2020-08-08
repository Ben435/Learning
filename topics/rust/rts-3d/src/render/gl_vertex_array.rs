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

    /// Assume interleaved vec3's of GLfloats (yeap, a big assumption)
    pub fn add_interleaved_buffer(&mut self, buf: GlBuffer) {
        self.bind();
        buf.bind();
        
        let stride: i32 = 6 * std::mem::size_of::<gl::types::GLfloat>() as i32;

        unsafe {
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<gl::types::GLfloat>()) as *mut gl::types::GLvoid);
            gl::EnableVertexAttribArray(1);
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

impl Drop for GlVertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.gl_vao);
        }
    }
}
