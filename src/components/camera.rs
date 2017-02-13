use geom::Rect;
use specs::{Entity, Component, VecStorage};

#[derive(Debug)]
pub struct Camera {
  pub target: Entity,
  pub screen: Rect,
  pub bounds: Rect
}

impl Component for Camera {
  type Storage = VecStorage<Camera>;
}

impl Camera {
  pub fn center(&mut self, x: f64, y: f64) {
    //Calc new rect
    let mut rect = Rect::from_center(x, y, self.screen.w, self.screen.h);

    if rect.left() < self.bounds.left() { rect.x += self.bounds.left() - rect.left(); }
    if rect.right() > self.bounds.right() { rect.x -= rect.right() - self.bounds.right(); }
    if rect.top() < self.bounds.top() { rect.y += self.bounds.top() - rect.top() }
    if rect.bottom() > self.bounds.bottom() { rect.y -= rect.bottom() - self.bounds.bottom() }
    self.screen = rect;
  }
}
