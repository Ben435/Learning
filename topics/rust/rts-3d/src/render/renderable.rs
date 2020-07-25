use super::gl_buffer::GlBuffer;
use super::gl_index_buffer::GlIndexBuffer;

pub type Vertex = cgmath::Vector3<gl::types::GLfloat>;
pub type Index = u16;

pub trait Renderable {
    fn get_vbo(&self) -> &GlBuffer;
    fn get_ebo(&self) -> &GlIndexBuffer;
}
