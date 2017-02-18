use components::{Checkpoint, Collision, Position, Velocity, Deadly};
use components::behavior::Hero as HeroBehavior;
use components::behavior::hero::MoveState;
use specs::{Entity, System, RunArg, Join};
use systems::NamedSystem;
use world::Context;
use systems::physics::GRAVITY;
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
    let (mut positions, mut velocities, mut collisions, mut heroes, deadlies, checkpoints, mut hero_events, phys_events) = arg.fetch(|w| {
      let pos = w.write::<Position>();
      let vel = w.write::<Velocity>();
      let col = w.write::<Collision>();
      let heroes = w.write::<HeroBehavior>();
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
          },

          events::Physics::Landed(e1) => {
            heroes.get_mut(e1).and_then(|mut hero| {
              velocities.get(e1).map(|vel| {
                if vel.y == 0.0 { hero.move_state = (0, MoveState::Standing) }
              })
            });
          }
        }
      }
    }

    // Jump around, Jump around
    for mut val in (&mut positions, &mut velocities, &mut heroes).iter() {
      update(&mut val, context.input.current());
      run(&mut val, context.input.current());
    }
  }
}

fn update(&mut (_, ref mut vel, ref mut hero): &mut (&mut Position, &mut Velocity, &mut HeroBehavior), (last_input, next_input): (input::Input, input::Input)) {
  match hero.move_state.1 {
    MoveState::Standing => {
      if vel.y != 0.0 { hero.move_state = (0, MoveState::Falling) }
      else if next_input.contains(input::JUMP) && !last_input.contains(input::JUMP) {
        hero.move_state = (0, MoveState::PreJump);
        vel.y = -2.0
      }
    },
    MoveState::PreJump => {
      hero.move_state.0 += 1;

      if vel.y > 0.0 { hero.move_state = (0, MoveState::Falling) };
      vel.y -= 1.5 * 0.75f64.powi(hero.move_state.0 as i32);
      if hero.move_state.0 > 15 || !next_input.contains(input::JUMP) {
        hero.move_state = (0, MoveState::Jumping)
      };
    },
    MoveState::Jumping => {
      if vel.y < 0.0 { hero.move_state = (0, MoveState::Falling) };
    },
    MoveState::Falling => {
      vel.y += GRAVITY.y;
    }
  }
}

fn run(&mut (_, ref mut vel, _): &mut (&mut Position, &mut Velocity, &mut HeroBehavior), (_, next_input): (input::Input, input::Input)) {
  if next_input.contains(input::RIGHT) {
    vel.x = 3.0;
  }

  if !next_input.contains(input::RIGHT) {
    vel.x = vel.x.min(0.0)
  }

  if next_input.contains(input::LEFT) {
    vel.x = -3.0;
  }

  if !next_input.contains(input::LEFT) {
    vel.x = vel.x.max(0.0)
  }
}

impl NamedSystem<Context> for Hero {
  fn name(&self) -> &'static str {
    "behavior_hero"
  }
}
