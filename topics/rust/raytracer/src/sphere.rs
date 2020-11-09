use cgmath::Point3;

pub struct Sphere {
    origin: Point3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32) -> Self {
        Sphere {
            origin: Point3::new(x, y, z),
            radius,
        }
    }
}
