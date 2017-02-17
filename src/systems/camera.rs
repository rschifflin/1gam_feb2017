use components;
use specs::{System, RunArg, Join};
use systems::NamedSystem;
use world::Context;

pub struct Camera;

impl System<Context> for Camera {
  fn run(&mut self, arg: RunArg, _: Context) {
    let (positions, mut cameras) = arg.fetch(|w| {
      let pos = w.read::<components::Position>();
      let cam = w.write::<components::Camera>();
      (pos, cam)
    });

    for cam in (&mut cameras).iter() {
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
