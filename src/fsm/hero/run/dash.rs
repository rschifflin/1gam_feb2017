use input::{self, Input};
use super::Run;
use super::constants::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum DashState {
  Standing,
  Running(Dir),
  Dashing(Dir)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum InputState {
  NoInput,
  One(Dir),
  Two(Dir),
  Cooldown,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Dir {
  Right,
  Left
}

#[derive(Debug, PartialEq)]
pub struct DashFSM {
  dash_state: (usize, DashState),
  input_state: (usize, InputState),
  xvel: f64,
  yvel: f64
}

impl DashFSM {
  pub fn new() -> DashFSM {
    DashFSM {
      dash_state: (0, DashState::Standing),
      input_state: (0, InputState::NoInput),
      xvel: 0.0,
      yvel: 0.0
    }
  }

  fn _update(&self, xvel: f64, yvel: f64, (last_input, next_input): (Input, Input)) -> DashFSM {
    let input_state: (usize, InputState) = match self.input_state.1 {
      InputState::NoInput => {
        if next_input.contains(input::LEFT) && !last_input.contains(input::LEFT) {
          (0, InputState::One(Dir::Left))
        } else if next_input.contains(input::RIGHT) && !last_input.contains(input::RIGHT) {
          (0, InputState::One(Dir::Right))
        } else {
          (0, InputState::NoInput)
        }
      },

      InputState::One(Dir::Left) => {
        if next_input.contains(input::LEFT) && !last_input.contains(input::LEFT) {
          (0, InputState::Two(Dir::Left))
        } else if next_input.contains(input::RIGHT) && !last_input.contains(input::RIGHT) {
          (0, InputState::One(Dir::Right))
        } else if self.input_state.0 >= INPUT_FRAMES {
          (0, InputState::NoInput)
        } else {
          (self.input_state.0 + 1, InputState::One(Dir::Left))
        }
      },

      InputState::One(Dir::Right) => {
        if next_input.contains(input::LEFT) && !last_input.contains(input::LEFT) {
          (0, InputState::One(Dir::Left))
        } else if next_input.contains(input::RIGHT) && !last_input.contains(input::RIGHT) {
          (0, InputState::Two(Dir::Right))
        } else if self.input_state.0 >= INPUT_FRAMES {
          (0, InputState::NoInput)
        } else {
          (self.input_state.0 + 1, InputState::One(Dir::Right))
        }
      },

      InputState::Two(_) => (0, InputState::Cooldown),

      InputState::Cooldown => {
        if self.input_state.0 >= COOLDOWN_FRAMES {
          (0, InputState::NoInput)
        } else {
          (self.input_state.0 + 1, InputState::Cooldown)
        }
      }
    };

    match (input_state.1, self.dash_state.1) {
      (InputState::Two(Dir::Left), _) => {
          DashFSM {
            dash_state: (0, DashState::Dashing(Dir::Left)),
            input_state: input_state,
            xvel: -DASHSPEED,
            yvel: yvel
          }
      },

      (InputState::Two(Dir::Right), _) => {
          DashFSM {
            dash_state: (0, DashState::Dashing(Dir::Right)),
            input_state: input_state,
            xvel: DASHSPEED,
            yvel: yvel
          }
      },

      (_, DashState::Standing) => {
        if next_input.contains(input::RIGHT) {
          DashFSM {
            dash_state: (0, DashState::Running(Dir::Right)),
            input_state: input_state,
            xvel: RUNSPEED,
            yvel: yvel
          }
        } else if next_input.contains(input::LEFT) {
          DashFSM {
            dash_state: (0, DashState::Running(Dir::Left)),
            input_state: input_state,
            xvel: -RUNSPEED,
            yvel: yvel
          }
        } else {
          DashFSM {
            dash_state: (0, DashState::Standing),
            input_state: input_state,
            xvel: xvel,
            yvel: yvel
          }
        }
      },
      (_, DashState::Running(Dir::Right)) => {
        if next_input.contains(input::LEFT) && !last_input.contains(input::LEFT) {
          DashFSM {
            dash_state: (0, DashState::Running(Dir::Left)),
            input_state: input_state,
            xvel: -RUNSPEED,
            yvel: yvel
          }
        } else if !next_input.contains(input::RIGHT) {
          DashFSM {
            dash_state: (0, DashState::Standing),
            input_state: input_state,
            xvel: xvel.min(0.0),
            yvel: yvel
          }
        } else {
          DashFSM {
            dash_state: (0, DashState::Running(Dir::Right)),
            input_state: input_state,
            xvel: RUNSPEED,
            yvel: yvel
          }
        }
      },
      (_, DashState::Running(Dir::Left)) => {
        if next_input.contains(input::RIGHT) && !last_input.contains(input::RIGHT) {
          DashFSM {
            dash_state: (0, DashState::Running(Dir::Right)),
            input_state: input_state,
            xvel: RUNSPEED,
            yvel: yvel
          }
        } else if !next_input.contains(input::LEFT) {
          DashFSM {
            dash_state: (0, DashState::Standing),
            input_state: input_state,
            xvel: xvel.max(0.0),
            yvel: yvel
          }
        } else {
          DashFSM {
            dash_state: (0, DashState::Running(Dir::Left)),
            input_state: input_state,
            xvel: -RUNSPEED,
            yvel: yvel
          }
        }
      },
      (_, DashState::Dashing(Dir::Left)) => {
        if self.dash_state.0 >= DASH_FRAMES {
          DashFSM {
            dash_state: (0, DashState::Running(Dir::Left)),
            input_state: input_state,
            xvel: -RUNSPEED,
            yvel: yvel
          }
        } else {
          DashFSM {
            dash_state: (self.dash_state.0 + 1, DashState::Dashing(Dir::Left)),
            input_state: input_state,
            xvel: -DASHSPEED,
            yvel: DASH_YVEL
          }
        }
      },

      (_, DashState::Dashing(Dir::Right)) => {
        if self.dash_state.0 >= DASH_FRAMES {
          DashFSM {
            dash_state: (0, DashState::Running(Dir::Right)),
            input_state: input_state,
            xvel: RUNSPEED,
            yvel: yvel

          }
        } else {
          DashFSM {
            dash_state: (self.dash_state.0 + 1, DashState::Dashing(Dir::Right)),
            input_state: input_state,
            xvel: DASHSPEED,
            yvel: DASH_YVEL
          }
        }
      }
    }
  }
}

impl Run for DashFSM {
  fn get_vel(&self) -> (f64, f64) {
    (self.xvel, self.yvel)
  }

  fn is_dashing(&self) -> bool {
    match self.dash_state.1 {
      DashState::Dashing(_) => true,
      _ => false
    }
  }

  fn update(&mut self, xvel: f64, yvel: f64, inputs: (Input, Input)) {
    *self = self._update(xvel, yvel, inputs);
  }
}
