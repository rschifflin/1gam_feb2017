bitflags! {
  pub flags Input: u32 {
    const UP       = 0b00000001,
    const DOWN     = 0b00000010,
    const LEFT     = 0b00000100,
    const RIGHT    = 0b00001000,
    const JUMP     = 0b00010000,
    const WHISTLE  = 0b00100000,
  }
}
