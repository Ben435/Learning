use super::gl_mesh::GlMesh;
use super::renderable::{Vertex,Index};
use cgmath::{prelude::*,Quaternion,vec3,Matrix3};
use wavefront_obj::obj::{parse,Primitive,ObjSet};

#[derive(Debug)]
pub struct GlModel {
    pub meshes: Vec<GlMesh>
}

impl GlModel {
    pub fn builder() -> GlModelBuilder {
        GlModelBuilder::new()
    }
}

pub struct GlModelBuilder {
    meshes: Vec<GlMesh>
}

impl GlModelBuilder {
    pub fn new() -> GlModelBuilder {
        GlModelBuilder {
            meshes: Vec::new(),
        }
    }

    pub fn with_mesh(mut self, mesh: GlMesh) -> GlModelBuilder {
        self.meshes.push(mesh);

        self
    }

    /// Takes meshes from contents of an obj file
    /// TODO: Proper error handling (currently panics on any issue)
    /// TODO: Handles textures (currently only verts + normals)
    pub fn with_obj_file(mut self, file_content: String) -> GlModelBuilder {
        let meshes: ObjSet = parse(file_content).unwrap();

        self.meshes.extend(meshes
            .objects
            .iter()
            .map(|obj| {
                let indices: Vec<Index> = obj.geometry
                    .iter()
                    .flat_map(|g| g.shapes.iter())
                    .map(|shape| shape.primitive)
                    .fold(Vec::new(), |mut v, primitive| {
                        let to_append = match primitive {
                            Primitive::Point(v) => [v.0, v.0, v.0],
                            Primitive::Line(v1, v2) => [v1.0, v2.0, v2.0],
                            Primitive::Triangle(v1, v2, v3) => [v1.0, v2.0, v3.0],
                        };

                        v.push(to_append[0] as Index);
                        v.push(to_append[1] as Index);
                        v.push(to_append[2] as Index);

                        v
                    });

                let verts: Vec<Vertex> = obj.vertices
                    .iter()
                    .zip(&obj.normals)
                    .map(|(v, n)| Vertex::from_coords(
                        v.x as f32, v.y as f32, v.z as f32, 
                        n.x as f32, n.y as f32, n.z as f32,
                    )).collect();

                GlMesh::from_vertices(verts, indices, vec3(0.0, 0.0, 0.0), Quaternion::from(Matrix3::from_value(0.0)), 1.0)
            }));

        self
    }

    pub fn build(mut self) -> GlModel {
        GlModel {
            meshes: self.meshes,
        }
    }
}
