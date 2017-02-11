use components::{Physical, Position, Collision, Velocity};
use specs::{Allocator, System, RunArg, Join, Storage, MaskedStorage};
use systems::NamedSystem;
use world::Context;
use std::f64::consts::PI;
use input;

pub struct Physics;
const MAX_SPEED: f64 = 10.0;
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
      jump(&mut val, context.input);
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

fn jump(&mut (_, ref mut vel, _, _): &mut (&mut Position, &mut Velocity, &Collision, &Physical), (last_input, next_input): (input::Input, input::Input)) {
  if next_input.contains(input::JUMP) && !last_input.contains(input::JUMP) {
    **vel = Velocity::add(vel, &Velocity { speed: 1.0, angle: -0.5 * PI });
  }
}

fn update(&mut (_, ref mut vel, _, _): &mut (&mut Position, &mut Velocity, &Collision, &Physical)) {
  **vel = Velocity::add(vel, &GRAVITY);
  vel.speed = vel.speed.min(MAX_SPEED);
}

fn apply(&mut (ref mut pos, ref mut vel, _, _): &mut (&mut Position, &mut Velocity, &Collision, &Physical)) {
  let (x, y) = vel.to_cart();
  pos.x += x;
  pos.y += y;
}
