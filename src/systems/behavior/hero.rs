use components::{Checkpoint, Collision, Position, Velocity, Deadly, Sprite, Song, Graphic, Layer};
use components::behavior::Hero as HeroBehavior;
use specs::{World, Entity, Entities, System, RunArg, Join};
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
    let (entities, mut positions, mut velocities, mut collisions, mut songs, mut sprites, mut heroes, deadlies, checkpoints, mut hero_events, phys_events) = arg.fetch(|w| {
      let pos = w.write::<Position>();
      let vel = w.write::<Velocity>();
      let col = w.write::<Collision>();
      let songs = w.write::<Song>();
      let sprites = w.write::<Sprite>();
      let heroes = w.write::<HeroBehavior>();
      let deadlies = w.read::<Deadly>();
      let checkpoints = w.read::<Checkpoint>();
      let hero_events = w.write_resource::<Vec<events::Hero>>();
      let phys_events = w.read_resource::<Vec<events::Physics>>();
      (w.entities(), pos, vel, col, songs, sprites, heroes, deadlies, checkpoints, hero_events, phys_events)
    });
    hero_events.clear();
    let input = context.input.current();

    //Physics event checking
    {
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
                collisions.remove(hero.0);
                hero_events.push(events::Hero::Dead(hero.0));
              },
              (hero, cp) | (cp, hero) if (hero.1.contains(HERO_FLAG) && cp.1.contains(CHECKPOINT_FLAG)) => {
                positions.get(hero.0).map(|pos| {
                  collisions.remove(cp.0);
                  sprites.insert(cp.0, Sprite::new(Graphic::Checkpoint(true), Layer::Layer3));
                  hero_events.push(events::Hero::Checkpoint((pos.x, pos.y)));
                });
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

    for (entity, hero) in (&entities, &heroes).iter() {
      if input.1.contains(input::WHISTLE) && !input.0.contains(input::WHISTLE) {
        hero_events.push(events::Hero::Singing(entity, hero.progress));
        songs.insert(arg.create(), Song::new(entity, hero.progress));
      }
    }

    for mut val in (&positions, &mut velocities, &mut sprites, &mut heroes).iter() {
      update(&mut val, input);
    }
  }
}

fn update(&mut (ref pos, ref mut vel, ref mut sprite, ref mut hero): &mut (&Position, &mut Velocity, &mut Sprite, &mut HeroBehavior), inputs: (input::Input, input::Input)) {
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
