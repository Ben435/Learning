use cgmath::{vec3};
use super::renderable::{Renderable};
use super::gl_buffer::GlBuffer;
use super::gl_vertex_array::GlVertexArray;
use super::gl_index_buffer::GlIndexBuffer;

#[derive(Debug)]
pub struct Sprite {
    vao: GlVertexArray,
    ebo: GlIndexBuffer,
}

impl Sprite {
    pub fn square() -> Sprite {
        let vbo = GlBuffer::new(&[
            vec3(0.5, 0.5, 0.0),    // Top right
            vec3(0.5, -0.5, 0.0),   // Bottom right
            vec3(-0.5, -0.5, 0.0),  // Bottom left
            vec3(-0.5, 0.5, 0.0),   // Top left
        ]);
        let mut vao = GlVertexArray::new();
        vao.add_buffer(vbo, 0);

        Sprite{
            vao,
            ebo: GlIndexBuffer::new(&[
                0, 1, 3,
                1, 2, 3,
            ])
        }
    }
}

impl Default for Sprite {
    fn default() -> Sprite {
        Sprite::square()
    }
}

impl Renderable for Sprite {
    fn get_vao(&self) -> &GlVertexArray {
        &self.vao
    }

    fn get_ebo(&self) -> &GlIndexBuffer {
        &self.ebo
    }
}
