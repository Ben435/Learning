use cgmath::Point2;

pub struct Rect {
    pub origin: Point2<f32>,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            origin: Point2::new(x, y),
            width,
            height,
        }
    }
}
