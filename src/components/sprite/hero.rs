use graphics;
use super::Sprite;

pub fn draw(sprite: &Sprite, x: f64, y: f64) -> Option<graphics::Image> {
  let image = graphics::image::Image::new().rect([x, y, 32.0, 32.0]);
  if sprite.flip {
    Some(image.src_rect([544.0, 0.0, -256.0, 256.0]))
  } else {
    Some(image.src_rect([288.0, 0.0, 256.0, 256.0]))
  }
}
