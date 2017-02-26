use specs::{Component, VecStorage};

#[derive(Debug, Default, Copy, Clone)]
pub struct Position {
  pub x: f64,
  pub y: f64
}

impl Component for Position {
  type Storage = VecStorage<Position>;
}
