use graphics::types::{Rectangle, SourceRectangle};
use super::Sprite;

pub fn draw(sprite: &Sprite, x: f64, y: f64) -> Option<(Rectangle, SourceRectangle)> {
  let rect = [x, y, 64.0, 64.0];
  let source_rect = [576.0, 320.0, 288.0, 288.0];
  Some((rect, source_rect))
}
