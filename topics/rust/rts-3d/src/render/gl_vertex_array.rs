use gl;

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

    pub fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.gl_vao);
        }
    }

    pub fn unbind(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
