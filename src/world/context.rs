use specs::Entity;
use input::Input;

#[derive(Clone)]
pub struct Context {
  pub input: (Input, Input), //(Last, Next)
  pub director: Entity,
  //sound_tx: sound_tx
}
