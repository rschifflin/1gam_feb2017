use specs::{Component, NullStorage};

#[derive(Debug, Default)]
pub struct Sprite { }

impl Component for Sprite {
  type Storage = NullStorage<Sprite>;
}
