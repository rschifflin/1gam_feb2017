use specs::{Component, VecStorage};
use progress::Progress;
use fsm::hero::{Jump, SingleJumpFSM, DoubleJumpFSM};

#[derive(Debug)]
pub struct Hero {
  pub jump_state: Box<Jump>,
  pub progress: Progress
}

impl Hero {
  pub fn new(progress: Progress) -> Hero {
    Hero {
      jump_state: Box::new(DoubleJumpFSM::new()),
      progress: progress
    }
  }
}

impl Component for Hero {
  type Storage = VecStorage<Hero>;
}
