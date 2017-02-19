use std::fmt::Debug;
use input::Input;
pub use self::run::RunFSM;
pub use self::dash::DashFSM;

mod run;
mod dash;
mod constants;

pub trait Run: Debug + Send + Sync {
  fn get_vel(&self) -> (f64, f64);
  fn is_dashing(&self) -> bool;
  fn update(&mut self, xvel: f64, yvel: f64, inputs: (Input, Input));
}
