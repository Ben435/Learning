use cgmath::Vector3;
use gl::types::GLfloat;
use super::gl_vertex_array::GlVertexArray;
use super::gl_index_buffer::GlIndexBuffer;
use super::gl_shader::GlShader;

pub type Vertex = cgmath::Vector3<GLfloat>;
pub type Index = u16;

pub trait Renderable {
    fn get_vao(&self) -> &GlVertexArray;
    fn get_ebo(&self) -> &GlIndexBuffer;
    fn get_shader(&self) -> &GlShader;
    fn get_position(&self) -> &Vector3<GLfloat>;
    fn get_size(&self) -> &Vector3<GLfloat>;
}
