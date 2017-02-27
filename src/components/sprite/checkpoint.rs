use graphics::types::{Rectangle, SourceRectangle};
use super::Sprite;

pub fn draw_unchecked(sprite: &Sprite, x: f64, y: f64) -> Option<(Rectangle, SourceRectangle)> {
  let rect = [x, y, 32.0, 64.0];
  let source_rect = if sprite.flip {
    [1216.0, 0.0, -128.0, 256.0]
  } else {
    [1088.0, 0.0, 128.0, 256.0]
  };
  Some((rect, source_rect))
}

pub fn draw_checked(sprite: &Sprite, x: f64, y: f64) -> Option<(Rectangle, SourceRectangle)> {
  let rect = [x, y, 32.0, 64.0];
  let source_rect = if sprite.flip {
    [1376.0, 0.0, -128.0, 256.0]
  } else {
    [1248.0, 0.0, 128.0, 256.0]
  };
  Some((rect, source_rect))
}
