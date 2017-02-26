mod hero;
mod bird;
mod note;

use graphics;
use specs::{Component, VecStorage};
pub use self::note::NoteType;

#[derive(Debug, Copy, Clone)]
pub enum Graphic {
  Hero,
  Bird,
  Note(note::NoteType),
}

pub struct Sprite {
  frame: usize,
  flip: bool,
  graphic: Graphic,
}

impl Component for Sprite {
  type Storage = VecStorage<Sprite>;
}

impl Sprite {
  pub fn new(graphic: Graphic) -> Sprite {
    Sprite {
      frame: 0,
      flip: false,
      graphic: graphic
    }
  }

  pub fn update(&mut self) {
    self.frame = self.frame.saturating_add(1);
  }

  pub fn flip(&mut self, flip: bool) {
    self.flip = flip;
  }

  pub fn as_image(&self, x: f64, y: f64) -> Option<graphics::Image> {
    match self.graphic {
      Graphic::Hero => hero::draw(self, x, y),
      Graphic::Bird => bird::draw(self, x, y),
      Graphic::Note(note_type) => note::draw(self, note_type, x, y)
    }
  }
}
