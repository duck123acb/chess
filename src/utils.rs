use macroquad::prelude::*;

pub fn window_conf() -> Conf {
  Conf {
    window_title: "chess".to_string(),
    window_width: 800,
    window_height: 800,
    window_resizable: false,
    ..Default::default()
  }
}

pub fn contains(rect: Rect, point: Vec2) -> bool {
  (point.x >= rect.x && point.x <= rect.x + rect.w) && (point.y >= rect.y && point.y <= rect.y + rect.h)
}