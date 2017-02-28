use graphics::types::{Rectangle, SourceRectangle};
use super::Sprite;
use facing::Facing;

pub fn draw(sprite: &Sprite, facing: Facing, x: f64, y: f64) -> Option<(Rectangle, SourceRectangle)> {
  let rect = [x, y, 32.0, 32.0];
  let source_rect = match facing {
    Facing::Up    => [896.0, 352.0, 128.0, 128.0],
    Facing::Down  => [896.0, 480.0, 128.0, -128.0],
    Facing::Left  => [1216.0, 352.0, 128.0, 128.0],
    Facing::Right => [1344.0, 352.0, -128.0, 128.0]
  };

  Some((rect, source_rect))
}
