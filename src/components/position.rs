use specs::{Component, VecStorage};

#[derive(Debug, Default, Copy, Clone)]
pub struct Position {
  pub x: f64,
  pub y: f64
}

impl Position {
  pub fn distance_to(&self, other: &Position) -> f64 {
    let delta_x = other.x.max(self.x) - other.x.min(self.x);
    let delta_y = other.y.max(self.y) - other.y.min(self.y);
    delta_x.hypot(delta_y)
  }
}

impl Component for Position {
  type Storage = VecStorage<Position>;
}

