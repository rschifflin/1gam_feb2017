bitflags! {
  pub flags Progress: u32 {
    const WHISTLE      = 0b00000001,
    const DASH         = 0b00000010,
    const HANG         = 0b00000100,
    const DOUBLE_JUMP  = 0b00001000
  }
}
