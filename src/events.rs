use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use itertools::Itertools;
use std::iter::{Iterator, FromIterator};
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

trait EventType {}
impl EventType for Game {}
impl EventType for Physics {}

pub type Events<T: EventType + Debug + Clone> = Vec<T>;
