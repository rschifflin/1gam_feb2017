use components;
use specs::{Allocator, System, RunArg, Join, Storage, MaskedStorage};
use specs::UnprotectedStorage;
use systems::NamedSystem;
use world::Context;

pub struct Camera;

impl System<Context> for Camera {
  fn run(&mut self, arg: RunArg, context: Context) {
    let (positions, mut cameras) = arg.fetch(|w| {
      let pos = w.read::<components::Position>();
      let cam = w.write::<components::Camera>();
      (pos, cam)
    });

    for mut cam in (&mut cameras).iter() {
      let pos = positions.get(cam.target);
      pos.map(|p| cam.center(p.x, p.y));
    }
  }
}

impl NamedSystem<Context> for Camera {
  fn name(&self) -> &'static str {
    "camera"
  }
}
