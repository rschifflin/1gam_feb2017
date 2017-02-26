use graphics;

pub fn draw(current_frame: usize, flip: bool, x: f64, y: f64) -> graphics::Image {
  let image = graphics::image::Image::new().rect([x, y, 32.0, 32.0]);
  if flip {
    image.src_rect([512.0, 0.0, -256.0, 256.0])
  } else {
    image.src_rect([256.0, 0.0, 256.0, 256.0])
  }
}
