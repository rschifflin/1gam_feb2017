use graphics::types::{Rectangle, SourceRectangle};
use super::Sprite;

pub fn draw(sprite: &Sprite, x: f64, y: f64) -> Option<(Rectangle, SourceRectangle)> {
  let rect = [x, y, 32.0, 32.0];
  let source_rect = if sprite.flip {
    [896.0, 192.0, 128.0, 128.0]
  } else {
    [1024.0, 192.0, -128.0, 128.0]
  };
  Some((rect, source_rect))
}
