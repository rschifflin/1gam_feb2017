use collider::geom::Shape;
use collider::Hitbox;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Collision {
  pub bounds: Shape,
}

impl Component for Collision {
  type Storage = VecStorage<Collision>;
}
