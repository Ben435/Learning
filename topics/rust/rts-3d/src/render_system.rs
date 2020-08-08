use specs::prelude::*;
use crate::render::{SimpleRenderer,GlMesh};
use crate::camera::Camera;
use crate::components::*;

pub struct RenderSystem {}

impl <'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadExpect<'a, SimpleRenderer<GlMesh>>,
        ReadExpect<'a, Camera>,
        ReadStorage<'a, Pos>, 
        ReadStorage<'a, Renderable3D>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (renderer, camera, positions, renderables) = data;

        let mut ctx = renderer.begin();
        for (_pos, rend) in (&positions, &renderables).join() {
            ctx.submit(&rend.mesh, &rend.shader);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            ctx.present(&camera);
        }
    }
}
