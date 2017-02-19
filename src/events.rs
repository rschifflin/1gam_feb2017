use std::fmt::Debug;
use specs::Entity;

#[derive(Debug, Clone)]
pub enum Game {
  Init,
  Level1,
  Level2,
  Level3,
}

#[derive(Debug, Clone)]
pub enum Hero {
  Dead(Entity),
  Checkpoint((f64, f64)),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Physics {
  Collide(Entity, Entity),
  Landed(Entity)
}

#[derive(Debug, Clone)]
pub enum Camera {
  Switch(Entity, Entity)
}

pub trait EventType: Debug + Clone {}
impl EventType for Game {}
impl EventType for Physics {}
impl EventType for Hero {}
impl EventType for Camera {}
