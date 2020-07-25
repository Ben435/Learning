use cgmath::{Vector3,vec3,Vector2};
use super::renderable::{Renderable};
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
    size: Vector3<GLfloat>,
}

impl <'a> Sprite<'a> {
    pub fn square(shader: &'a GlShader, position: Vector3<GLfloat>, size: Vector2<GLfloat>) -> Sprite<'a> {
        let vbo = GlBuffer::new(&[
            vec3(1.0, 1.0, 0.0),    // Top right
            vec3(1.0, 0.0, 0.0),    // Bottom right
            vec3(0.0, 0.0, 0.0),    // Bottom left
            vec3(0.0, 1.0, 0.0),    // Top left
        ]);
        let mut vao = GlVertexArray::new();
        vao.add_buffer(vbo, 0);

        Sprite{
            vao,
            ebo: GlIndexBuffer::new(&[
                0, 1, 3,
                1, 2, 3,
            ]),
            shader,
            position,
            size: Vector3::new(size.x, size.y, 1.0),
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

    fn get_position(&self) -> &Vector3<GLfloat> {
        &self.position
    }

    fn get_size(&self) -> &Vector3<GLfloat> {
        &self.size
    }
}
