use super::Hang;

#[derive(Debug, PartialEq)]
pub struct HanglessFSM {
  yvel: f64
}

impl HanglessFSM {
  pub fn new() -> HanglessFSM {
    HanglessFSM {
      yvel: 0.0
    }
  }
}

impl Hang for HanglessFSM {
  fn get_yvel(&self) -> f64 { self.yvel }
  fn on_bonked(&mut self) { }
  fn update(&mut self, yvel: f64) {
    self.yvel = yvel;
  }
}
