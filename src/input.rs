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

#[derive(Clone)]
pub struct InputBuffer {
  raw_input: Input,
  buf_input: Input,
  last_buf_input: Input
}

impl InputBuffer {
  pub fn new() -> InputBuffer {
    InputBuffer {
      raw_input: Input::empty(),
      buf_input: Input::empty(),
      last_buf_input: Input::empty()
    }
  }

  pub fn current(&self) -> (Input, Input) {
    (self.last_buf_input, self.buf_input)
  }

  pub fn on(&mut self, flag: Input) {
    self.raw_input |= flag;
    self.buf_input |= flag;
  }

  pub fn off(&mut self, flag: Input) {
    self.raw_input -= flag;
  }

  pub fn update(&mut self) {
    self.last_buf_input = self.buf_input;
    self.buf_input = self.raw_input;
  }
}
