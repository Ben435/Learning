
pub type Vertex = cgmath::Vector3<gl::types::GLfloat>;
pub type Index = u16;

pub trait Renderable {
    fn get_vertices(&self) -> &Vec<Vertex>;
    fn get_indices(&self) -> &Vec<Index>;
}
