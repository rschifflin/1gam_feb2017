use specs::{Component, VecStorage};
use fsm::enemy::EnemyAi;

#[derive(Debug)]
pub struct Enemy {
  pub ai: Box<EnemyAi>
}

impl Enemy {
  pub fn new(ai: Box<EnemyAi>) -> Enemy {
    Enemy {
      ai: ai
    }
  }
}

impl Component for Enemy {
  type Storage = VecStorage<Enemy>;
}
