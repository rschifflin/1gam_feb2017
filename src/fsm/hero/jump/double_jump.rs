use input::{self, Input};
use super::Jump;
use super::constants::*;

#[derive(Debug, PartialEq)]
enum DoubleJumpState {
  Standing,
  PreJump,
  Jumping,
  Falling,
  JumpingAgain,
  FallingAgain
}

#[derive(Debug, PartialEq)]
pub struct DoubleJumpFSM {
  jump_state: (usize, DoubleJumpState),
  yvel: f64
}

impl DoubleJumpFSM {
  pub fn new() -> DoubleJumpFSM {
    DoubleJumpFSM {
      jump_state: (0, DoubleJumpState::Standing),
      yvel: 0.0
    }
  }

  fn _update(&self, yvel: f64, (last_input, next_input): (Input, Input)) -> DoubleJumpFSM {
    match self.jump_state.1 {
      DoubleJumpState::Standing => {
        if yvel != 0.0 {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::Falling),
            yvel: yvel
          }
        } else if next_input.contains(input::JUMP) && !last_input.contains(input::JUMP) {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::PreJump),
            yvel: PREJUMP_YVEL
          }
        } else {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::Standing),
            yvel: yvel
          }
        }
      },

      DoubleJumpState::PreJump => {
        let frame_counter = self.jump_state.0 + 1;
        if yvel > 0.0 {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::Falling),
            yvel: yvel
          }
        } else {
          let new_yvel = yvel + PREJUMP_ACCEL * PREJUMP_ACCEL_DECAY.powi(frame_counter as i32);
          if frame_counter > PREJUMP_FRAMES || !next_input.contains(input::JUMP) {
            DoubleJumpFSM {
              jump_state: (0, DoubleJumpState::Jumping),
              yvel: new_yvel
            }
          } else {
            DoubleJumpFSM {
              jump_state: (frame_counter, DoubleJumpState::PreJump),
              yvel: new_yvel
            }
          }
        }
      },
      DoubleJumpState::Jumping => {
        if yvel > ALLOW_DOUBLEJUMP_YVEL && next_input.contains(input::JUMP) && !last_input.contains(input::JUMP) {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::JumpingAgain),
            yvel: DOUBLEJUMP_YVEL
          }
        } else if yvel > 0.0 {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::Falling),
            yvel: yvel
          }
        } else {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::Jumping),
            yvel: yvel
          }
        }
      },

      DoubleJumpState::Falling => {
        if next_input.contains(input::JUMP) && !last_input.contains(input::JUMP) {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::JumpingAgain),
            yvel: DOUBLEJUMP_YVEL
          }
        } else {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::Falling),
            yvel: yvel + FALLING_DECEL
          }
        }
      }

      DoubleJumpState::JumpingAgain => {
        if yvel > 0.0 {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::FallingAgain),
            yvel: yvel
          }
        } else {
          DoubleJumpFSM {
            jump_state: (0, DoubleJumpState::JumpingAgain),
            yvel: yvel
          }
        }
      }

      DoubleJumpState::FallingAgain => {
        DoubleJumpFSM {
          jump_state: (0, DoubleJumpState::FallingAgain),
          yvel: yvel + FALLING_DECEL
        }
      }
    }
  }
}

impl Jump for DoubleJumpFSM {
  fn get_yvel(&self) -> f64 { self.yvel }
  fn on_landed(&mut self) {
    *self = DoubleJumpFSM {
      jump_state: (0, DoubleJumpState::Standing),
      yvel: 0.0
    };
  }

  fn update(&mut self, yvel: f64, inputs: (Input, Input)) {
    *self = self._update(yvel, inputs);
  }
}
