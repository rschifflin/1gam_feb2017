use graphics;
use specs::{Component, VecStorage};

#[derive(Debug, Copy, Clone)]
pub enum Sprite {
  Hero,
  Bird
}

impl Component for Sprite {
  type Storage = VecStorage<Sprite>;
}

impl Sprite {
  pub fn as_image(&self) -> graphics::Image {
    match *self {
      Sprite::Hero => {
        graphics::image::Image::new()
          .rect([0.0, 0.0, 38.0, 24.0])
          .src_rect([342.0, 0.0, 812.0, 512.0])
      },
      Sprite::Bird => {
        graphics::image::Image::new()
          .rect([0.0, 0.0, 32.0, 128.0])
          .src_rect([0.0, 0.0, 330.0, 720.0])
      }
    }
  }
}
