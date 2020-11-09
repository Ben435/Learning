mod texture;
mod window;
mod sphere;

use sphere::Sphere;
use futures::executor::block_on;
use std::f32::consts::PI;
use image::Rgba;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const NUMS_PER_PIXEL: u32 = 4; // rgba

pub struct Scene {
    spheres: Vec<Sphere>
}

impl Scene {
    pub fn new(spheres: Vec<Sphere>) -> Self {
        Scene {
            spheres
        }
    }

    pub fn render(&self, width: u32, height: u32, fov: f32) -> Vec<u8> {
        let pixel_buffer: Vec<u8> = vec![128; (WIDTH * HEIGHT * NUMS_PER_PIXEL) as usize];

        let width = width as f32;
        let height = height as f32;
        let inv_width = 1.0 / width;
        let inv_height = 1.0 / height;

        let aspect_ratio = width / height;
        let angle =(PI * 0.5 * fov / 180.0).tan();


        // how
        // pixel_buffer
        //     .into_iter()
        //     .flat_map(|pxl| pxl.into_vec())
        //     .collect::<Vec<u8>>()

        pixel_buffer
    }
}

fn main() {
    env_logger::init();
    const FOV: f32 = 30.0;

    let spheres = vec![Sphere::new(0.0, 0.0, 0.0, 5.0)];
    let scene = Scene::new(spheres);

    let image_buffer = scene.render(WIDTH, HEIGHT, FOV);

    block_on(window::render_texture(image_buffer, WIDTH, HEIGHT));
}
