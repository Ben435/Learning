use cgmath::{Point3,Vector3};


pub struct IntersectResult {
    pub distance: f32,
    pub point: Point3<f32>,
    pub normal: Vector3<f32>,
}

pub trait Intersectable {
    fn intersect(&self, ray_origin: Vector3<f32>, ray_direction: Vector3<f32>) -> Option<IntersectResult>;
}
