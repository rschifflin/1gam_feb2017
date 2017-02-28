use specs::{Component, VecStorage};
use progress::Progress;

#[derive(Debug)]

pub struct GameState {
  pub level: usize,
  pub spawn: (f64, f64),
  pub progress: Progress
}

impl Default for GameState {
  fn default() -> GameState {
    GameState {
      level: 0,
      spawn: (10.0, 10.0),
      progress: Progress::empty()
    }
  }
}

impl Component for GameState {
  type Storage = VecStorage<GameState>;
}
