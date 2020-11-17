use cgmath::{prelude::*,Point3,Vector3};
use crate::intersect::{Intersectable,IntersectResult};

#[derive(PartialEq)]
pub struct Sphere {
    pub origin: Point3<f32>,
    pub radius: f32,

    pub surface_color: Vector3<f32>,
    pub reflectance: f32,
    pub transmission: f32,
    pub emmission_color: Vector3<f32>
}

impl Sphere {
    // TODO: Builder
    pub fn new(origin: Point3<f32>, radius: f32, surface_color: Vector3<f32>, reflectance: f32, transmission: f32, emmission_color: Vector3<f32>) -> Self {
        Sphere {
            origin,
            radius,
            surface_color,
            reflectance,
            transmission,
            emmission_color,
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray_origin: Vector3<f32>, ray_direction: Vector3<f32>) -> Option<IntersectResult> {
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
        let distance = {
            if thc > tca {
                t1
            } else {
                let t0 = tca - thc;
                t0.min(t1)
            }
        };

        let v_point = ray_origin + distance * ray_direction;
        let normal = (v_point - self.origin.to_vec()).normalize();
        return Some(IntersectResult {
            distance,
            point: Point3::from_vec(v_point),
            normal,
        });
    }
}
