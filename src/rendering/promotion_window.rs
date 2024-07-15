use crate::rendering::piece_sprite::*;
use crate::utils::contains;
use crate::PieceType;
use macroquad::prelude::*;

struct Button {
  rect: Rect,
  sprite: PieceSprite
}
impl Button {
  fn handle_click(&self) -> Option<PieceType> {
    if is_mouse_button_pressed(MouseButton::Left) && contains(self.rect, mouse_position().into()) { 
      return Some(self.sprite.get_piecetype());
    }
    None
  }
}
struct Window {
  buttons: [PieceSprite; 4],
  selected_piece: Option<PieceSprite>
}
impl Window {
  fn get_promotion_piece(&mut self) {
    for mut button in self.buttons { // idk m8 wilil fix later
      button.handle_mousedown();
    }
  }
}