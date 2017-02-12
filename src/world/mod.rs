use specs::{Entity, World};
use components::*;
use components::behavior::*;
use std::f64::consts::PI;
use collider::geom::Shape;
use collider::geom::Vec2;
use itertools::Itertools;
use map;

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

pub fn create_initial_entities(world: &mut World, map: &map::Map) -> (Entity,) {
  let entity = create_hero(world, map);
  world //Some non-hero object
    .create_now()
    .with::<Position>(Position { x: 30.0, y: 30.0 })
    .with::<Collision>(Collision { bounds: Shape::new_rect(Vec2::new(10.0, 10.0)) } )
    .with::<Physical>(Physical {})
    .with::<Sprite>(Sprite {})
    .with::<Velocity>(Velocity::zero())
    .build();
  create_floors(world, map);
  (entity,)
}

fn create_hero(world: &mut World, map: &map::Map) -> Entity {
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

  /*
  let ref (start_x, start_y) = map.layers
    .iter()
    .find(|layer| layer.layer_type == map::LayerType::ObjectGroup)
    .map(|layer| {
      &layer.objects
        .unwrap()
        .iter()
          .find(|obj| obj.object_type == map::ObjectType::Start)
          .map(|obj| (obj.x as f64, obj.y as f64))
          .unwrap()
    })
    .unwrap();
  */

  world
    .create_now() //Hero
    .with::<Position>(Position { x: start_x, y: start_y })
    .with::<Collision>(Collision { bounds: Shape::new_rect(Vec2::new(10.0, 10.0)) } )
    .with::<Physical>(Physical {})
    .with::<Sprite>(Sprite {})
    .with::<Hero>(Hero::new())
    .with::<Velocity>(Velocity::zero())
    .build()
}

fn create_floors(world: &mut World, map: &map::Map) {
  let (tile_w, tile_h) = (map.tilewidth as f64, map.tileheight as f64);
  map.layers
    .iter()
    .find(|layer| layer.layer_type == map::LayerType::TileLayer)
    .and_then(|layer| layer.data.as_ref().map(|data| {
      data.chunks(map.height).enumerate().foreach(|(row_index, row)| {
        row.iter().enumerate().foreach(|(col_index, id)| {
          if *id == 7 {
            world
              .create_now()
              .with::<Position>(Position { x: col_index as f64 * tile_w, y: row_index as f64 * tile_h })
              .with::<Collision>(Collision { bounds: Shape::new_rect(Vec2::new(tile_w, tile_h)) } )
              .with::<Velocity>(Velocity::zero())
              .build();
          }
        })
      })
    })).unwrap_or_else(|| ());
}
