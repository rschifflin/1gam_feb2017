use input::{self, Input};
use super::Jump;
use super::constants::*;

#[derive(Debug, PartialEq)]
enum SingleJumpState {
  Standing,
  PreJump,
  Jumping,
  Falling
}

#[derive(Debug, PartialEq)]
pub struct SingleJumpFSM {
  jump_state: (usize, SingleJumpState),
  yvel: f64
}

impl SingleJumpFSM {
  pub fn new() -> SingleJumpFSM {
    SingleJumpFSM {
      jump_state: (0, SingleJumpState::Standing),
      yvel: 0.0
    }
  }

  fn _update(&self, yvel: f64, (last_input, next_input): (Input, Input)) -> SingleJumpFSM {
    match self.jump_state.1 {
      SingleJumpState::Standing => {
        if yvel != 0.0 {
          SingleJumpFSM {
            jump_state: (0, SingleJumpState::Falling),
            yvel: yvel
          }
        } else if next_input.contains(input::JUMP) && !last_input.contains(input::JUMP) {
          SingleJumpFSM {
            jump_state: (0, SingleJumpState::PreJump),
            yvel: PREJUMP_YVEL
          }
        } else {
          SingleJumpFSM {
            jump_state: (0, SingleJumpState::Standing),
            yvel: yvel
          }
        }
      },

      SingleJumpState::PreJump => {
        let frame_counter = self.jump_state.0 + 1;
        if yvel > 0.0 {
          SingleJumpFSM {
            jump_state: (0, SingleJumpState::Falling),
            yvel: yvel
          }
        } else {
          let new_yvel = yvel + PREJUMP_ACCEL * PREJUMP_ACCEL_DECAY.powi(frame_counter as i32);
          if frame_counter > PREJUMP_FRAMES || !next_input.contains(input::JUMP) {
            SingleJumpFSM {
              jump_state: (0, SingleJumpState::Jumping),
              yvel: new_yvel
            }
          } else {
            SingleJumpFSM {
              jump_state: (frame_counter, SingleJumpState::PreJump),
              yvel: new_yvel
            }
          }
        }
      },
      SingleJumpState::Jumping => {
        if yvel > 0.0 {
          SingleJumpFSM {
            jump_state: (0, SingleJumpState::Falling),
            yvel: yvel
          }
        } else {
          SingleJumpFSM {
            jump_state: (0, SingleJumpState::Jumping),
            yvel: yvel
          }
        }
      },
      SingleJumpState::Falling => {
        SingleJumpFSM {
          jump_state: (0, SingleJumpState::Falling),
          yvel: yvel + FALLING_DECEL
        }
      }
    }
  }
}

impl Jump for SingleJumpFSM {
  fn get_yvel(&self) -> f64 { self.yvel }
  fn on_landed(&mut self) {
    *self = SingleJumpFSM {
      jump_state: (0, SingleJumpState::Standing),
      yvel: 0.0
    };
  }

  fn update(&mut self, yvel: f64, inputs: (Input, Input)) {
    *self = self._update(yvel, inputs);
  }
}
