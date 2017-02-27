use specs::{Entity, Component, VecStorage};
use progress;

#[derive(Debug)]
pub struct Song {
  pub anchor: Entity,
  pub notes: Vec<Entity>,
  pub frame: usize,
  pub length: SongLength
}

#[derive(Debug, Copy, Clone)]
pub enum SongLength {
  OneNote = 0,
  TwoNotes,
  ThreeNotes,
  FourNotes
}

impl Song {
  pub fn new(e: Entity, progress: progress::Progress) -> Song {
    let length = if progress.contains(progress::DOUBLE_JUMP) {
      SongLength::FourNotes
    } else if progress.contains(progress::HANG) {
      SongLength::ThreeNotes
    } else if progress.contains(progress::DASH) {
      SongLength::TwoNotes
    } else {
      SongLength::OneNote
    };

    Song {
      anchor: e,
      notes: vec![],
      frame: 0,
      length: length
    }
  }
}

impl Component for Song {
  type Storage = VecStorage<Song>;
}
