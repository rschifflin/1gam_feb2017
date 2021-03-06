use graphics::types::{Rectangle, SourceRectangle};
use super::Sprite;

pub fn draw(sprite: &Sprite, x: f64, y: f64) -> Option<(Rectangle, SourceRectangle)> {
  let rect = [x, y, 64.0, 128.0];
  let source_rect = if sprite.flip {
    [256.0, 0.0, -256.0, 512.0]
  } else {
    [0.0, 0.0, 256.0, 512.0]
  };
  Some((rect, source_rect))
}
