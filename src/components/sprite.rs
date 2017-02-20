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
  pub fn as_image(&self, x: f64, y: f64) -> graphics::Image {
    match *self {
      Sprite::Hero => {
        graphics::image::Image::new()
          .rect([x, y, 32.0, 32.0])
          .src_rect([256.0, 0.0, 256.0, 256.0])
      },
      Sprite::Bird => {
        graphics::image::Image::new()
          .rect([x, y, 64.0, 128.0])
          .src_rect([0.0, 0.0, 256.0, 512.0])
      }
    }
  }
}
