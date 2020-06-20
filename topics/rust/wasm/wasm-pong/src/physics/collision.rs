use crate::physics::*;

pub trait CollideWith<T> {
    // Returns new position and velocity.
    // Position will only be updated to correct for polygon overlaps (common at low fps or after lag spikes)
    fn collision(&self, other: T, movement: Velocity) -> (Point, Velocity);
}
