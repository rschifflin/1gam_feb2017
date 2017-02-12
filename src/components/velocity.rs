use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Velocity {
  pub angle: f64, //rads
  pub speed: f64
}

impl Component for Velocity {
  type Storage = VecStorage<Velocity>;
}

impl Velocity {
  pub fn zero() -> Velocity {
    Velocity {
      speed: 0.0,
      angle: 0.0
    }
  }

  pub fn to_cart(&self) -> (f64, f64) {
    (self.speed * self.angle.cos(),
     self.speed * self.angle.sin())
  }

  pub fn from_cart((x, y): (f64, f64)) -> Velocity {
    let speed = x.hypot(y);
    let angle = y.atan2(x);

    Velocity {
      speed: speed,
      angle: angle
    }
  }

  pub fn add(v1: &Velocity, v2: &Velocity) -> Velocity {
    let (x1, y1) = v1.to_cart();
    let (x2, y2) = v2.to_cart();
    let (x3, y3) = (x2 + x1, y2 + y1);
    Velocity::from_cart((x3, y3))
  }
}
