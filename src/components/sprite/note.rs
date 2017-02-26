use graphics;
use super::Sprite;
use std::f64::consts::PI;

const TTL: usize = 60;

#[derive(Debug, Clone, Copy)]
pub enum NoteType {
  First,
  Second,
  Third,
  Fourth
}

pub fn draw(sprite: &Sprite, note_type: NoteType, x: f64, y: f64) -> Option<graphics::Image> {
  if sprite.frame >= TTL { return None }

  //X pos scrolls from right to left over 1s
  let lifetime_ratio = 1.0 - sprite.frame as f64 / TTL as f64;
  let draw_x = (x - 32.0) + 32.0 * lifetime_ratio;
  let draw_y = y + 16.0 * (lifetime_ratio*2.0*PI).sin();
  let image = graphics::image::Image::new().rect([draw_x, draw_y, 32.0, 32.0]);

  let src_rect = match note_type {
    NoteType::First => [576.0, 0.0, 128.0, 128.0],
    NoteType::Second => [576.0, 160.0, 128.0, 128.0],
    NoteType::Third => [736.0, 0.0, 128.0, 128.0],
    NoteType::Fourth => [736.0, 160.0, 128.0, 128.0]
  };

  if sprite.flip {
    Some(image.src_rect([src_rect[0] + src_rect[2], src_rect[1], -src_rect[2], src_rect[3]]))
  } else {
    Some(image.src_rect(src_rect))
  }
}
