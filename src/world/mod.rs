use specs::{Entity, World};
use components::*;
use std::f64::consts::PI;

mod context;
pub use self::context::Context;
pub fn register(world: &mut World) {
  world.register::<Collision>();
  world.register::<Physical>();
  world.register::<Position>();
  world.register::<Sprite>();
  world.register::<Velocity>();

}
pub fn create_initial_entities(world: &mut World) -> (Entity,) {
  let init_vel = Velocity { speed: 5.0, angle: -0.25 * PI };
  let entity = world
    .create_now()
    .with::<Position>(Position { x: 10.0, y: 400.0 })
    .with::<Collision>(Collision { bounds: vec![]  })
    .with::<Physical>(Physical {})
    .with::<Sprite>(Sprite {})
    .with::<Velocity>(init_vel)
    .build();
  (entity,)
}
