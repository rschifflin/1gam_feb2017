use specs::Entity;
use input::InputBuffer;

#[derive(Clone)]
pub struct Context {
  pub input: InputBuffer,
  pub director: Entity,
  //sound_tx: sound_tx
}
