use cgmath::{Matrix4,Vector3,vec3};
use gl::types::GLfloat;
use super::gl_vertex_array::GlVertexArray;
use super::gl_index_buffer::GlIndexBuffer;
use super::gl_shader::GlShader;

pub type Index = u16;

pub struct Vertex {
    pub position: Vector3<GLfloat>,
    pub normal: Vector3<GLfloat>,
}

impl Vertex {
    pub fn new(position: Vector3<GLfloat>, normal: Vector3<GLfloat>) -> Vertex {
        Vertex {
            position,
            normal,
        }
    }

    pub fn from_coords(
            pos_x: GLfloat, pos_y: GLfloat, pos_z: GLfloat, 
            norm_x: GLfloat, norm_y: GLfloat, norm_z: GLfloat) -> Vertex {
        Vertex {
            position: vec3(pos_x, pos_y, pos_z),
            normal: vec3(norm_x, norm_y, norm_z),
        }
    }
}

pub trait Renderable {
    fn get_vao(&self) -> &GlVertexArray;
    fn get_ebo(&self) -> &GlIndexBuffer;
    fn get_transform(&self) -> Matrix4<GLfloat>;
}
