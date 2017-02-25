use super::Hang;
use super::constants::*;

#[derive(Debug, PartialEq)]
enum HangState {
  Free,
  Hanging
}

#[derive(Debug, PartialEq)]
pub struct HangFSM {
  hang_state: HangState,
  yvel: f64
}

impl HangFSM {
  pub fn new() -> HangFSM {
    HangFSM {
      hang_state: HangState::Free,
      yvel: 0.0
    }
  }

  fn _update(&self, yvel: f64) -> HangFSM {
    match self.hang_state {
      HangState::Free => {
        HangFSM {
          hang_state: HangState::Free,
          yvel: yvel
        }
      },

      HangState::Hanging => {
        HangFSM {
          hang_state: HangState::Free,
          yvel: HANG_YVEL
        }
      }
    }
  }
}

impl Hang for HangFSM {
  fn get_yvel(&self) -> f64 { self.yvel }

  fn on_bonked(&mut self) {
    *self = HangFSM {
      hang_state: HangState::Hanging,
      yvel: 0.0
    };
  }

  fn update(&mut self, yvel: f64) {
    *self = self._update(yvel);
  }
}
