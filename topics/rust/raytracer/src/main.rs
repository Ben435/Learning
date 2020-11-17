mod texture;
mod window;
mod sphere;
mod intersect;

use sphere::Sphere;
use futures::executor::block_on;
use std::f32::consts::PI;
use image::{Rgba,DynamicImage,GenericImage};
use cgmath::{prelude::*,Vector3,vec3,Point3};
use intersect::Intersectable;
use std::time::Instant;
use log::{info,log_enabled,Level};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const MAX_DEPTH: u8 = 16;

pub struct Scene {
    spheres: Vec<Sphere>
}

/// Vector3<0.0 -> 1.0> to Rgba<0 -> 255>
fn vec_to_rgba(vec: Vector3<f32>) -> Rgba<u8> {
    let unit_to_pxl = |val: f32| (255.0 * val.min(1.0).max(0.0)) as u8;

    Rgba([unit_to_pxl(vec.x), unit_to_pxl(vec.y), unit_to_pxl(vec.z), 255])
}

fn mix(a: f32, b: f32, mix_ratio: f32) -> f32 {
    b * mix_ratio + a * (1.0 - mix_ratio)
}

/// Reflect ray over normal
/// Assumes normal is normalized
fn reflect(ray: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    ray - (normal * 2.0 * ray.dot(normal))
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
        let start_time = Instant::now();
        let mut prev_log_time = Instant::now();
        let total_rays_to_trace = width * height;
        info!("Beginning tracing of {} rays", total_rays_to_trace);
        for (i, (x, y)) in iter_coords.enumerate() {
            let xx = (2.0 * ((x as f32 + 0.5) * inv_width) - 1.0) * angle * aspect_ratio;
            let yy = (1.0 - 2.0 * ((y as f32 + 0.5) * inv_height)) * angle;

            // TODO: Ray origin === eye position
            let ray_origin = vec3(0.0, 0.0, 0.0);
            let ray_direction = vec3(xx, yy, -1.0).normalize();

            let vec_color = self.trace(ray_origin, ray_direction, 0);

            img.put_pixel(x, y, vec_to_rgba(vec_color));

            if log_enabled!(Level::Info) {
                let cur_time = Instant::now();
                if prev_log_time.elapsed().as_secs() > 1 {
                    info!("Traced {} / {}", i, total_rays_to_trace);
                    prev_log_time = cur_time;
                }
            }
        }

        let elapsed = start_time.elapsed();
        info!("Done in {}.{}secs", elapsed.as_secs(), elapsed.subsec_nanos());

        img
    }

    fn trace(&self, ray_origin: Vector3<f32>, ray_direction: Vector3<f32>, current_depth: u8) -> Vector3<f32> {
        // TODO: This can be reduced by pre-sorting spheres, maybe do later as a pre-processing step.
        let maybe_intersect = self.spheres.iter().fold(None, |prev, sphere| {
            let intersection_result = sphere.intersect(ray_origin, ray_direction);

            match intersection_result {
                None => prev,
                Some(current_result) => {
                    match prev {
                        None => Some((current_result, sphere)),
                        Some((prev_result, prev_sphere)) => {
                            if current_result.distance < prev_result.distance {
                                return Some((current_result, sphere))
                            }
                            Some((prev_result, prev_sphere))
                        }
                    }
                },
            }
        });

        match maybe_intersect {
            None => vec3(1.0, 1.0, 1.0), // Missed scene, background goes here.
            Some((intersection_result, sphere)) => {
                let point = intersection_result.point.to_vec();
                let (normal, inside) = {
                    let normal = intersection_result.normal;

                    if ray_direction.dot(normal) > 0.0 {
                        (-normal, true)
                    } else {
                        (normal, false)
                    }
                };
                let bias = 1e-4;

                if current_depth > MAX_DEPTH || (sphere.reflectance <= 0.0 && sphere.transmission <= 0.0) {
                    let resultant_color: Vector3<f32> = self.spheres.iter()
                        .filter(|s| !s.emmission_color.is_zero())
                        .fold(vec3(0.0, 0.0, 0.0), |result_color, light| {
                            let shadow_ray_origin = point + normal * bias;
                            let shadow_ray_direction = (light.origin.to_vec() - point).normalize();
                            let in_shadow = self.spheres.iter()
                                .filter(|s| s.origin != light.origin)
                                .find(|s| s.intersect(shadow_ray_origin, shadow_ray_direction).is_some())
                                .is_some();
                            
                            let transmission = match in_shadow {
                                true => 0.0,
                                false => 1.0,
                            };

                            let addition_from_light = (sphere.surface_color * transmission).mul_element_wise(light.emmission_color);

                            return result_color.add_element_wise(addition_from_light);
                        });
                    return resultant_color;
                }

                let facing_ratio = -ray_direction.dot(normal);
                let fresnel_effect = mix((1.0 - facing_ratio).powi(3), 1.0, 0.1);

                let reflection = match sphere.reflectance {
                    reflectance if reflectance > 0.0 => {
                        let reflection_ray_dir = reflect(ray_direction, normal).normalize();
                        let reflection_ray_origin = point + normal * bias;

                        self.trace(reflection_ray_origin, reflection_ray_dir, current_depth + 1)
                    },
                    _ => vec3(0.0, 0.0, 0.0),
                };

                let refraction = match sphere.transmission {
                    transmission if transmission > 0.0 => {
                        let ior: f32 = 1.1; // TODO: This should be a material property
                        let eta = if inside { ior } else { ior.recip() };
                        let cosi = -normal.dot(ray_direction);
                        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

                        if k > 0.0 {
                            let normal_scale_factor = eta * cosi - k.sqrt();
                            let refraction_ray_direction = ((ray_direction * eta) + (normal * normal_scale_factor)).normalize();
                            let refraction_ray_origin = point - (normal * bias);
                            self.trace(refraction_ray_origin, refraction_ray_direction, current_depth + 1)
                        } else {
                            println!("Total internal reflection occurred!");
                            // Total internal reflection
                            // 100% reflected, so we ignore
                            vec3(0.0, 0.0, 0.0)
                        }
                    },
                    _ => vec3(0.0, 0.0, 0.0),
                };

                let reflection_input = reflection * fresnel_effect;
                let refraction_input = refraction * (1.0 - fresnel_effect) * sphere.transmission;

                let result = (reflection_input + refraction_input).mul_element_wise(sphere.surface_color) + sphere.emmission_color;

                // println!("Result, {:?} {:?} {:?} {:?}", result, reflection_input, refraction_input, sphere.surface_color);

                return result;
            }
        }
    }
}

