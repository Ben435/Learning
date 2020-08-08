use cgmath::{Vector3,Quaternion,Matrix4};
use super::renderable::{Renderable,Index,Vertex};
use super::gl_buffer::GlBuffer;
use super::gl_vertex_array::GlVertexArray;
use super::gl_index_buffer::GlIndexBuffer;
use gl::types::GLfloat;

#[derive(Debug)]
pub struct GlMesh {
    vao: GlVertexArray,
    ebo: GlIndexBuffer,

    pub position: Vector3<GLfloat>,
    pub rotation: Quaternion<GLfloat>,
    pub scale: GLfloat,
}

impl GlMesh {
    /// 2D card, with front normals only (as front+back vertices causes depth fighting if "proper" 2D).
    pub fn square(position: Vector3<GLfloat>, rotation: Quaternion<GLfloat>, scale: GLfloat) -> GlMesh {
        let vertices = vec!(
            Vertex::from_coords(1.0, 1.0, 0.0, 0.0, 0.0, 1.0),    // Top right
            Vertex::from_coords(1.0, 0.0, 0.0, 0.0, 0.0, 1.0),    // Bottom right
            Vertex::from_coords(0.0, 0.0, 0.0, 0.0, 0.0, 1.0),    // Bottom left
            Vertex::from_coords(0.0, 1.0, 0.0, 0.0, 0.0, 1.0),    // Top left
        );
        let indices = vec!(
            0, 1, 3,
            1, 2, 3,
        );
        
        GlMesh::from_vertices(vertices, indices, position, rotation, scale)
    }

    pub fn cube(position: Vector3<GLfloat>, rotation: Quaternion<GLfloat>, scale: GLfloat) -> GlMesh {
        let vertices = vec!(
            Vertex::from_coords(1.0, 1.0, 0.0, 0.0, 0.0, -1.0),    // Back Top right
            Vertex::from_coords(1.0, 0.0, 0.0, 0.0, 0.0, -1.0),    // Back Bottom right
            Vertex::from_coords(0.0, 0.0, 0.0, 0.0, 0.0, -1.0),    // Back Bottom left
            Vertex::from_coords(0.0, 1.0, 0.0, 0.0, 0.0, -1.0),    // Back Top left

            Vertex::from_coords(0.0, 1.0, 1.0, 1.0, 0.0, 0.0),    // Left Top right
            Vertex::from_coords(0.0, 0.0, 1.0, 1.0, 0.0, 0.0),    // Left Bottom right
            Vertex::from_coords(0.0, 0.0, 0.0, 1.0, 0.0, 0.0),    // Left Bottom left
            Vertex::from_coords(0.0, 1.0, 0.0, 1.0, 0.0, 0.0),    // Left Top left

            Vertex::from_coords(1.0, 1.0, 0.0, -1.0, 0.0, 0.0),    // Right Top right
            Vertex::from_coords(1.0, 0.0, 0.0, -1.0, 0.0, 0.0),    // Right Bottom right
            Vertex::from_coords(1.0, 0.0, 1.0, -1.0, 0.0, 0.0),    // Right Bottom left
            Vertex::from_coords(1.0, 1.0, 1.0, -1.0, 0.0, 0.0),    // Right Top left

            Vertex::from_coords(1.0, 1.0, 0.0, 0.0, 1.0, 0.0),    // Top Top right
            Vertex::from_coords(1.0, 1.0, 1.0, 0.0, 1.0, 0.0),    // Top Bottom right
            Vertex::from_coords(0.0, 1.0, 1.0, 0.0, 1.0, 0.0),    // Top Bottom left
            Vertex::from_coords(0.0, 1.0, 0.0, 0.0, 1.0, 0.0),    // Top Top left

            Vertex::from_coords(1.0, 0.0, 1.0, 0.0, -1.0, 0.0),    // Bottom Top right
            Vertex::from_coords(1.0, 0.0, 0.0, 0.0, -1.0, 0.0),    // Bottom Bottom right
            Vertex::from_coords(0.0, 0.0, 0.0, 0.0, -1.0, 0.0),    // Bottom Bottom left
            Vertex::from_coords(0.0, 0.0, 1.0, 0.0, -1.0, 0.0),    // Bottom Top left

            Vertex::from_coords(1.0, 1.0, 1.0, 0.0, 0.0, 1.0),    // Front Top right
            Vertex::from_coords(1.0, 0.0, 1.0, 0.0, 0.0, 1.0),    // Front Bottom right
            Vertex::from_coords(0.0, 0.0, 1.0, 0.0, 0.0, 1.0),    // Front Bottom left
            Vertex::from_coords(0.0, 1.0, 1.0, 0.0, 0.0, 1.0),    // Front Top left
        );
        let indices = vec!(
            0, 1, 3,    // Back
            1, 2, 3,
            4, 5, 7,    // Left
            5, 6, 7,
            8, 9, 11,    // Right
            9,10, 11,
            12,13, 15,    // Top
            13,14, 15,
            16,17, 19,    // Bottom
            17,18, 19,
            20, 21, 23,    // Front
            21, 22, 23,
        );
        GlMesh::from_vertices(vertices, indices, position, rotation, scale)
    }

    pub fn from_vertices(vertices: Vec<Vertex>, indices: Vec<Index>, position: Vector3<GLfloat>, rotation: Quaternion<GLfloat>, scale: GLfloat) -> GlMesh {
        let vbo = GlBuffer::new(&vertices);
        let mut vao = GlVertexArray::new();
        vao.add_interleaved_buffer(vbo);

        GlMesh{
            vao,
            ebo: GlIndexBuffer::new(&indices),
            position,
            rotation,
            scale,
        }
    }
}

impl <'a> Renderable for GlMesh {
    fn get_vao(&self) -> &GlVertexArray {
        &self.vao
    }

    fn get_ebo(&self) -> &GlIndexBuffer {
        &self.ebo
    }

    fn get_transform(&self) -> Matrix4<GLfloat> {
        Matrix4::from_translation(self.position) * Matrix4::from(self.rotation) * Matrix4::from_scale(self.scale)
    }
}
