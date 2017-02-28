use components::{Position, Velocity};
use components::behavior::Enemy as EnemyBehavior;
use specs::{Entity, System, RunArg, Join};
use systems::NamedSystem;
use world::Context;
use events;

pub struct Enemy;

impl System<Context> for Enemy {
  fn run(&mut self, arg: RunArg, context: Context) {
    let (entities, mut enemies, mut positions, mut velocities, phys_events) = arg.fetch(|w| {
      let enemies = w.write::<EnemyBehavior>();
      let pos = w.write::<Position>();
      let vel = w.write::<Velocity>();
      let phys_events = w.write_resource::<Vec<events::Physics>>();
      (w.entities(), enemies, pos, vel, phys_events)
    });

    for phys_event in phys_events.iter() {
      match *phys_event {
        events::Physics::Collide(e1, e2) => {
          enemies.get_mut(e1).map(|mut enemy| enemy.ai.on_collide());
          enemies.get_mut(e2).map(|mut enemy| enemy.ai.on_collide());
        },
        _ => ()
      }
    }

    for (entity, enemy, vel) in (&entities, &mut enemies, &mut velocities).iter() {
      enemy.ai.update(vel.x, vel.y);
      let (vel_x, vel_y) = enemy.ai.get_vel();
      vel.x = vel_x;
      vel.y = vel_y;
    }
  }
}

impl NamedSystem<Context> for Enemy {
  fn name(&self) -> &'static str {
    "behavior_enemy"
  }
}
