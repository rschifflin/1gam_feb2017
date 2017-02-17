use specs::{Component, NullStorage};

#[derive(Default, Debug)]
pub struct Checkpoint;

impl Component for Checkpoint {
  type Storage = NullStorage<Checkpoint>;
}
