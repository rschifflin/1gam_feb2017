use components;
use specs::{System, RunArg, Join};
use systems::NamedSystem;
use world::Context;
use events;

pub struct Sprite;

impl System<Context> for Sprite {
  fn run(&mut self, arg: RunArg, _: Context) {
    let mut sprites = arg.fetch(|w| {
      w.write::<components::Sprite>()
    });

    for mut sprite in (&mut sprites).iter() {
      sprite.update();
    }
  }
}

impl NamedSystem<Context> for Sprite {
  fn name(&self) -> &'static str {
    "sprite"
  }
}
