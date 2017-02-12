use components::{Physical, Position, Collision, Velocity};
use specs::{Entity, Entities, Allocator, System, RunArg, Join, Storage, MaskedStorage};
use specs::UnprotectedStorage;
use systems::NamedSystem;
use world::Context;
use std::f64::consts::PI;
use collider::{Event, Collider, Hitbox};
use collider::geom::{PlacedShape, vec2};
use input;

pub struct Physics;
const MAX_SPEED: f64 = 10.0;
const NUDGE_PADDING: f64 = 0.001;
const COLLIDE_PADDING: f64 = NUDGE_PADDING * 0.01;
const GRAVITY: Velocity = Velocity { speed: 0.1, angle: 0.5*PI };

impl System<Context> for Physics {
  fn run(&mut self, arg: RunArg, context: Context) {
    let (mut entities, mut positions, mut velocities, collisions, physicals) = arg.fetch(|w| {
      let pos = w.write::<Position>();
      let vel = w.write::<Velocity>();
      let col = w.read::<Collision>();
      let phys = w.read::<Physical>();
      (w.entities(), pos, vel, col, phys)
    });

    for mut val in (&mut positions, &mut velocities, &physicals).iter() {
      gravitate(&mut val);
    }

    let mut collider: Collider = Collider::new(12.0, COLLIDE_PADDING);
    for (mut eid, ref mut pos, ref mut vel, ref col) in (&entities, &mut positions, &mut velocities, &collisions).iter() {
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
      collider.add_hitbox(eid.get_id() as u64, hitbox);
    }

    let mut entity_key = &entities;
    while collider.time() < 1.0 {
      let next_time = collider.next_time().min(1.0);
      collider.set_time(next_time);
      while let Some((event, e1, e2)) = collider.next() {
        match event {
          Event::Collide => {
            let mut h1 = collider.get_hitbox(e1);
            let h2 = collider.get_hitbox(e2);
            let adjustment = h1.shape.normal_from(&h2.shape);
            h1.shape.pos.x += (adjustment.dir().x * adjustment.len());
            h1.shape.pos.y += (adjustment.dir().y * adjustment.len());

            // Bump perfectly-adjacent hitboxes and kill velocity in the travelled dir
            if adjustment.dir().x != 0.0 {
              h1.vel.pos.x = 0.0;
              h1.shape.pos.x += (adjustment.dir().x.signum() * NUDGE_PADDING);
            }

            if adjustment.dir().y != 0.0 {
              h1.vel.pos.y = 0.0;
              h1.shape.pos.y += (adjustment.dir().y.signum() * NUDGE_PADDING);
            }
            collider.update_hitbox(e1, h1);
          },
          Event::Separate => { }
        }
      }
    };

    for (eid, ref mut pos, ref mut vel, ref col) in (&entities, &mut positions, &mut velocities, &collisions).iter() {
      let mut hitbox = collider.get_hitbox(eid.get_id() as u64);
      **vel = Velocity::from_cart((hitbox.vel.pos.x, hitbox.vel.pos.y));
      pos.x = hitbox.shape.left();
      pos.y = hitbox.shape.bottom();
    }
  }
}

impl NamedSystem<Context> for Physics {
  fn name(&self) -> &'static str {
    "physics"
  }
}

fn gravitate(&mut (ref mut pos, ref mut vel, _): &mut (&mut Position, &mut Velocity, &Physical)) {
  **vel = Velocity::add(vel, &GRAVITY);
  vel.speed = (vel.speed.min(MAX_SPEED) * 1000.0).round() / 1000.0;
}
