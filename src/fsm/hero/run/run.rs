use input::{self, Input};
use super::Run;
use super::constants::*;

#[derive(Debug, PartialEq)]
enum RunState {
  Standing,
  Running(Dir)
}

#[derive(Debug, PartialEq)]
enum Dir {
  Right,
  Left
}

#[derive(Debug, PartialEq)]
pub struct RunFSM {
  run_state: RunState,
  xvel: f64,
  yvel: f64
}

impl RunFSM {
  pub fn new() -> RunFSM {
    RunFSM {
      run_state: RunState::Standing,
      xvel: 0.0,
      yvel: 0.0
    }
  }

  fn _update(&self, xvel: f64, yvel: f64, (last_input, next_input): (Input, Input)) -> RunFSM {
    match self.run_state {
      RunState::Standing => {
        if next_input.contains(input::RIGHT) {
          RunFSM {
            run_state: RunState::Running(Dir::Right),
            xvel: RUNSPEED,
            yvel: yvel
          }
        } else if next_input.contains(input::LEFT) {
          RunFSM {
            run_state: RunState::Running(Dir::Left),
            xvel: -RUNSPEED,
            yvel: yvel
          }
        } else {
          RunFSM {
            run_state: RunState::Standing,
            xvel: xvel,
            yvel: yvel
          }
        }
      },
      RunState::Running(Dir::Right) => {
        if next_input.contains(input::LEFT) && !last_input.contains(input::LEFT) {
          RunFSM {
            run_state: RunState::Running(Dir::Left),
            xvel: -RUNSPEED,
            yvel: yvel
          }
        } else if !next_input.contains(input::RIGHT) {
          RunFSM {
            run_state: RunState::Standing,
            xvel: xvel.min(0.0),
            yvel: yvel
          }
        } else {
          RunFSM {
            run_state: RunState::Running(Dir::Right),
            xvel: RUNSPEED,
            yvel: yvel
          }
        }
      },
      RunState::Running(Dir::Left) => {
        if next_input.contains(input::RIGHT) && !last_input.contains(input::RIGHT) {
          RunFSM {
            run_state: RunState::Running(Dir::Right),
            xvel: RUNSPEED,
            yvel: yvel
          }
        } else if !next_input.contains(input::LEFT) {
          RunFSM {
            run_state: RunState::Standing,
            xvel: xvel.max(0.0),
            yvel: yvel
          }
        } else {
          RunFSM {
            run_state: RunState::Running(Dir::Left),
            xvel: -RUNSPEED,
            yvel: yvel
          }
        }
      }
    }
  }
}

impl Run for RunFSM {
  fn get_vel(&self) -> (f64, f64) {
    (self.xvel, self.yvel)
  }

  fn is_dashing(&self) -> bool {
    false
  }

  fn update(&mut self, xvel: f64, yvel: f64, inputs: (Input, Input)) {
    *self = self._update(xvel, yvel, inputs);
  }
}
