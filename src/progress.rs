bitflags! {
  pub flags Progress: u32 {
    const FLAG_WHISTLE      = 0b00000001,
    const FLAG_DOUBLE_JUMP  = 0b00000010,
    const FLAG_HANG         = 0b00000100,
    const FLAG_FLOAT        = 0b00001000,
    const FLAG_DASH         = 0b00010000
  }
}
