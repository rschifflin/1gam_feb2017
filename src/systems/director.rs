use components;
use specs::{System, RunArg, Join};
use systems::NamedSystem;
use world::Context;
use itertools::Itertools;
use specs::{Entity, World};
use components::*;
use components::behavior::*;
use collider::geom::Shape;
use collider::geom::Vec2;
use geom::Rect;
use serde_json;
use std::fs::File;
use map;
use events;

pub struct Director;

impl System<Context> for Director {
  fn run(&mut self, arg: RunArg, _: Context) {
    let (game_states, heroes, blast_zones, mut game_events, phys_events) = arg.fetch(|w| {
      let mut game_states = w.write::<components::GameState>();
      let mut game_events = w.write_resource::<Vec<events::Game>>();
      let mut phys_events = w.write_resource::<Vec<events::Physics>>();
      let old_events = game_events.clone();
      game_events.clear();
      for event in old_events.iter() {
        let (mut game_state,) = (&mut game_states,).iter().next().unwrap();
        match *event {
          events::Game::Init => {
            game_events.push(events::Game::Level1);
          }
          events::Game::Level1 => {
            game_state.level = 1;
            phys_events.clear();
            delete_entities(w);
            create_entities(w, "./assets/testmap.json");
          },
          events::Game::Level2 => {
            game_state.level = 2;
            phys_events.clear();
            delete_entities(w);
            create_entities(w, "./assets/testmap2.json");
          }
        }
      }

      let heroes = w.read::<components::behavior::Hero>();
      let blast_zones = w.read::<components::BlastZone>();
      (game_states, heroes, blast_zones, game_events, phys_events)
    });

    let game_state = game_states.iter().next().unwrap();
    for event in phys_events.iter() {
      match *event {
        events::Physics::Collide(e1, e2) => {
          let (h1, h2) = (heroes.get(e1).is_some(), heroes.get(e2).is_some());
          let (b1, b2) = (blast_zones.get(e1).is_some(), blast_zones.get(e2).is_some());
          if (h1 && b2) || (h2 && b1) {
            if game_state.level == 1 { game_events.push(events::Game::Level2) }
            else { game_events.push(events::Game::Level1) };
          }
        }
      }
    }
  }
}

impl NamedSystem<Context> for Director {
  fn name(&self) -> &'static str {
    "director"
  }
}

fn delete_entities(world: &World) {
  let essentials = world.read::<components::Essential>();
  for (_, e) in (!&essentials, &world.entities()).iter() {
    world.delete_later(e)
  };
}

fn create_entities(world: &World, map_file: &'static str) {
  let map: map::Map = File::open(map_file)
    .map_err(|e| e.into())
    .and_then(serde_json::from_reader)
    .unwrap();

  let hero = create_hero(world, &map);
  world
    .create_later_build()
    .with::<Camera>(Camera {
      target: hero.clone(),
      screen: Rect::new(0.0,0.0,640.0,480.0),
      bounds: Rect::new(0.0,0.0,1920.0,1080.0)
    })
    .build(); //Initial Camera

  world
    .create_later_build()
    .with::<Position>(Position { x: 30.0, y: 30.0 })
    .with::<Collision>(Collision { bounds: Shape::new_rect(Vec2::new(16.0, 16.0)) } )
    .with::<Physical>(Physical {})
    .with::<Sprite>(Sprite {})
    .with::<Velocity>(Velocity::zero())
    .build(); //Some non-hero object

  create_floors(world, &map);
  create_blast_zone(world, &map);
}

fn create_hero(world: &World, map: &map::Map) -> Entity {
  let (start_x, start_y) = map.layers
    .iter()
    .find(|layer| layer.layer_type == map::LayerType::ObjectGroup)
    .and_then(|layer| {
      layer.objects
        .as_ref()
        .and_then(|objects| objects
          .iter()
          .find(|obj| obj.object_type == map::ObjectType::Start))
      .map(|obj| (obj.x as f64, obj.y as f64))
    }).unwrap();

  world
    .create_later_build()
    .with::<Position>(Position { x: start_x, y: start_y })
    .with::<Collision>(Collision { bounds: Shape::new_rect(Vec2::new(16.0, 16.0)) } )
    .with::<Physical>(Physical {})
    .with::<Sprite>(Sprite {})
    .with::<Hero>(Hero::new())
    .with::<Velocity>(Velocity::zero())
    .build() //Hero
}

fn create_blast_zone(world: &World, map: &map::Map) {
  let (x, y, w, h) = map.layers
    .iter()
    .find(|layer| layer.layer_type == map::LayerType::ObjectGroup)
    .and_then(|layer| {
      layer.objects
        .as_ref()
        .and_then(|objects| objects
          .iter()
          .find(|obj| obj.object_type == map::ObjectType::BlastZone))
      .map(|obj| (obj.x as f64, obj.y as f64, obj.width as f64, obj.height as f64))
    }).unwrap();

  world
    .create_later_build()
    .with::<Position>(Position { x: x, y: y })
    .with::<Collision>(Collision { bounds: Shape::new_rect(Vec2::new(w, h)) } )
    .with::<Velocity>(Velocity::zero())
    .with::<BlastZone>(BlastZone {})
    .build(); //Blast Zone
}

fn create_floors(world: &World, map: &map::Map) {
  let (tile_w, tile_h) = (map.tilewidth as f64, map.tileheight as f64);
  map.layers
    .iter()
    .find(|layer| layer.layer_type == map::LayerType::TileLayer)
    .and_then(|layer| layer.data.as_ref().map(|data| {
      data.chunks(map.height).enumerate().foreach(|(row_index, row)| {
        row.iter().enumerate().foreach(|(col_index, id)| {
          if *id == 7 {
            world
              .create_later_build()
              .with::<Position>(Position { x: col_index as f64 * tile_w, y: row_index as f64 * tile_h })
              .with::<Collision>(Collision { bounds: Shape::new_rect(Vec2::new(tile_w, tile_h)) } )
              .with::<Velocity>(Velocity::zero())
              .build(); //Floor block
          }
        })
      })
    })).unwrap_or_else(|| ());
}
