use specs::{Entity, World};
use components::*;
use components::behavior::*;
use std::f64::consts::PI;
use collider::geom::Shape;
use collider::geom::Vec2;
use itertools::Itertools;
use geom::Rect;
use map;
use events;

mod context;
pub use self::context::Context;
pub fn register(world: &mut World) {
  world.register::<Collision>();
  world.register::<Physical>();
  world.register::<Position>();
  world.register::<Sprite>();
  world.register::<Velocity>();
  world.register::<Hero>();
  world.register::<Camera>();
  world.register::<BlastZone>();
  world.register::<Essential>();
  world.register::<GameState>();

  world //Initial Game State
    .create_now()
    .with::<GameState>(GameState { level: 1 })
    .with::<Essential>(Essential {})
    .build();

  let phys_events: events::Events<events::Physics> = events::Events::new();
  world.add_resource(phys_events);

  let mut game_events: events::Events<events::Game> = events::Events::new();
  game_events.push(events::Game::Init);
  world.add_resource(game_events);
}
