use cgmath::{vec3};
use super::renderable::{Renderable,Vertex,Index};

#[derive(Debug)]
pub struct Sprite {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Sprite {
    pub fn square() -> Sprite {
        Sprite{
            vertices: vec![
                vec3(0.5, 0.5, 0.0),
                vec3(0.5, -0.5, 0.0),
                vec3(-0.5, -0.5, 0.0),
                vec3(-0.5, 0.5, 0.0),
            ],
            indices: vec![
                0, 1, 3,
                1, 2, 3,
            ]
        }
    }
}

impl Default for Sprite {
    fn default() -> Sprite {
        Sprite{
            vertices: vec![vec3(0.0, 0.0, 0.0)],
            indices: vec![0, 1, 2],
        }
    }
}

impl Renderable for Sprite {
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn get_indices(&self) -> &Vec<Index> {
        &self.indices
    }
}
