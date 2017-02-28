use super::EnemyAi;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct ArcFSM {
  is_clockwise: bool,
  radius: f64,
  delta_angle: f64,
  angle: f64
}

impl ArcFSM {
  pub fn new(is_clockwise: bool, radius: f64, delta_angle: f64, angle: f64) -> ArcFSM {
    ArcFSM {
      is_clockwise: is_clockwise,
      radius: radius,
      delta_angle: delta_angle,
      angle: angle,
    }
  }

  fn _update(&self) -> ArcFSM {
    if self.is_clockwise {
      ArcFSM {
        is_clockwise: true,
        radius: self.radius,
        delta_angle: self.delta_angle,
        angle: self.angle - self.delta_angle
      }
    } else {
      ArcFSM {
        is_clockwise: false,
        radius: self.radius,
        delta_angle: self.delta_angle,
        angle: self.angle + self.delta_angle
      }
    }
  }
}

impl EnemyAi for ArcFSM {
  fn on_collide(&mut self) {
    self.is_clockwise = !self.is_clockwise;
  }

  fn get_vel(&self) -> (f64, f64) {
    let next_angle = if self.is_clockwise {
      (self.angle - self.delta_angle) % (2.0 * PI)
    } else {
      (self.angle + self.delta_angle) % (2.0 * PI)
    };

    let (xvel, yvel) = (self.radius * self.angle.sin(),
                        self.radius * self.angle.cos());
    let (next_xvel, next_yvel) = (self.radius * next_angle.sin(),
                                  self.radius * next_angle.cos());
    (next_xvel - xvel, next_yvel - yvel)
  }

  fn update(&mut self, _: f64, _: f64) {
    *self = self._update()
  }
}
