use systems::physics::GRAVITY;

pub const PREJUMP_FRAMES: usize = 15;
pub const PREJUMP_YVEL: f64 = -1.0;
pub const PREJUMP_ACCEL: f64 = -1.2;
pub const PREJUMP_ACCEL_DECAY: f64 = 0.80;
pub const FALLING_DECEL: f64 = 1.5 * GRAVITY.y;
pub const DOUBLEJUMP_YVEL: f64 = -5.0;
pub const ALLOW_DOUBLEJUMP_YVEL: f64 = -3.0;
