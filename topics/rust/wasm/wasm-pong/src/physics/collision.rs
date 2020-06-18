
pub trait CollideWith<T> {
    fn collision(&self, other: T) -> Option<CollisionType>;
}

// CollisionType: Describes collision.
pub enum CollisionType {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}
