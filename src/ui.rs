use graphics;
use progress;
use geom::Rect;

pub fn draw(progress: progress::Progress, ui_area: &Rect) -> Vec<graphics::Image> {
  let mut graphics = vec![];
  graphics.push(
    graphics::image::Image::new()
      .rect([ui_area.x, ui_area.y, 213.0, 72.0])
      .src_rect([0.0, 832.0, 576.0, 192.0])
  );

  if progress.contains(progress::DASH) {
    graphics.push(
      graphics::image::Image::new()
        .rect([ui_area.x + 213.0, ui_area.y, 213.0, 72.0])
        .src_rect([0.0, 608.0, 576.0, 192.0])
    );
  } else {
    graphics.push(
      graphics::image::Image::new()
        .rect([ui_area.x + 213.0, ui_area.y, 213.0, 72.0])
        .src_rect([0.0, 1056.0, 576.0, 192.0])
    );
  }

  if progress.contains(progress::HANG) {
    graphics.push(
      graphics::image::Image::new()
        .rect([ui_area.x + 426.0, ui_area.y, 213.0, 72.0])
        .src_rect([608.0, 640.0, 576.0, 192.0])
    );
  } else {
    graphics.push(
      graphics::image::Image::new()
        .rect([ui_area.x + 426.0, ui_area.y, 213.0, 72.0])
        .src_rect([608.0, 1088.0, 576.0, 192.0])
    );
  }

  if progress.contains(progress::DOUBLE_JUMP) {
    graphics.push(
      graphics::image::Image::new()
        .rect([ui_area.x + 639.0, ui_area.y, 213.0, 72.0])
        .src_rect([608.0, 864.0, 576.0, 192.0])
    );
  } else {
    graphics.push(
      graphics::image::Image::new()
        .rect([ui_area.x + 639.0, ui_area.y, 213.0, 72.0])
        .src_rect([0.0, 1280.0, 576.0, 192.0])
    );
  }

  graphics
}
