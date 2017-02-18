use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Velocity {
  pub x: f64,
  pub y: f64
}

impl Component for Velocity {
  type Storage = VecStorage<Velocity>;
}

impl Velocity {
  pub fn zero() -> Velocity {
    Velocity {
      x: 0.0,
      y: 0.0
    }
  }

  pub fn to_polar(&self) -> (f64, f64) {
    let angle = self.y.atan2(self.x);
    let len = self.x.hypot(self.y);
    (angle, len)
  }

  pub fn from_polar((angle, len): (f64, f64)) -> Velocity {
    Velocity {
      x: len * angle.cos(),
      y: len * angle.sin()
    }
  }

  pub fn add(v1: &Velocity, v2: &Velocity) -> Velocity {
    Velocity {
      x: v1.x + v2.x,
      y: v1.y + v2.y
    }
  }
}
