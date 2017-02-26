use specs::{Component, NullStorage};

#[derive(Default, Debug)]
pub struct StaticGeom;

impl Component for StaticGeom {
  type Storage = NullStorage<StaticGeom>;
}
