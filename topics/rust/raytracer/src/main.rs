mod texture;
mod window;
mod sphere;

use sphere::Sphere;
use futures::executor::block_on;
use std::f32::consts::PI;
use image::{Rgba,DynamicImage,GenericImage};

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

    pub fn render(&self, width: u32, height: u32, fov: f32) -> DynamicImage {
        let mut img = DynamicImage::new_rgba8(width, height);

        let fwidth = width as f32;
        let fheight = height as f32;
        let inv_width = 1.0 / fwidth;
        let inv_height = 1.0 / fheight;

        let aspect_ratio = fwidth / fheight;
        let angle =(PI * 0.5 * fov / 180.0).tan();

        for x in 0..width {
            for y in 0..height {
                img.put_pixel(x, y, Rgba([128, 255, 128, 255]));
            }
        }

        img
    }
}

fn main() {
    env_logger::init();
    const FOV: f32 = 30.0;

    let spheres = vec![Sphere::new(0.0, 0.0, 0.0, 5.0)];
    let scene = Scene::new(spheres);

    let img = scene.render(WIDTH, HEIGHT, FOV);

    block_on(window::render_texture(img));
}
