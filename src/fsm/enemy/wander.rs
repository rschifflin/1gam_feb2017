use super::EnemyAi;
use rand;

use systems::physics::GRAVITY;
const IDLE_TIME: usize = 45;
const WANDER_TIME: usize = 30;
const JUMP_RATIO: f64 = 0.25;
const JUMP_YVEL: f64 = (GRAVITY.y * WANDER_TIME as f64) / -2.0;

#[derive(Debug, Copy, Clone)]
enum WanderState {
  Idle,
  Wandering,
}

#[derive(Debug)]
pub struct WanderFSM {
  state: (usize, WanderState),
  is_left: bool,
  frequency: f64,
  speed: f64,
  yvel: f64,
}

impl WanderFSM {
  pub fn new(frequency: f64, speed: f64) -> WanderFSM {
    WanderFSM {
      state: (0, WanderState::Idle),
      is_left: true,
      frequency: frequency,
      speed: speed,
      yvel: 0.0
    }
  }

  fn _update(&self, xvel: f64, yvel: f64) -> WanderFSM {
    match self.state {
      (n, WanderState::Idle) => {
        if (n as f64) < (IDLE_TIME as f64 * self.frequency) {
          WanderFSM {
            state: (n+1, WanderState::Idle),
            is_left: self.is_left,
            frequency: self.frequency,
            speed: self.speed,
            yvel: yvel
          }
        } else {
          WanderFSM {
            state: (0, WanderState::Wandering),
            is_left: rand::random(),
            frequency: self.frequency,
            speed: self.speed,
            yvel: yvel
          }
        }
      },

      (n, WanderState::Wandering) => {
        if n < WANDER_TIME {
          WanderFSM {
            state: (n+1, WanderState::Wandering),
            is_left: self.is_left,
            frequency: self.frequency,
            speed: self.speed,
            yvel: yvel
          }
        } else {
          WanderFSM {
            state: (0, WanderState::Idle),
            is_left: self.is_left,
            frequency: self.frequency,
            speed: self.speed,
            yvel: yvel
          }
        }
      }
    }
  }
}

impl EnemyAi for WanderFSM {
  fn on_collide(&mut self) {
  }

  fn get_vel(&self) -> (f64, f64) {
    let speed = if self.is_left { -self.speed } else { self.speed };
    match self.state {
      (_, WanderState::Idle) => { if self.yvel == 0.0 { (0.0, 0.0) } else { (speed, self.yvel) } },
      (0, WanderState::Wandering) if rand::random::<f64>() < JUMP_RATIO => (speed, JUMP_YVEL),
      (_, WanderState::Wandering) => (speed, self.yvel)
    }
  }

  fn update(&mut self, xvel: f64, yvel: f64) {
    *self = self._update(xvel, yvel)
  }
}
