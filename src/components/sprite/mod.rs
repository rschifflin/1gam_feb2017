mod hero;
mod bird;
mod note;
mod block;
mod checkpoint;
mod spikes;
mod spikeblock;
mod enemy;
mod portal;

use graphics::types::{Rectangle, SourceRectangle};
use specs::{Component, VecStorage};
use facing::Facing;
pub use self::note::NoteType;

#[derive(Debug, Copy, Clone)]
pub enum Graphic {
  Hero,
  Bird,
  Block,
  Spikeblock,
  Enemy,
  Portal,
  Note(note::NoteType),
  Checkpoint(bool),
  Spikes(Facing)
}

#[derive(Debug, Copy, Clone)]
pub enum Layer {
  Background = 0,
  Layer1,
  Layer2,
  Layer3,
  Layer4,
  Layer5,
  Layer6,
  Layer7,
  Layer8,
  Foreground
}

pub struct Sprite {
  frame: usize,
  flip: bool,
  graphic: Graphic,
  pub layer: Layer
}

impl Component for Sprite {
  type Storage = VecStorage<Sprite>;
}

impl Sprite {
  pub fn new(graphic: Graphic, layer: Layer) -> Sprite {
    Sprite {
      frame: 0,
      flip: false,
      graphic: graphic,
      layer: layer
    }
  }

  pub fn update(&mut self) {
    self.frame = self.frame.saturating_add(1);
  }

  pub fn flip(&mut self, flip: bool) {
    self.flip = flip;
  }

  pub fn as_image_rects(&self, x: f64, y: f64) -> Option<(Rectangle, SourceRectangle)> {
    match self.graphic {
      Graphic::Hero => hero::draw(self, x, y),
      Graphic::Bird => bird::draw(self, x, y),
      Graphic::Block => block::draw(self, x, y),
      Graphic::Portal => portal::draw(self, x, y),
      Graphic::Note(note_type) => note::draw(self, note_type, x, y),
      Graphic::Checkpoint(checked) => if checked { checkpoint::draw_checked(self, x, y) } else { checkpoint::draw_unchecked(self, x, y) },
      Graphic::Spikeblock => spikeblock::draw(self, x, y),
      Graphic::Spikes(facing) => spikes::draw(self, facing, x, y),
      Graphic::Enemy => enemy::draw(self, x, y)
    }
  }
}
