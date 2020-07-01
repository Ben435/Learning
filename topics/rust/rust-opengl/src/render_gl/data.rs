
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32x3 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32x3 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32x3 {
        f32x3{ d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32)> for f32x3 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32x3::new(other.0, other.1, other.2)
    }
}
