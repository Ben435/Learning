use cgmath::{Vector3,vec3,Matrix4};
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
        
        Sprite::from_normalized_vertices(vertices, shader, position, scale)
    }

    pub fn from_vertices(vertices: Vec<Vector3<GLfloat>>, shader: &'a GlShader, position: Vector3<GLfloat>, scale: GLfloat) -> Sprite<'a> {
        // Need to nroamlize vertices to be within 0-1
        // let mut x_min = f32::MAX;
        // let mut y_min = f32::MAX;
        // let mut z_min = f32::MAX;
        // let mut x_max = f32::MIN;
        // let mut y_max = f32::MIN;
        // let mut z_max = f32::MIN;

        // vertices.iter().for_each(|v| {
        //     if v.x < x_min {
        //         x_min = v.x;
        //     }
        //     if v.x > x_max {
        //         x_max = v.x;
        //     };
        //     if v.y < y_min {
        //         y_min = v.y;
        //     }
        //     if v.y > y_max {
        //         y_max = v.y;
        //     };
        //     if v.z < z_min {
        //         z_min = v.z;
        //     }
        //     if v.z > z_max {
        //         z_max = v.z;
        //     };
        // });
        // let x_scaler = |x| {
        //     (x - x_min) / (x_max - x_min)
        // };
        // let y_scaler = |y| {
        //     (y - y_min) / (y_max - y_min)
        // };
        // let z_scaler = |z| {
        //     (z - z_min) / (z_max - z_min)
        // };
        // let normalized_vertices = vertices
        //     .iter()
        //     .map(|v| {
        //         vec3(x_scaler(v.x), y_scaler(v.y), z_scaler(v.z))
        //     }).collect();

        // Sprite::from_normalized_vertices(normalized_vertices, shader, position, scale)
        Sprite::from_normalized_vertices(vertices, shader, position, scale)
    }

    pub fn from_normalized_vertices(vertices: Vec<Vector3<GLfloat>>, shader: &'a GlShader, position: Vector3<GLfloat>, scale: GLfloat) -> Sprite<'a> {
        let vbo = GlBuffer::new(&vertices);
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
