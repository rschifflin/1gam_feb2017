use std::fmt::Debug;
use input::Input;
pub use self::single_jump::SingleJumpFSM;
pub use self::double_jump::DoubleJumpFSM;

mod single_jump;
mod double_jump;
mod constants;

pub trait Jump: Debug + Send + Sync {
  fn on_landed(&mut self);
  fn get_yvel(&self) -> f64;
  fn update(&mut self, yvel: f64, inputs: (Input, Input));
}
