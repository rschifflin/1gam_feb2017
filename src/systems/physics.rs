use components::{Physical, Position, Collision, Velocity};
use components::collision::CGroup;
use specs::{Entity, System, RunArg, Join};
use std::collections::{HashMap, HashSet};
use systems::NamedSystem;
use world::Context;
use std::f64::consts::PI;
use collider::{Event, Collider, Hitbox};
use collider::geom::{PlacedShape, vec2};
use events;

pub struct Physics;
const MAX_SPEED: f64 = 10.0;
const NUDGE_PADDING: f64 = 0.01;
const COLLIDE_PADDING: f64 = NUDGE_PADDING * 0.01;
const GRAVITY: Velocity = Velocity { speed: 0.1, angle: 0.5*PI };

impl System<Context> for Physics {
  fn run(&mut self, arg: RunArg, _: Context) {
    let (entities, mut positions, mut velocities, collisions, physicals, mut events) = arg.fetch(|w| {
      let pos = w.write::<Position>();
      let vel = w.write::<Velocity>();
      let col = w.read::<Collision>();
      let phys = w.read::<Physical>();
      let events = w.write_resource::<Vec<events::Physics>>();
      (w.entities(), pos, vel, col, phys, events)
    });

    events.clear();
    let mut lookup_table: HashMap<u64, Entity> = HashMap::new();
    let mut buffered_events = HashSet::new();

    for mut val in (&mut positions, &mut velocities, &physicals).iter() {
      gravitate(&mut val);
    }

    let mut collider: Collider<CGroup> = Collider::new(12.0, COLLIDE_PADDING);
    for (eid, ref mut pos, ref mut vel, ref col) in (&entities, &mut positions, &mut velocities, &collisions).iter() {
      let (vel_x, vel_y) = vel.to_cart();
      let hitbox_bounds = col.bounds;
      let (bounds_w, bounds_h) = {
        let dims = col.bounds.dims();
        let w = dims.x;
        let h = dims.y;
        (w, h)
      };
      let hitbox_pos = vec2(pos.x + bounds_w/2.0, pos.y + bounds_h/2.0);

      let mut hitbox = Hitbox::new(PlacedShape::new(hitbox_pos, hitbox_bounds));
      hitbox.vel.pos = vec2(vel_x, vel_y);
      let id = id_for(&eid, col);
      lookup_table.insert(id, eid.clone());
      collider.add_hitbox_with_interactivity(id, hitbox, col.group);
    }

    while collider.time() < 1.0 {
      let next_time = collider.next_time().min(1.0);
      collider.set_time(next_time);
      while let Some((event, e1, e2)) = collider.next() {
        match event {
          Event::Collide => {
            let mut h1 = collider.get_hitbox(e1);
            let mut h2 = collider.get_hitbox(e2);
            let adjustment = h1.shape.normal_from(&h2.shape);
            h1.shape.pos.x += adjustment.dir().x * adjustment.len();
            h1.shape.pos.y += adjustment.dir().y * adjustment.len();

            // Bump perfectly-adjacent hitboxes and kill velocity in the travelled dir
            if adjustment.dir().x != 0.0 {
              h1.vel.pos.x = 0.0;
              h2.vel.pos.x = 0.0;
              h1.shape.pos.x += adjustment.dir().x.signum() * NUDGE_PADDING;
            }

            if adjustment.dir().y != 0.0 {
              h1.vel.pos.y = 0.0;
              h2.vel.pos.y = 0.0;
              h1.shape.pos.y += adjustment.dir().y.signum() * NUDGE_PADDING;
            }
            collider.update_hitbox(e1, h1);
          },
          Event::Separate => { buffered_events.insert((e1, e2)); }
        }
      }
    };

    for (eid, ref mut pos, ref mut vel, ref col) in (&entities, &mut positions, &mut velocities, &collisions).iter() {
      let hitbox = collider.get_hitbox(id_for(&eid, col));
      **vel = Velocity::from_cart((hitbox.vel.pos.x, hitbox.vel.pos.y));
      pos.x = hitbox.shape.left();
      pos.y = hitbox.shape.bottom();
    }

    for &(ref e1, ref e2) in buffered_events.iter() {
      let eid1 = lookup_table.get(e1).unwrap();
      let eid2 = lookup_table.get(e2).unwrap();
      events.push(events::Physics::Collide(eid1.clone(), eid2.clone()))
    }
  }
}

impl NamedSystem<Context> for Physics {
  fn name(&self) -> &'static str {
    "physics"
  }
}

fn gravitate(&mut (_, ref mut vel, _): &mut (&mut Position, &mut Velocity, &Physical)) {
  **vel = Velocity::add(vel, &GRAVITY);
  vel.speed = (vel.speed.min(MAX_SPEED) * 1000.0).round() / 1000.0;
}

fn id_for(e: &Entity, c: &Collision) -> u64 {
  let priority = c.priority;
  e.get_id() as u64 | ((priority as u64) << 32)
}
