use specs::{Component, VecStorage};

#[derive(Debug, PartialEq)]
pub enum MoveState {
  Standing,
  PreJump,
  Jumping,
  Falling
}

#[derive(Debug)]
pub struct Hero {
  pub move_state: (u32, MoveState)
}

impl Hero {
  pub fn new() -> Hero {
    Hero {
      move_state: (0, MoveState::Falling)
    }
  }
}

impl Component for Hero {
  type Storage = VecStorage<Hero>;
}
