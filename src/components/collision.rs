use collider::inter::{Interactivity, Group};
use collider::geom::Shape;
use specs::{Component, VecStorage};

static ALL_INTERACTS_WITH: &'static [Group] = &[CGroup::All as Group, CGroup::Friendly as Group, CGroup::Enemy as Group, CGroup::Static as Group];
static FRIENDLY_INTERACTS_WITH: &'static [Group] = &[CGroup::All as Group, CGroup::Enemy as Group, CGroup::Static as Group];
static ENEMY_INTERACTS_WITH: &'static [Group] = &[CGroup::All as Group, CGroup::Friendly as Group, CGroup::Static as Group];
static STATIC_INTERACTS_WITH: &'static [Group] = &[CGroup::All as Group, CGroup::Friendly as Group, CGroup::Enemy as Group];

#[derive(Debug)]
pub struct Collision {
  pub bounds: Shape,
  pub priority: Priority,
  pub group: CGroup,
}

#[derive(Debug, Copy, Clone)]
pub enum Priority {
  Low = 0x00000000,
  High = 0x10000000
}

#[derive(Debug, Copy, Clone)]
pub enum CGroup {
  All = 0,
  Friendly,
  Enemy,
  Static
}

impl Interactivity for CGroup {
  fn can_interact(&self, other: &Self) -> bool {
    match (*self, *other) {
      (CGroup::Friendly, CGroup::Friendly) |
      (CGroup::Enemy, CGroup::Enemy) |
      (CGroup::Static, CGroup::Static) =>  false,
      _ => true
    }
  }

  fn group(&self) -> Option<Group> { Some(*self as Group) }
  fn interact_groups(&self) -> &'static [Group] {
    match *self {
      CGroup::All => ALL_INTERACTS_WITH,
      CGroup::Friendly => FRIENDLY_INTERACTS_WITH,
      CGroup::Enemy => ENEMY_INTERACTS_WITH,
      CGroup::Static => STATIC_INTERACTS_WITH
    }
  }
}

impl Component for Collision {
  type Storage = VecStorage<Collision>;
}
