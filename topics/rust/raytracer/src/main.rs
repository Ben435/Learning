mod texture;
mod window;
mod sphere;

use sphere::Sphere;
use futures::executor::block_on;
use std::f32::consts::PI;
use image::{Rgba,DynamicImage,GenericImage,Pixel};
use cgmath::{prelude::*,Vector3,vec3};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

// const WIDTH: u32 = 64;
// const HEIGHT: u32 = 48;

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
        
        let iter_coords = (0..width)
            .into_iter()
            .flat_map(|x| (0..height)
                .into_iter()
                .map(move |y| (x.clone(), y))
            );
        for (x, y) in iter_coords {
            let xx = (2.0 * ((x as f32 + 0.5) * inv_width) - 1.0) * angle * aspect_ratio;
            let yy = (1.0 - 2.0 * ((y as f32 + 0.5) * inv_height)) * angle;

            // TODO: Ray origin === eye position
            let ray_origin = vec3(0.0, 0.0, 0.0);
            let ray_direction = vec3(xx, yy, -1.0).normalize();

            let color = self.trace(ray_origin, ray_direction, 0);

            img.put_pixel(x, y, color);
        }

        img
    }

    fn trace(&self, ray_origin: Vector3<f32>, ray_direction: Vector3<f32>, current_depth: u8) -> Rgba<u8> {
        // TODO: This can be reduced by pre-sorting spheres, maybe do later as a pre-processing step.
        let maybe_intersect = self.spheres.iter().fold(None, |prev, sphere| {
            let intersection_result = sphere.intersect(ray_origin, ray_direction);

            match intersection_result {
                None => prev,
                Some(closest_dist) => {
                    match prev {
                        None => Some((closest_dist, sphere)),
                        Some((min_dist, _s)) => {
                            if closest_dist < min_dist {
                                return Some((closest_dist, sphere))
                            }
                            prev
                        }
                    }
                },
            }
        });

        match maybe_intersect {
            None => Rgba([0, 0, 0, 255]), // Missed scene, background goes here.
            Some((dist, sphere)) => {
                let point = ray_origin + ray_direction * dist;
                let normal = (point - sphere.origin.to_vec()).normalize();

                // TODO: Handle being inside an object

                let resultant_color: Rgba<u8> = self.spheres.iter()
                    .filter(|s| s.emmission_color.is_some())
                    .fold(Rgba([0, 0, 0, 255]), |result_color, light| {
                        let shadow_ray_origin = point + normal * 1e-4;
                        let shadow_ray_direction = (light.origin.to_vec() - point).normalize();
                        let in_shadow = self.spheres.iter()
                            .filter(|s| s.origin != light.origin)
                            .find(|s| s.intersect(shadow_ray_origin, shadow_ray_direction).is_some())
                            .is_some();
                        
                        let transmission = match in_shadow {
                            true => 0.0,
                            false => 1.0,
                        };

                        let addition_from_light = sphere.surface_color
                            .map(|pxl| (pxl as f32 * transmission) as u8) // This is kinda bad, casting like this is messy at best, don't like, may fix
                            .map2(&light.emmission_color.unwrap(), |pxl1, pxl2| ((pxl1 as f32) * (pxl2 as f32 / 255.0)) as u8); // Same here, horrible

                        return result_color.map2(&addition_from_light, |px1, px2| px1.saturating_add(px2));
                    });
                resultant_color
            }
        }
    }
}

fn main() {
    env_logger::init();
    const FOV: f32 = 30.0;

    let spheres = vec![
        Sphere::new(0.0, 0.0, -20.0, 4.0, Rgba([128, 255, 128, 255]), None),
        Sphere::new(0.0, 20.0, -30.0, 3.0, Rgba([255, 255, 255, 255]), Some(Rgba([255, 255, 255, 255]))),
    ];
    let scene = Scene::new(spheres);

    let img = scene.render(WIDTH, HEIGHT, FOV);

    block_on(window::render_texture(img));
}
