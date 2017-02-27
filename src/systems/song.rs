use components::{self, Position, Sprite, Graphic, NoteType, SongLength, Layer};
use specs::{System, RunArg, Join};
use systems::NamedSystem;
use world::Context;
use events;

const NOTE_KEYFRAME: usize = 40;

pub struct Song;

impl System<Context> for Song {
  fn run(&mut self, arg: RunArg, _: Context) {
    let (entities, mut pos, mut songs, mut sprites) = arg.fetch(|w| {
      let pos = w.write::<Position>();
      let songs = w.write::<components::Song>();
      let sprites = w.write::<components::Sprite>();
      let entities = w.entities();
      (entities, pos, songs, sprites)
    });

    for (ref entity, ref mut song) in (&entities, &mut songs).iter() {
      let ttl = match song.length {
        SongLength::OneNote  => NOTE_KEYFRAME,
        SongLength::TwoNotes => 2*NOTE_KEYFRAME,
        SongLength::ThreeNotes => 3*NOTE_KEYFRAME,
        SongLength::FourNotes => 4*NOTE_KEYFRAME
      };

      if song.frame >= ttl {
          for note in song.notes.iter() { arg.delete(*note); }
          arg.delete(*entity);
      } else {
        let anchor_pos = pos.get(song.anchor).cloned().unwrap_or_else(|| Position::default());
        let note_pos = Position { x: anchor_pos.x, y: anchor_pos.y - 32.0 };
        for note in song.notes.iter_mut() {
          pos.get_mut(*note).map(|mut old_note_pos| *old_note_pos = note_pos);
        }

        if song.frame % NOTE_KEYFRAME == 0 {
          let next_note = arg.create();
          pos.insert(next_note, note_pos);
          let note_type = match song.frame / NOTE_KEYFRAME {
            3 => NoteType::Fourth,
            2 => NoteType::Third,
            1 => NoteType::Second,
            _ => NoteType::First
          };
          sprites.insert(next_note, Sprite::new(Graphic::Note(note_type), Layer::Layer6));
          song.notes.push(next_note);
        }
      }
      song.frame += 1;
    }
  }
}

impl NamedSystem<Context> for Song {
  fn name(&self) -> &'static str {
    "song"
  }
}
