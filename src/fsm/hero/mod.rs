use input::{self, Input};

mod jump;
mod run;

pub use self::jump::{Jump, SingleJumpFSM, DoubleJumpFSM};
pub use self::run::{Run, RunFSM, DashFSM};
