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
  if next_input.contains(input::UP) && !last_input.contains(input::UP) {
    let (x, y) = vel.to_cart();
    **vel = Velocity::from_cart((x, -5.0));
  }
}

fn run(&mut (_, ref mut vel, _): &mut (&mut Position, &mut Velocity, &HeroBehavior), (last_input, next_input): (input::Input, input::Input)) {
  if next_input.contains(input::RIGHT) {
    let (x, y) = vel.to_cart();
    **vel = Velocity::from_cart((3.0, y));
  }

  if !next_input.contains(input::RIGHT) {
    let (x, y) = vel.to_cart();
    **vel = Velocity::from_cart((x.min(0.0), y));
  }

  if next_input.contains(input::LEFT) {
    let (x, y) = vel.to_cart();
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
