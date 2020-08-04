use crate::render::mesh::Mesh;
use crate::render::GlShader;
use specs::{prelude::*, Component};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable3D {
    pub mesh: Mesh,
    pub shader: GlShader,
}

pub fn register(ecs: &mut World) {
    ecs.register::<Pos>();
    ecs.register::<Renderable3D>();
}
