use cgmath::{vec3};
use super::renderable::{Renderable};
use super::gl_buffer::GlBuffer;
use super::gl_index_buffer::GlIndexBuffer;

#[derive(Debug)]
pub struct Sprite {
    vbo: GlBuffer,
    ebo: GlIndexBuffer,
}

impl Sprite {
    pub fn square() -> Sprite {
        Sprite{
            vbo: GlBuffer::new(&[
                vec3(0.5, 0.5, 0.0),
                vec3(0.5, -0.5, 0.0),
                vec3(-0.5, -0.5, 0.0),
                vec3(-0.5, 0.5, 0.0),
            ]),
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
    fn get_vbo(&self) -> &GlBuffer {
        &self.vbo
    }

    fn get_ebo(&self) -> &GlIndexBuffer {
        &self.ebo
    }
}
