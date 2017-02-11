use components::{Physical, Position, Collision, Velocity};
use specs::{Allocator, System, RunArg, Join, Storage, MaskedStorage};
use systems::NamedSystem;
use world::Context;
use std::f64::consts::PI;

pub struct Physics;
const GRAVITY: Velocity = Velocity { speed: 0.1, angle: 0.5*PI };

impl System<Context> for Physics {
  fn run(&mut self, arg: RunArg, context: Context) {
    let (mut positions, mut velocities, collisions, physicals) = arg.fetch(|w| {
      let pos = w.write::<Position>();
      let vel = w.write::<Velocity>();
      let col = w.read::<Collision>();
      let phys = w.read::<Physical>();
      (pos, vel, col, phys)
    });

    for mut val in (&mut positions, &mut velocities, &collisions, &physicals).iter() {
      update(&mut val);
      apply(&mut val);
    }
  }
}

impl NamedSystem<Context> for Physics {
  fn name(&self) -> &'static str {
    "physics"
  }
}

fn update(&mut (_, ref mut vel, _, _): &mut (&mut Position, &mut Velocity, &Collision, &Physical)) {
  **vel = Velocity::add(vel, &GRAVITY);
}

fn apply(&mut (ref mut pos, ref mut vel, _, _): &mut (&mut Position, &mut Velocity, &Collision, &Physical)) {
  let (x, y) = vel.to_cart();
  pos.x += x;
  pos.y += y;
}
