use specs::{Entity, World};
use components::*;
use components::behavior::*;
use std::f64::consts::PI;
use collider::geom::Shape;
use collider::geom::Vec2;

mod context;
pub use self::context::Context;
pub fn register(world: &mut World) {
  world.register::<Collision>();
  world.register::<Physical>();
  world.register::<Position>();
  world.register::<Sprite>();
  world.register::<Velocity>();
  world.register::<Velocity>();
  world.register::<Hero>();
}

pub fn create_initial_entities(world: &mut World) -> (Entity,) {
  let entity = world
    .create_now() //Hero
    .with::<Position>(Position { x: 10.0, y: 10.0 })
    .with::<Collision>(Collision { bounds: Shape::new_rect(Vec2::new(10.0, 10.0)) } )
    .with::<Physical>(Physical {})
    .with::<Sprite>(Sprite {})
    .with::<Hero>(Hero::new())
    .with::<Velocity>(Velocity::zero())
    .build();

  world //Some non-hero object
    .create_now()
    .with::<Position>(Position { x: 30.0, y: 30.0 })
    .with::<Collision>(Collision { bounds: Shape::new_rect(Vec2::new(10.0, 10.0)) } )
    .with::<Physical>(Physical {})
    .with::<Sprite>(Sprite {})
    .with::<Velocity>(Velocity::zero())
    .build();

  world //Floor
    .create_now()
    .with::<Position>(Position { x: 0.0, y: 300.0 })
    .with::<Collision>(Collision { bounds: Shape::new_rect(Vec2::new(1000.0, 50.0)) } )
    .with::<Velocity>(Velocity::zero())
    .build();
  (entity,)
}
