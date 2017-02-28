use specs::{Entity, World};
use components::*;
use components::behavior::*;
use events;
use systems::physics::{COLLIDE_GRANULARITY, COLLIDE_PADDING};
use collider::Collider;
use components::collision::CGroup;
use std::collections::HashMap;

mod context;
pub use self::context::Context;
pub fn register(world: &mut World) {
  world.register::<Collision>();
  world.register::<Physical>();
  world.register::<Position>();
  world.register::<Sprite>();
  world.register::<Velocity>();
  world.register::<Hero>();
  world.register::<Bird>();
  world.register::<Camera>();
  world.register::<BlastZone>();
  world.register::<Essential>();
  world.register::<GameState>();
  world.register::<Deadly>();
  world.register::<Checkpoint>();
  world.register::<StaticGeom>();
  world.register::<Song>();
  world.register::<Enemy>();

  world //Initial Game State
    .create_now()
    .with::<GameState>(GameState::default())
    .with::<Essential>(Essential {})
    .build();

  let phys_events: Vec<events::Physics> = vec![];
  world.add_resource(phys_events);

  let hero_events: Vec<events::Hero> = vec![];
  world.add_resource(hero_events);

  let camera_events: Vec<events::Camera> = vec![];
  world.add_resource(camera_events);

  let collider: Collider<CGroup> = Collider::new(COLLIDE_GRANULARITY, COLLIDE_PADDING);
  let collider_lookup: HashMap<u64, Entity> = HashMap::new();
  world.add_resource((collider, collider_lookup));

  let mut game_events: Vec<events::Game> = vec![];
  game_events.push(events::Game::Init);
  world.add_resource(game_events);
}
