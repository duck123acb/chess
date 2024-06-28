use macroquad::prelude::*;
use crate::utils::contains;

pub const LIGHTSQUARE: Color = Color::new(0.95, 0.86, 0.71, 1.00);
pub const DARKSQUARE: Color = Color::new(0.71, 0.55, 0.4, 1.00);

#[derive(Copy, Clone, PartialEq)]
pub struct Square {
  pub rect: Rect,
  colour: Color,
}
impl Square {
  pub fn new(square_x: f32, square_y: f32, square_size: f32, square_colour: Color) -> Self {
    Self {
      rect: Rect::new(square_x, square_y, square_size, square_size),
      colour: square_colour,
    }
  }
  pub fn default() -> Self {
    Square::new(0.0, 0.0, screen_width() / 8.0, DARKSQUARE)
  }

  pub fn handle_mouseover(&self) -> bool {
    let mouse_pos = mouse_position().into();
    return contains(self.rect, mouse_pos);
  }
  pub fn draw(&self) {
    draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.colour);
  }

  pub fn set_location(&mut self, x: f32, y: f32) {
    self.rect.x = x;
    self.rect.y = y;
  }
  pub fn set_colour(&mut self, colour: Color) {
    self.colour = colour;
  }
}