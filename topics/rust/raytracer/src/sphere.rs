use cgmath::{prelude::*,Point3,Vector3};
use image::Rgba;

#[derive(PartialEq)]
pub struct Sphere {
    pub origin: Point3<f32>,
    pub radius: f32,

    pub surface_color: Rgba<u8>,
    pub emmission_color: Option<Rgba<u8>>
}

impl Sphere {
    // TODO: Builder
    pub fn new(x: f32, y: f32, z: f32, radius: f32, surface_color: Rgba<u8>, emmission_color: Option<Rgba<u8>>) -> Self {
        Sphere {
            origin: Point3::new(x, y, z),
            radius,
            surface_color,
            emmission_color,
        }
    }

    /// Returns closest point to ray, or None if no collision
    pub fn intersect(&self, ray_origin: Vector3<f32>, ray_direction: Vector3<f32>) -> Option<f32> {
        let length: Vector3<f32> = (self.origin - ray_origin).to_vec();

        // TODO: This can be a member var, save some calcs
        let radius2: f32 = self.radius * self.radius;

        let tca = length.dot(ray_direction);
        if tca < 0.0 {
            return None;
        }
        let d2 = length.dot(length) - tca * tca;
        if d2 > radius2 {
            return None;
        }

        let thc = (radius2 - d2).sqrt();

        let t1 = tca + thc;
        if thc > tca {
            return Some(t1);
        }

        let t0 = tca - thc;
        return Some(t0.min(t1));

    }
}
