use specs::World;
use components::*;
use components::behavior::*;
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

  let phys_events: Vec<events::Physics> = vec![];
  world.add_resource(phys_events);

  let mut game_events: Vec<events::Game> = vec![];
  game_events.push(events::Game::Init);
  world.add_resource(game_events);
}
