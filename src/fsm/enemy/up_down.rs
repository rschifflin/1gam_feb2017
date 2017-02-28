use super::EnemyAi;

#[derive(Debug)]
pub struct UpDownFSM {
  is_up: bool,
  xvel: f64,
  speed: f64,
}

impl UpDownFSM {
  pub fn new(is_up: bool, speed: f64) -> UpDownFSM {
    UpDownFSM {
      is_up: is_up,
      xvel: 0.0,
      speed: speed
    }
  }
fn _update(&self, xvel: f64, yvel: f64) -> UpDownFSM {
    if self.is_up {
      UpDownFSM {
        is_up: true,
        xvel: xvel,
        speed: self.speed
      }
    } else {
      UpDownFSM {
        is_up: false,
        xvel: xvel,
        speed: self.speed
      }
    }
  }
}

impl EnemyAi for UpDownFSM {
  fn on_collide(&mut self) {
    self.is_up = !self.is_up;
  }

  fn get_vel(&self) -> (f64, f64) {
    if self.is_up {
      (self.xvel, -1.0 * self.speed)
    } else {
      (self.xvel, self.speed)
    }
  }

  fn update(&mut self, xvel: f64, yvel: f64) {
    *self = self._update(xvel, yvel)
  }
}
