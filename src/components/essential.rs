use specs::{Component, NullStorage};

#[derive(Default, Debug)]
pub struct Essential;

impl Component for Essential {
  type Storage = NullStorage<Essential>;
}
