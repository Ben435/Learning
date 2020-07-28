use cgmath::{Vector3,vec3,Matrix4};
use super::renderable::{Renderable,Index,Vertex};
use super::gl_buffer::GlBuffer;
use super::gl_vertex_array::GlVertexArray;
use super::gl_index_buffer::GlIndexBuffer;
use super::gl_shader::GlShader;
use gl::types::GLfloat;

#[derive(Debug)]
pub struct Sprite<'a> {
    vao: GlVertexArray,
    ebo: GlIndexBuffer,
    shader: &'a GlShader,

    position: Vector3<GLfloat>,
    scale: GLfloat,
}

impl <'a> Sprite<'a> {
    pub fn square(shader: &'a GlShader, position: Vector3<GLfloat>, scale: GLfloat) -> Sprite<'a> {
        let vertices = vec!(
            vec3(1.0, 1.0, 0.0),    // Top right
            vec3(1.0, 0.0, 0.0),    // Bottom right
            vec3(0.0, 0.0, 0.0),    // Bottom left
            vec3(0.0, 1.0, 0.0),    // Top left
        );
        let indices = vec!(
            0, 1, 3,
            1, 2, 3,
        );
        
        Sprite::from_vertices(vertices, indices, shader, position, scale)
    }

    pub fn from_vertices(vertices: Vec<Vertex>, indices: Vec<Index>, shader: &'a GlShader, position: Vector3<GLfloat>, scale: GLfloat) -> Sprite<'a> {
        let vbo = GlBuffer::new(&vertices);
        let mut vao = GlVertexArray::new();
        vao.add_buffer(vbo, 0);

        Sprite{
            vao,
            ebo: GlIndexBuffer::new(&indices),
            shader,
            position,
            scale,
        }
    }
}

impl <'a> Renderable for Sprite<'a> {
    fn get_vao(&self) -> &GlVertexArray {
        &self.vao
    }

    fn get_ebo(&self) -> &GlIndexBuffer {
        &self.ebo
    }

    fn get_shader(&self) -> &GlShader {
        &self.shader
    }

    fn get_transform(&self) -> Matrix4<GLfloat> {
        Matrix4::from_translation(self.position) * Matrix4::from_scale(0.9)
    }
}
