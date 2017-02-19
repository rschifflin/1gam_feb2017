use systems::physics::GRAVITY;

pub const RUNSPEED: f64 = 3.0;
pub const DASHSPEED: f64 = 10.0;
pub const DASH_YVEL: f64 = -GRAVITY.y;
pub const INPUT_FRAMES: usize = 15;
pub const COOLDOWN_FRAMES: usize = 30;
pub const DASH_FRAMES: usize = 10;
