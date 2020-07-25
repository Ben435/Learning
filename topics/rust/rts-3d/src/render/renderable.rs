use super::gl_vertex_array::GlVertexArray;
use super::gl_index_buffer::GlIndexBuffer;

pub type Vertex = cgmath::Vector3<gl::types::GLfloat>;
pub type Index = u16;

pub trait Renderable {
    fn get_vao(&self) -> &GlVertexArray;
    fn get_ebo(&self) -> &GlIndexBuffer;
}
