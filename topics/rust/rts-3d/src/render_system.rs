use specs::prelude::*;
use crate::render::{SimpleRenderer,GlMesh};
use crate::camera::Camera;
use crate::components::*;
use cgmath::{prelude::*,Matrix4,vec3};

pub struct RenderSystem {}

impl <'a> System<'a> for RenderSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, SimpleRenderer<GlMesh>>,
        ReadExpect<'a, Camera>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Rot>,
        ReadStorage<'a, Renderable3D>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, renderer, camera, positions, rotations, renderables) = data;

        let mut ctx = renderer.begin();
        for (entity, rend) in (&entities, &renderables).join() {
            for mesh in rend.model.meshes.iter() {
                let pos = positions.get(entity);
                let rot = rotations.get(entity);
                let transform = Matrix4::<f32>::from_value(1.0);
                let transform = match pos {
                    Some(t) => transform * Matrix4::from_translation(vec3(t.x, t.y, t.z)),
                    None => transform,
                };
                let transform = match rot {
                    Some(r) => transform * Matrix4::from(r.quaternion),
                    None => transform,
                };

                ctx.submit(&mesh, transform, &rend.shader);
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            ctx.present(&camera);
        }
    }
}
