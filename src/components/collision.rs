use collider::geom::Shape;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Collision {
  pub bounds: Vec<Shape>,
}

impl Component for Collision {
  type Storage = VecStorage<Collision>;
}
