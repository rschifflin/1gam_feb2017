use specs::{Component, NullStorage};

#[derive(Default, Debug)]
pub struct Deadly;

impl Component for Deadly {
  type Storage = NullStorage<Deadly>;
}
