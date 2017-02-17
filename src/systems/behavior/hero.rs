use components::{Checkpoint, Collision, Position, Velocity, Deadly};
use components::behavior::Hero as HeroBehavior;
use specs::{Entity, System, RunArg, Join};
use systems::NamedSystem;
use world::Context;
use input;
use events;

bitflags! {
  flags ComponentFlags: u32 {
    const HERO_FLAG = 0b00000001,
    const DEADLY_FLAG = 0b00000010,
    const CHECKPOINT_FLAG = 0b00000100
  }
}

pub struct Hero;
impl System<Context> for Hero {
  fn run(&mut self, arg: RunArg, context: Context) {
    let (mut positions, mut velocities, mut collisions, heroes, deadlies, checkpoints, mut hero_events, phys_events) = arg.fetch(|w| {
      let pos = w.write::<Position>();
      let vel = w.write::<Velocity>();
      let col = w.write::<Collision>();
      let heroes = w.read::<HeroBehavior>();
      let deadlies = w.read::<Deadly>();
      let checkpoints = w.read::<Checkpoint>();
      let hero_events = w.write_resource::<Vec<events::Hero>>();
      let phys_events = w.read_resource::<Vec<events::Physics>>();
      (pos, vel, col, heroes, deadlies, checkpoints, hero_events, phys_events)
    });
    hero_events.clear();

    //Physics event checking
    {
      let mut collide_hero_deadly = |hero: Entity, _: Entity, events: &mut Vec<events::Hero>| {
          collisions.remove(hero);
          events.push(events::Hero::Dead(hero));
      };

      let collide_hero_checkpoint = |hero: Entity, checkpoint: Entity, events: &mut Vec<events::Hero>| {
          positions.get(hero).map(|pos| {
            arg.delete(checkpoint);
            events.push(events::Hero::Checkpoint((pos.x, pos.y)));
          });
      };

      for phys_event in phys_events.iter() {
        match *phys_event {
          events::Physics::Collide(e1, e2) => {
            let absent = ComponentFlags::empty();
            let lhs = (e1,
                       heroes.get(e1).map_or(absent, |_| HERO_FLAG) |
                       deadlies.get(e1).map_or(absent, |_| DEADLY_FLAG) |
                       checkpoints.get(e1).map_or(absent, |_| CHECKPOINT_FLAG));
            let rhs = (e2,
                       heroes.get(e2).map_or(absent, |_| HERO_FLAG) |
                       deadlies.get(e2).map_or(absent, |_| DEADLY_FLAG) |
                       checkpoints.get(e2).map_or(absent, |_| CHECKPOINT_FLAG));

            match (lhs, rhs) {
              (hero, deadly) | (deadly, hero) if (hero.1.contains(HERO_FLAG) && deadly.1.contains(DEADLY_FLAG)) => {
                collide_hero_deadly(hero.0, deadly.0, &mut hero_events);
              },
              (hero, cp) | (cp, hero) if (hero.1.contains(HERO_FLAG) && cp.1.contains(CHECKPOINT_FLAG)) => {
                collide_hero_checkpoint(hero.0, cp.0, &mut hero_events);
              },
              _ => ()
            }
          }
        }
      }
    }

    // Jump around, Jump around
    for mut val in (&mut positions, &mut velocities, &heroes).iter() {
      jump(&mut val, context.input.current());
      run(&mut val, context.input.current());
    }
  }
}

fn jump(&mut (_, ref mut vel, _): &mut (&mut Position, &mut Velocity, &HeroBehavior), (last_input, next_input): (input::Input, input::Input)) {
  if next_input.contains(input::UP) && !last_input.contains(input::UP) {
    let (x, _) = vel.to_cart();
    **vel = Velocity::from_cart((x, -5.0));
  }
}

fn run(&mut (_, ref mut vel, _): &mut (&mut Position, &mut Velocity, &HeroBehavior), (_, next_input): (input::Input, input::Input)) {
  if next_input.contains(input::RIGHT) {
    let (_, y) = vel.to_cart();
    **vel = Velocity::from_cart((3.0, y));
  }

  if !next_input.contains(input::RIGHT) {
    let (x, y) = vel.to_cart();
    **vel = Velocity::from_cart((x.min(0.0), y));
  }

  if next_input.contains(input::LEFT) {
    let (_, y) = vel.to_cart();
    **vel = Velocity::from_cart((-3.0, y));
  }

  if !next_input.contains(input::LEFT) {
    let (x, y) = vel.to_cart();
    **vel = Velocity::from_cart((x.max(0.0), y));
  }
}

impl NamedSystem<Context> for Hero {
  fn name(&self) -> &'static str {
    "behavior_hero"
  }
}
