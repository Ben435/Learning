use cgmath::Point2;

pub struct Player {
    position: Point2<f32>,
}

impl Player {
    pub fn new(position: Point2<f32>) -> Player {
        Player {
            position
        }
    }
}
