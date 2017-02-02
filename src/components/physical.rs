use specs::{Component, NullStorage};

#[derive(Debug, Default)]
pub struct Physical { }

impl Component for Physical {
  type Storage = NullStorage<Physical>;
}
