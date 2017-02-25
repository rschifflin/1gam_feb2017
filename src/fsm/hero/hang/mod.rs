use std::fmt::Debug;
pub use self::hang::HangFSM;
pub use self::hangless::HanglessFSM;

mod hang;
mod hangless;
mod constants;

pub trait Hang: Debug + Send + Sync {
  fn on_bonked(&mut self);
  fn get_yvel(&self) -> f64;
  fn update(&mut self, yvel: f64);
}
