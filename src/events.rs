use std::fmt::Debug;
use specs::Entity;

#[derive(Debug, Clone)]
pub enum Game {
  Init,
  Level1,
  Level2,
}

#[derive(Debug, Clone)]
pub enum Physics {
  Collide(Entity, Entity)
}

pub trait EventType: Debug + Clone {}
impl EventType for Game {}
impl EventType for Physics {}