fn main() {
    env_logger::init();
    const FOV: f32 = 30.0;

    let spheres = vec![
        // Platform
        Sphere::new(Point3::new(0.0, -10004.0, -20.0), 10000.0, vec3(0.2, 0.2, 0.2), 0.0, 0.0, vec3(0.0, 0.0, 0.0)),
    
        // Objects
        Sphere::new(Point3::new(0.0, 0.0, -20.0), 4.0, vec3(0.5, 1.0, 0.5), 1.0, 0.5, vec3(0.0, 0.0, 0.0)),
        Sphere::new(Point3::new(5.0, -1.0, -15.0), 2.0, vec3(0.90, 0.76, 0.46), 1.0, 0.0, vec3(0.0, 0.0, 0.0)),
        Sphere::new(Point3::new(5.0, 0.0, -25.0), 3.0, vec3(0.65, 0.77, 0.97), 1.0, 0.0, vec3(0.0, 0.0, 0.0)),
        Sphere::new(Point3::new(-5.5, 0.0, -15.0), 3.0, vec3(0.90, 0.90, 0.90), 1.0, 0.0, vec3(0.0, 0.0, 0.0)),

        // Light
        Sphere::new(Point3::new(0.0, 20.0, -30.0), 3.0, vec3(1.0, 1.0, 1.0), 0.0, 0.0, vec3(1.0, 1.0, 1.0)),
    ];
    let scene = Scene::new(spheres);

    let img = scene.render(WIDTH, HEIGHT, FOV);

    block_on(window::render_texture(img));
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::assert_relative_eq;

    #[test]
    fn test_reflect_over_origin() {
        let dummy_vecs: Vec<Vector3<f32>> = vec!(
            vec3(1.0, 0.0, 0.0),
            vec3(1.0, 1.0, 0.0),
            vec3(-1.0, 1.0, -1.0),
        );
        let normal_vec = vec3(0.0, 0.0, 0.0);

        dummy_vecs.into_iter().for_each(|vec| {
            let reflect_vec = reflect(vec, normal_vec);
            assert_eq!(reflect_vec, vec);
        })

    }

    #[test]
    fn test_reflect_normal_is_inverse() {
        let base_vec = vec3(-1.0, 0.0, 0.0);
        let normal_vec = vec3(1.0, 0.0, 0.0).normalize();

        let reflect_vec = reflect(base_vec, normal_vec);

        assert_eq!(reflect_vec, vec3(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_reflect_normal_is_perpendicular() {
        let base_vec = vec3(1.0, 0.0, 0.0);
        let normal_vec = vec3(0.0, 1.0, 0.0).normalize();

        let reflect_vec = reflect(base_vec, normal_vec);

        assert_eq!(reflect_vec, vec3(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_reflect_simple() {
        let base_vec = vec3(-1.0, -1.0, 0.0);
        let normal_vec = vec3(0.0, 1.0, 0.0).normalize();

        let reflect_vec = reflect(base_vec, normal_vec);

        assert_eq!(reflect_vec, vec3(-1.0, 1.0, 0.0));
    }

    #[test]
    fn test_reflect_off_angle() {
        let base_vec = vec3(0.0, -1.0, 0.0);
        let normal_vec = vec3(1.0, 1.0, 0.0).normalize();

        let reflect_vec = reflect(base_vec, normal_vec);

        assert_relative_eq!(reflect_vec, vec3(1.0, 0.0, 0.0));
    }
}
