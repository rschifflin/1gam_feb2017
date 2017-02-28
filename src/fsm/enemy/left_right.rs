use super::EnemyAi;

#[derive(Debug)]
pub struct LeftRightFSM {
  is_left: bool,
  speed: f64,
  yvel: f64
}

impl LeftRightFSM {
  pub fn new(is_left: bool, speed: f64) -> LeftRightFSM {
    LeftRightFSM {
      is_left: is_left,
      speed: speed,
      yvel: 0.0
    }
  }

  fn _update(&self, xvel: f64, yvel: f64) -> LeftRightFSM {
    if self.is_left {
      LeftRightFSM {
        is_left: true,
        speed: self.speed,
        yvel: yvel
      }
    } else {
      LeftRightFSM {
        is_left: false,
        speed: self.speed,
        yvel: yvel
      }
    }
  }
}

impl EnemyAi for LeftRightFSM {
  fn on_collide(&mut self) {
    self.is_left = !self.is_left;
  }

  fn get_vel(&self) -> (f64, f64) {
    if self.is_left {
      (-1.0 * self.speed, self.yvel)
    } else {
      (self.speed, self.yvel)
    }
  }

  fn update(&mut self, xvel: f64, yvel: f64) {
    *self = self._update(xvel, yvel)
  }
}
