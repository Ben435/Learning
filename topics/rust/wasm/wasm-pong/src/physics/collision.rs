use crate::physics::Velocity;

pub trait CollideWith<T> {
    fn collision(&self, other: T, movement: Velocity) -> Velocity;
}
