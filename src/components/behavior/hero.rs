use specs::{Component, VecStorage};
use progress::Progress;
use fsm::hero::{Run, Jump, SingleJumpFSM, DoubleJumpFSM, RunFSM, DashFSM};

#[derive(Debug)]
pub struct Hero {
  pub jump_state: Box<Jump>,
  pub run_state: Box<Run>,
  pub progress: Progress
}

impl Hero {
  pub fn new(progress: Progress) -> Hero {
    Hero {
      jump_state: Box::new(DoubleJumpFSM::new()),
      run_state: Box::new(DashFSM::new()),
      progress: progress
    }
  }
}

impl Component for Hero {
  type Storage = VecStorage<Hero>;
}
