use components;
use specs::{Entity, World, System, RunArg, Join};
use systems::NamedSystem;
use world::Context;
use itertools::Itertools;
use components::*;
use components::collision::{Priority, CGroup};
use components::behavior::*;
use collider::geom::{PlacedShape, Shape, Vec2, vec2};
use collider::{Collider, Hitbox};
use geom::Rect;
use serde_json;
use systems::physics::{COLLIDE_GRANULARITY, COLLIDE_PADDING};
use std::fs::File;
use std::collections::HashMap;
use progress::{self, Progress};
use map;
use events;
use systems;

use fsm::enemy::{UpDownFSM, LeftRightFSM, ArcFSM, WanderFSM};

pub struct Director;

impl System<Context> for Director {
  fn run(&mut self, arg: RunArg, _: Context) {
    let (game_states, heroes, blast_zones, mut game_events, phys_events) = arg.fetch(|w| {
      let mut game_states = w.write::<components::GameState>();
      let hero_events = w.read_resource::<Vec<events::Hero>>();
      let mut camera_events = w.write_resource::<Vec<events::Camera>>();
      let mut game_events = w.write_resource::<Vec<events::Game>>();
      let mut phys_events = w.write_resource::<Vec<events::Physics>>();
      let mut static_data = w.write_resource::<(Collider<CGroup>, HashMap<u64, Entity>)>();
      let old_events = game_events.clone();
      game_events.clear();

      //Locking Game State
      for event in old_events.iter() {
        let (mut game_state,) = (&mut game_states,).iter().next().unwrap();
        match *event {
          events::Game::Init => {
            game_events.push(events::Game::NextLevel);
          },
          events::Game::UpdateProgress(progress) => {
            game_state.progress |= progress;
          },
          events::Game::NextLevel => {
            game_state.level += 1;
            phys_events.clear();
            camera_events.clear();
            delete_entities(w);
            let level = game_state.level;
            create_entities(w, &mut game_state, &mut static_data, format!("./assets/level{}.json", level));
          },
        }
      }

      for event in hero_events.iter() {
        let (mut game_state,) = (&mut game_states,).iter().next().unwrap();
        match *event {
          events::Hero::Dead(dead_hero) => {
            w.delete_later(dead_hero);
            let new_hero = create_hero(w, game_state);
            camera_events.push(events::Camera::Switch(dead_hero, new_hero));
          },
          events::Hero::Checkpoint(new_spawn) => game_state.spawn = new_spawn,
          _ => ()
        }
      }

      let mut heroes = w.write::<components::behavior::Hero>();
      let blast_zones = w.read::<components::BlastZone>();

      for event in old_events.iter() {
        match *event {
          events::Game::UpdateProgress(progress) => {
            for mut hero in (&mut heroes).iter() {
              *hero = Hero::new(progress);
            }
          },
          _ => ()
        }
      }

      (game_states, heroes, blast_zones, game_events, phys_events)
    });

    let game_state = game_states.iter().next().unwrap();

    for event in phys_events.iter() {
      match *event {
        events::Physics::Collide(e1, e2) => {
          let (h1, h2) = (heroes.get(e1).is_some(), heroes.get(e2).is_some());
          let (b1, b2) = (blast_zones.get(e1).is_some(), blast_zones.get(e2).is_some());
          if (h1 && b2) || (h2 && b1) { game_events.push(events::Game::NextLevel) }
        },
        _ => ()
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

fn create_entities(world: &World, game_state: &mut GameState, &mut (ref mut collider, ref mut lookup): &mut (Collider<CGroup>, HashMap<u64, Entity>), map_file: String) {
  //Empty the statics
  *collider = Collider::new(COLLIDE_GRANULARITY, COLLIDE_PADDING);
  lookup.clear();

  let map: map::Map = File::open(map_file)
    .map_err(|e| e.into())
    .and_then(serde_json::from_reader)
    .unwrap();
  let spawn = find_start(&map);
  game_state.spawn = spawn;
  let hero = create_hero(world, game_state);
  world
    .create_later_build()
    .with::<Camera>(Camera {
      target: hero.clone(),
      screen: Rect::new(0.0,0.0,852.0,480.0),
      bounds: Rect::new(0.0, 0.0, (map.width * map.tilewidth) as f64, (map.height * map.tileheight) as f64)
    })
    .build(); //Initial Camera

  create_from_object_layer(world, &map, collider, lookup);
  create_from_tile_layer(world, &map, collider, lookup);
  create_boundaries(world, &map, collider, lookup);
}

fn find_start(map: &map::Map) -> (f64, f64) {
  let (tile_w, tile_h) = (map.tilewidth as f64, map.tileheight as f64);
  map.layers
    .iter()
    .find(|layer| layer.layer_type == map::LayerType::TileLayer)
    .and_then(|layer| layer.data.as_ref().and_then(|data| {
      data.chunks(map.height)
        .enumerate()
        .map(|(row_index, row)| {
          row.iter().enumerate()
            .find(|&(col_index, id)| map::tile_from_id(*id) == map::Tile::HeroStart)
            .map(|(col_index, _)| (col_index as f64 * tile_w, row_index as f64 * tile_h))
        }).filter_map(|opt| opt)
          .next()
    })).unwrap()
}

fn create_hero(world: &World, game_state: &GameState) -> Entity {
  let (start_x, start_y) = game_state.spawn;
  let progress = game_state.progress;

  world
    .create_later_build()
    .with::<Position>(Position { x: start_x, y: start_y })
    .with::<Collision>(Collision {
      bounds: Shape::new_rect(Vec2::new(30.0, 30.0)),
      priority: Priority::Low,
      group: CGroup::Friendly
    })
    .with::<Physical>(Physical {})
    .with::<Sprite>(Sprite::new(Graphic::Hero, Layer::Layer4))
    .with::<Hero>(Hero::new(progress))
    .with::<Velocity>(Velocity::zero())
    .build() //Hero
}

fn create_from_object_layer(world: &World, map: &map::Map, collider: &mut Collider<CGroup>, lookup: &mut HashMap<u64, Entity>) {
  let (tile_w, tile_h) = (map.tilewidth as f64, map.tileheight as f64);
  map.layers
    .iter()
    .find(|layer| layer.layer_type == map::LayerType::ObjectGroup)
    .map(|layer| {
      layer.objects
        .as_ref()
        .map(|objects| objects
          .iter()
          .foreach(|obj| {
              let (x, y) = (obj.x as f64, obj.y as f64 - tile_h);
              match obj.object_type {
                map::ObjectType::Bird => {
                  let requisite_progress = obj.properties.requisite_progress
                    .map(|p| Progress::from_bits_truncate(p))
                    .unwrap_or(Progress::empty());
                  let reward_progress = obj.properties.reward_progress
                    .map(|p| Progress::from_bits_truncate(p))
                    .unwrap_or(progress::DASH);
                  world
                    .create_later_build()
                    .with::<Position>(Position { x: x, y: y })
                    .with::<Sprite>(Sprite::new(Graphic::Bird, Layer::Layer2))
                    .with::<Bird>(Bird::new(requisite_progress, reward_progress))
                    .build(); //Bird
                },
                map::ObjectType::LeftRight => {
                  let speed = obj.properties.speed.unwrap_or(4.0);
                  world
                    .create_later_build()
                    .with::<Position>(Position { x: x + 0.1, y: y + 0.1})
                    .with::<Collision>(Collision {
                      bounds: Shape::new_rect(Vec2::new(31.8, 31.8)),
                      priority: Priority::Enemy,
                      group: CGroup::Enemy
                    })
                    .with::<Sprite>(Sprite::new(Graphic::Spikeblock, Layer::Layer6))
                    .with::<Velocity>(Velocity { x: 0.0, y: 0.0 } )
                    .with::<Deadly>(Deadly {})
                    .with::<Enemy>(Enemy::new(
                        Box::new(
                          LeftRightFSM::new(true, speed)
                        )
                      )
                    )
                    .build(); // Left<->Right spikey
                },
                map::ObjectType::UpDown => {
                  let speed = obj.properties.speed.unwrap_or(4.0);
                  world
                    .create_later_build()
                    .with::<Position>(Position { x: x + 0.1, y: y + 0.1})
                    .with::<Collision>(Collision {
                      bounds: Shape::new_rect(Vec2::new(31.8, 31.8)),
                      priority: Priority::Enemy,
                      group: CGroup::Enemy
                    })
                    .with::<Sprite>(Sprite::new(Graphic::Spikeblock, Layer::Layer6))
                    .with::<Velocity>(Velocity { x: 0.0, y: 0.0 } )
                    .with::<Deadly>(Deadly {})
                    .with::<Enemy>(Enemy::new(
                        Box::new(
                          UpDownFSM::new(true, speed)
                        )
                      )
                    )
                    .build(); // Left<->Right spikey
                },
                map::ObjectType::Arc => {
                  let speed = obj.properties.speed.unwrap_or(0.1);
                  let radius = obj.properties.radius.unwrap_or(32.0);
                  let angle = obj.properties.angle.unwrap_or(0.0);
                  let flip = obj.properties.flip.unwrap_or(false);
                  world
                    .create_later_build()
                    .with::<Position>(Position { x: x + 0.1, y: y + 0.1})
                    .with::<Collision>(Collision {
                      bounds: Shape::new_rect(Vec2::new(31.8, 31.8)),
                      priority: Priority::Enemy,
                      group: CGroup::Enemy
                    })
                    .with::<Sprite>(Sprite::new(Graphic::Spikeblock, Layer::Layer6))
                    .with::<Velocity>(Velocity { x: 0.0, y: 0.0 } )
                    .with::<Deadly>(Deadly {})
                    .with::<Enemy>(Enemy::new(
                        Box::new(
                          ArcFSM::new(flip, radius, speed, angle)
                        )
                      )
                    )
                    .build(); // Left<->Right spikey
                },
                map::ObjectType::Pirate => {
                  let frequency = obj.properties.frequency.unwrap_or(1.0);
                  let speed = obj.properties.speed.unwrap_or(4.0);
                  world
                    .create_later_build()
                    .with::<Position>(Position { x: x + 0.1, y: y + 0.1})
                    .with::<Collision>(Collision {
                      bounds: Shape::new_rect(Vec2::new(31.8, 31.8)),
                      priority: Priority::Enemy,
                      group: CGroup::Enemy
                    })
                    .with::<Physical>(Physical {})
                    .with::<Sprite>(Sprite::new(Graphic::Enemy, Layer::Layer5))
                    .with::<Velocity>(Velocity { x: 0.0, y: 0.0 } )
                    .with::<Deadly>(Deadly {})
                    .with::<Enemy>(Enemy::new(
                        Box::new(
                          WanderFSM::new(frequency, speed)
                        )
                      )
                    )
                    .build(); //Pirate duder
                },
              }
          })
        )
    }).unwrap();

}

fn create_from_tile_layer(world: &World, map: &map::Map, collider: &mut Collider<CGroup>, lookup: &mut HashMap<u64, Entity>) {
  let (tile_w, tile_h) = (map.tilewidth as f64, map.tileheight as f64);
  map.layers
    .iter()
    .find(|layer| layer.layer_type == map::LayerType::TileLayer)
    .and_then(|layer| layer.data.as_ref().map(|data| {
      data.chunks(map.height).enumerate().foreach(|(row_index, row)| {
        row.iter().enumerate().foreach(|(col_index, id)| {
          match map::tile_from_id(*id) {
            map::Tile::Floor => {
              let pos = Position { x: col_index as f64 * tile_w, y: row_index as f64 * tile_h };
              let col = Collision {
                bounds: Shape::new_rect(Vec2::new(tile_w, tile_h)),
                priority: Priority::High,
                group: CGroup::Static
              };
              let (priority, group) = (col.priority, col.group);
              let hitbox_bounds = col.bounds;
              let (bounds_w, bounds_h) = {
                let dims = col.bounds.dims();
                let w = dims.x;
                let h = dims.y;
                (w, h)
              };
              let hitbox_pos = vec2(pos.x + bounds_w/2.0, pos.y + bounds_h/2.0);
              let mut hitbox = Hitbox::new(PlacedShape::new(hitbox_pos, hitbox_bounds));
              hitbox.vel.pos = vec2(0.0, 0.0);
              let eid = world
                .create_later_build()
                .with::<Position>(pos)
                .with::<Collision>(col)
                .with::<Sprite>(Sprite::new(Graphic::Block, Layer::Layer1))
                .with::<StaticGeom>(StaticGeom {})
                .build(); //Floor block
              let id = systems::physics::id_for(&eid, priority);
              lookup.insert(id, eid);
              collider.add_hitbox_with_interactivity(id, hitbox, group);
            },
            map::Tile::Checkpoint => {
              let (x, y) = (col_index as f64 * tile_w, row_index as f64 * tile_h);
              world
                .create_later_build()
                .with::<Position>(Position { x: x, y: y })
                .with::<Collision>(Collision {
                  bounds: Shape::new_rect(Vec2::new(32.0, 64.0)),
                  priority: Priority::High,
                  group: CGroup::Static
                })
                .with::<Sprite>(Sprite::new(Graphic::Checkpoint(false), Layer::Layer3))
                .with::<Checkpoint>(Checkpoint {})
                .build(); // Checkpoint
            },
            map::Tile::Spike(facing) => {
              let (x, y) = (col_index as f64 * tile_w + 0.1, row_index as f64 * tile_h + 0.1);
              world
                .create_later_build()
                .with::<Position>(Position { x: x, y: y })
                .with::<Collision>(Collision {
                  bounds: Shape::new_rect(Vec2::new(31.8, 31.8)),
                  priority: Priority::High,
                  group: CGroup::Static
                })
                .with::<Sprite>(Sprite::new(Graphic::Spikes(facing), Layer::Layer6))
                .with::<Deadly>(Deadly {})
                .build(); //Deadly deadly spikes
            },
            map::Tile::Portal => {
              let (x, y) = (col_index as f64 * tile_w, row_index as f64 * tile_h);
              world
                .create_later_build()
                .with::<Position>(Position { x: x, y: y })
                .with::<Collision>(Collision {
                  bounds: Shape::new_rect(Vec2::new(64.0, 64.0)),
                  priority: Priority::High,
                  group: CGroup::Static
                })
                .with::<Sprite>(Sprite::new(Graphic::Portal, Layer::Layer6))
                .with::<BlastZone>(BlastZone {})
                .build(); //Deadly deadly spikes
            },
            _ => ()
          }
        })
      })
    })).unwrap_or(());
}

fn create_boundaries(world: &World, map: &map::Map, collider: &mut Collider<CGroup>, lookup: &mut HashMap<u64, Entity>) {
  let thickness = 32.0;
  let map_width = (map.width * map.tilewidth) as f64;
  let map_height = (map.height * map.tileheight) as f64;

  for &(x,y,w,h) in [
    (-thickness, 0.0, thickness, map_height), //Left Boundary
    (map_width, 0.0, thickness, map_height), //Right Boundary
    (-thickness, -thickness, map_width + 2.0*thickness, thickness), //Top Boundary
    (-thickness, map_height, map_width + 2.0*thickness, thickness) //Bottom Boundary
  ].iter() {

    let pos = Position { x: x, y: y };
    let col = Collision {
      bounds: Shape::new_rect(Vec2::new(w, h)),
      priority: Priority::High,
      group: CGroup::Static
    };
    let (priority, group) = (col.priority, col.group);
    let hitbox_bounds = col.bounds;
    let (bounds_w, bounds_h) = {
      let dims = col.bounds.dims();
      let w = dims.x;
      let h = dims.y;
      (w, h)
    };
    let hitbox_pos = vec2(pos.x + bounds_w/2.0, pos.y + bounds_h/2.0);
    let mut hitbox = Hitbox::new(PlacedShape::new(hitbox_pos, hitbox_bounds));
    hitbox.vel.pos = vec2(0.0, 0.0);
    let eid = world
      .create_later_build()
      .with::<Position>(pos)
      .with::<Collision>(col)
      .with::<StaticGeom>(StaticGeom {})
      .build(); //Boundary block
    let id = systems::physics::id_for(&eid, priority);
    lookup.insert(id, eid);
    collider.add_hitbox_with_interactivity(id, hitbox, group);
  }
}
