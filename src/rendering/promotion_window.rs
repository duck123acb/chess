use crate::rendering::piece_sprite::*;
use crate::utils::contains;
use crate::PieceType;
use macroquad::prelude::*;

struct Button {
  rect: Rect,
  sprite: PieceSprite
}
impl Button {
  fn new(button_rect: Rect, piece_type: PieceType, texture: &Texture2D) -> Self {
    Self {
      rect: button_rect,
      sprite: PieceSprite::new(button_rect.w, texture, piece_type, -1)
    }
  }
  fn handle_click(&self) -> Option<&PieceSprite> {
    if is_mouse_button_pressed(MouseButton::Left) && contains(self.rect, mouse_position().into()) { 
      return Some(&self.sprite);
    }
    None
  }
}
struct Window {
  buttons: [Button; 4],
  selected_piece: Option<PieceSprite>
}
impl Window {
  fn new(x: f32, y: f32, size: f32, white_to_move: bool, texture: &Texture2D) -> Self {
    Self {
      buttons: if white_to_move {
        [
          Button::new(Rect::new(x, y, size, size), PieceType::WhiteQueen, texture),
          Button::new(Rect::new(x, y, size, size), PieceType::WhiteKnight, texture),
          Button::new(Rect::new(x, y, size, size), PieceType::WhiteBishop, texture),
          Button::new(Rect::new(x, y, size, size), PieceType::WhiteRook, texture),
        ]
      } else {
        [
          Button::new(Rect::new(x, y, size, size), PieceType::BlackQueen, texture),
          Button::new(Rect::new(x, y, size, size), PieceType::BlackKnight, texture),
          Button::new(Rect::new(x, y, size, size), PieceType::BlackBishop, texture),
          Button::new(Rect::new(x, y, size, size), PieceType::BlackRook, texture),
        ]
      },
      selected_piece: None
    }
  }
  fn get_promotion_piece(&mut self) {
    while self.selected_piece.is_none() {
      for button in &self.buttons {
        self.selected_piece = button.handle_click().cloned();
      }
    }
  }
}