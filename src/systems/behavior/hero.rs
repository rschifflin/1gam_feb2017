use components::{Physical, Position, Collision, Velocity};
use components::behavior::Hero as HeroBehavior;
use specs::{Allocator, System, RunArg, Join, Storage, MaskedStorage};
use systems::NamedSystem;
use world::Context;
use std::f64::consts::PI;
use input;

pub struct Hero;
impl System<Context> for Hero {
  fn run(&mut self, arg: RunArg, context: Context) {
    let (mut positions, mut velocities, mut heroes) = arg.fetch(|w| {
      let pos = w.write::<Position>();
      let vel = w.write::<Velocity>();
      let heroes = w.read::<HeroBehavior>();
      (pos, vel, heroes)
    });

    for mut val in (&mut positions, &mut velocities, &heroes).iter() {
      jump(&mut val, context.input.current());
      run(&mut val, context.input.current());
    }
  }
}

fn jump(&mut (_, ref mut vel, _): &mut (&mut Position, &mut Velocity, &HeroBehavior), (last_input, next_input): (input::Input, input::Input)) {
  if next_input.contains(input::JUMP) && !last_input.contains(input::JUMP) {
    **vel = Velocity::add(vel, &Velocity { speed: 5.0, angle: -0.5 * PI });
  }
}

fn run(&mut (_, ref mut vel, _): &mut (&mut Position, &mut Velocity, &HeroBehavior), (last_input, next_input): (input::Input, input::Input)) {
  if next_input.contains(input::RIGHT) && !last_input.contains(input::RIGHT) {
    **vel = Velocity::add(vel, &Velocity { speed: 3.0, angle: 0.0});
  }

  if !next_input.contains(input::RIGHT) && last_input.contains(input::RIGHT) {
    **vel = Velocity::add(vel, &Velocity { speed: 3.0, angle: PI });
  }

  if next_input.contains(input::LEFT) && !last_input.contains(input::LEFT) {
    **vel = Velocity::add(vel, &Velocity { speed: 3.0, angle: PI });
  }

  if !next_input.contains(input::LEFT) && last_input.contains(input::LEFT) {
    **vel = Velocity::add(vel, &Velocity { speed: 3.0, angle: 0.0 });
  }
}

impl NamedSystem<Context> for Hero {
  fn name(&self) -> &'static str {
    "behavior_hero"
  }
}
