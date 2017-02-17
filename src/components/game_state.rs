use specs::{Component, VecStorage};

#[derive(Default, Debug)]
pub struct GameState {
  pub level: usize,
  pub spawn: (f64, f64),
}

impl Component for GameState {
  type Storage = VecStorage<GameState>;
}
