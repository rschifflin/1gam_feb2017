use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct GameState {
  pub level: usize,
}

impl Component for GameState {
  type Storage = VecStorage<GameState>;
}
