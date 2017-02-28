mod left_right;
mod up_down;
mod arc;
mod wander;

use std::fmt::Debug;
pub use self::left_right::LeftRightFSM;
pub use self::up_down::UpDownFSM;
pub use self::arc::ArcFSM;
pub use self::wander::WanderFSM;

pub trait EnemyAi: Debug + Send + Sync {
  fn on_collide(&mut self);
  fn update(&mut self, xvel: f64, yvel: f64);
  fn get_vel(&self) -> (f64, f64);
}
