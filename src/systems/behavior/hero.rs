use components::{Checkpoint, Collision, Position, Velocity, Deadly, Sprite};
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
    let (mut positions, mut velocities, mut collisions, mut sprites, mut heroes, deadlies, checkpoints, mut hero_events, phys_events) = arg.fetch(|w| {
      let pos = w.write::<Position>();
      let vel = w.write::<Velocity>();
      let col = w.write::<Collision>();
      let sprites = w.write::<Sprite>();
      let heroes = w.write::<HeroBehavior>();
      let deadlies = w.read::<Deadly>();
      let checkpoints = w.read::<Checkpoint>();
      let hero_events = w.write_resource::<Vec<events::Hero>>();
      let phys_events = w.read_resource::<Vec<events::Physics>>();
      (pos, vel, col, sprites, heroes, deadlies, checkpoints, hero_events, phys_events)
    });
    hero_events.clear();
    let input = context.input.current();

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
                if vel.y == 0.0 { hero.jump_state.on_landed() }
              })
            });
          },

          events::Physics::Bonked(e1) => {
            heroes.get_mut(e1).map(|hero| {
              if input.1.contains(input::JUMP) { hero.hang_state.on_bonked() }
            });
          }
        }
      }
    }

    // Jump around, Jump around
    for mut val in (&mut positions, &mut velocities, &mut sprites, &mut heroes).iter() {
      update(&mut val, input);
    }
  }
}

fn update(&mut (_, ref mut vel, ref mut sprite, ref mut hero): &mut (&mut Position, &mut Velocity, &mut Sprite, &mut HeroBehavior), inputs: (input::Input, input::Input)) {
  hero.hang_state.update(vel.y);
  vel.y = hero.hang_state.get_yvel();

  hero.jump_state.update(vel.y, inputs);
  vel.y = hero.jump_state.get_yvel();

  hero.run_state.update(vel.x, vel.y, inputs);
  let (new_xvel, new_yvel) = hero.run_state.get_vel();
  vel.x = new_xvel;
  vel.y = new_yvel;

  if vel.x < 0.0 {
    sprite.flip(true);
  } else if vel.x > 0.0 {
    sprite.flip(false);
  }
}

impl NamedSystem<Context> for Hero {
  fn name(&self) -> &'static str {
    "behavior_hero"
  }
}
