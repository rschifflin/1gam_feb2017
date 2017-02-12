use specs::{Component, VecStorage};

#[derive(Debug)]
enum MoveState {
  Standing,
  Jumping
}

#[derive(Debug)]
pub struct Hero {
  move_state: MoveState
}

impl Hero {
  pub fn new() -> Hero {
    Hero {
      move_state: MoveState::Standing
    }
  }
}

impl Component for Hero {
  type Storage = VecStorage<Hero>;
}
