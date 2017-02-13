#[derive(Debug)]
pub struct Rect {
  pub x: f64,
  pub y: f64,
  pub w: f64,
  pub h: f64
}

impl Rect {
  pub fn from_center(x: f64, y: f64, w: f64, h: f64) -> Rect {
    Rect {
      x: x - w/2.0,
      y: y - h/2.0,
      w: w,
      h: h
    }
  }

  pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rect {
    Rect {
      x: x,
      y: y,
      w: w,
      h: h
    }
  }

  pub fn left(&self) -> f64 {
    self.x
  }

  pub fn right(&self) -> f64 {
    self.x + self.w
  }

  pub fn top(&self) -> f64 {
    self.y
  }

  pub fn bottom(&self) -> f64 {
    self.y + self.h
  }
}
