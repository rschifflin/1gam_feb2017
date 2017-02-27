use components::{Position, Song};
use components::behavior::Bird as BirdBehavior;
use specs::{Entity, System, RunArg, Join};
use systems::NamedSystem;
use world::Context;
use events;

pub struct Bird;
const SING_RANGE: f64 = 200.0;

impl System<Context> for Bird {
  fn run(&mut self, arg: RunArg, context: Context) {
    let (entities, mut birds, mut positions, mut songs, hero_events, mut game_events) = arg.fetch(|w| {
      let birds = w.write::<BirdBehavior>();
      let pos = w.write::<Position>();
      let songs = w.write::<Song>();
      let hero_events = w.read_resource::<Vec<events::Hero>>();
      let game_events = w.write_resource::<Vec<events::Game>>();
      (w.entities(), birds, pos, songs, hero_events, game_events)
    });

    //Sing event checking
    for (bird_ent, mut bird) in (&entities, &mut birds).iter() {
      for hero_event in hero_events.iter() {
        match *hero_event {
          events::Hero::Singing(hero_ent, progress) => {
            let is_near: bool = positions.get(bird_ent).and_then(|bird_pos| {
              positions.get(hero_ent).map(|hero_pos| {
                bird_pos.distance_to(hero_pos) <= SING_RANGE
              })
            }).unwrap_or(false);

            if is_near { bird.heard_song(progress) }
          },
          _ => ()
        }
      }
    }

    for (entity, bird) in (&entities, &mut birds).iter() {
      bird.update();
      if bird.should_sing() {
        songs.insert(arg.create(), Song::new(entity, bird.reward_progress));
        game_events.push(events::Game::UpdateProgress(bird.reward_progress));
      }
    }
  }
}

impl NamedSystem<Context> for Bird {
  fn name(&self) -> &'static str {
    "behavior_bird"
  }
}
