use specs::{Entity, Component, VecStorage};

#[derive(Debug)]
pub struct Song {
  pub anchor: Entity,
  pub notes: Vec<Entity>,
  pub frame: usize
}

impl Song {
  pub fn new(e: Entity) -> Song {
    Song {
      anchor: e,
      notes: vec![],
      frame: 0
    }
  }
}

impl Component for Song {
  type Storage = VecStorage<Song>;
}
