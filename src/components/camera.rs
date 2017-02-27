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
  pub fn gameplay_area(&self) -> Rect {
    Rect::new(self.screen.x, self.screen.y, self.screen.w, self.screen.h * 0.85)
  }

  pub fn ui_area(&self) -> Rect {
    Rect::new(0.0, self.screen.h * 0.85, self.screen.w, self.screen.h * 0.15)
  }

  pub fn center(&mut self, x: f64, y: f64) {
    //Calc new rect
    let gameplay_area = self.gameplay_area();
    let mut rect = Rect::from_center(x, y, gameplay_area.w, gameplay_area.h);

    if rect.left() < self.bounds.left() { rect.x += self.bounds.left() - rect.left(); }
    if rect.right() > self.bounds.right() { rect.x -= rect.right() - self.bounds.right(); }
    if rect.top() < self.bounds.top() { rect.y += self.bounds.top() - rect.top() }
    if rect.bottom() > self.bounds.bottom() { rect.y -= rect.bottom() - self.bounds.bottom() }

    self.screen.x = rect.x;
    self.screen.y = rect.y;
  }
}
