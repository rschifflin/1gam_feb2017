use input::{self, Input};

mod jump;
mod run;
mod hang;

pub use self::jump::{Jump, SingleJumpFSM, DoubleJumpFSM};
pub use self::run::{Run, RunFSM, DashFSM};
pub use self::hang::{Hang, HangFSM, HanglessFSM};
