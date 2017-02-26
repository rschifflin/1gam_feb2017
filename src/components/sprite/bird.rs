use graphics;
use super::Sprite;

pub fn draw(sprite: &Sprite, x: f64, y: f64) -> Option<graphics::Image> {
  let image = graphics::image::Image::new().rect([x, y, 64.0, 128.0]);
  if sprite.flip {
    Some(image.src_rect([256.0, 0.0, -256.0, 512.0]))
  } else {
    Some(image.src_rect([0.0, 0.0, 256.0, 512.0]))
  }
}
