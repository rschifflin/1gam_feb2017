use components;
use specs::{System, RunArg, Join};
use systems::NamedSystem;
use world::Context;
use events;

pub struct Camera;

impl System<Context> for Camera {
  fn run(&mut self, arg: RunArg, _: Context) {
    let (positions, mut cameras) = arg.fetch(|w| {
      let mut events = w.write_resource::<Vec<events::Camera>>();
      let pos = w.read::<components::Position>();
      let mut cams = w.write::<components::Camera>();
      for event in events.iter() {
        match *event {
          events::Camera::Switch(old, new) => {
            for (camera,) in (&mut cams,).iter() { if camera.target == old { camera.target = new } }
          }
        }
      }
      events.clear();
      (pos, cams)
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
