use super::EnemyAi;

#[derive(Debug)]
pub struct WanderFSM {
  xvel: f64,
  yvel: f64
}

impl WanderFSM {
  pub fn new() -> WanderFSM {
    WanderFSM {
      xvel: 0.0,
      yvel: 0.0
    }
  }

  fn _update(&self, xvel: f64, yvel: f64) -> WanderFSM {
    WanderFSM {
      xvel: 0.0,
      yvel: 0.0
    }
  }
}

impl EnemyAi for WanderFSM {
  fn on_collide(&mut self) {
  }

  fn get_vel(&self) -> (f64, f64) {
    (self.xvel, self.yvel)
  }

  fn update(&mut self, xvel: f64, yvel: f64) {
    *self = self._update(xvel, yvel)
  }
}
