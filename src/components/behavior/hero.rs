use specs::{Component, VecStorage};
use progress::{self, Progress};
use fsm::hero::{Run, Jump, Hang, SingleJumpFSM, DoubleJumpFSM, RunFSM, DashFSM, HangFSM, HanglessFSM};

#[derive(Debug)]
pub struct Hero {
  pub jump_state: Box<Jump>,
  pub run_state: Box<Run>,
  pub hang_state: Box<Hang>,
  pub progress: Progress
}

impl Hero {
  pub fn new(progress: Progress) -> Hero {
    let jump_state: Box<Jump> = if progress.contains(progress::DOUBLE_JUMP) {
      Box::new(DoubleJumpFSM::new())
    } else {
      Box::new(SingleJumpFSM::new())
    };

    let run_state: Box<Run> = if progress.contains(progress::DASH) {
      Box::new(DashFSM::new())
    } else {
      Box::new(RunFSM::new())
    };

    let hang_state: Box<Hang> = if progress.contains(progress::HANG) {
      Box::new(HangFSM::new())
    } else {
      Box::new(HanglessFSM::new())
    };

    Hero {
      jump_state: jump_state,
      run_state: run_state,
      hang_state: hang_state,
      progress: progress
    }
  }
}

impl Component for Hero {
  type Storage = VecStorage<Hero>;
}
