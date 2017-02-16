use specs::{Entity, Component, NullStorage};

#[derive(Default, Debug)]
pub struct BlastZone;

impl Component for BlastZone {
  type Storage = NullStorage<BlastZone>;
}
