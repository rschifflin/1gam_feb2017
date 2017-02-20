use specs::{Component, VecStorage};
use progress::Progress;
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
    Hero {
      jump_state: Box::new(DoubleJumpFSM::new()),
      run_state: Box::new(DashFSM::new()),
      hang_state: Box::new(HangFSM::new()),
      progress: progress
    }
  }
}

impl Component for Hero {
  type Storage = VecStorage<Hero>;
}
