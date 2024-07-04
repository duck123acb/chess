use macroquad::prelude::*;
use crate::utils::contains;
use crate::utils::PieceType;

pub const TEXTURE_PATH: &str = "assets/pieces.png";
const TEXTURE_SIZE: i32 = 133;

fn draw_from_atlas(atlas: &Texture2D, sprite_rect: Rect, texture_mask: Rect) {
  let params = DrawTextureParams {
    dest_size: Some(vec2(sprite_rect.w, sprite_rect.h)), // sets onscreen size
    source: Some(Rect {x: texture_mask.x as f32, y: texture_mask.y as f32, w: texture_mask.w, h: texture_mask.h}), // gets the sprite from the atlas
    ..Default::default() // sets the rest of the parameters to their default
  };

  draw_texture_ex(atlas, sprite_rect.x, sprite_rect.y, WHITE, params);
}


#[derive(Clone)]
pub struct PieceSprite {
  texture: Texture2D,
  piece_type: PieceType,
  square: i32,
  mouse_on_sprite: bool,
  
  pub rect: Rect,
  pub moved_piece: bool,
}
impl PieceSprite {
  pub fn new(sprite_size: f32, sprite_texture: &Texture2D, sprite_type: PieceType, sprite_square: i32) -> Self {
    Self {
      texture: sprite_texture.clone(),
      piece_type: sprite_type,
      square: sprite_square,
      mouse_on_sprite: false,
      
      rect: Rect::new(0.0, 0.0, sprite_size as f32, sprite_size as f32),
      moved_piece: false,
    }
  }
  fn get_sprite_coords(key: &PieceType) -> (i32, i32) { // retufns coordinates of sprite on the atlas
    let sprite_map = [
      (0, 0),
      (1, 0),
      (2, 0),
      (3, 0),
      (4, 0),
      (5, 0),
      (0, 1),
      (1, 1),
      (2, 1),
      (3, 1),
      (4, 1),
      (5, 1),
    ];
    return sprite_map[*key as usize];
  }

  pub fn handle_mousedown(&mut self) {
    if is_mouse_button_pressed(MouseButton::Left) && !self.mouse_on_sprite { 
      self.mouse_on_sprite = contains(self.rect, mouse_position().into());
    }
    if is_mouse_button_released(MouseButton::Left) && self.mouse_on_sprite {
      self.mouse_on_sprite = false;
    }
  }
  
  pub fn draw(&self) {
    let (x, y) = Self::get_sprite_coords(&self.piece_type);
    let texture_mask = Rect::new((x * TEXTURE_SIZE) as f32, (y * TEXTURE_SIZE) as f32 , TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
    draw_from_atlas(&self.texture, self.rect, texture_mask);
  }

  pub fn get_piecetype(&self) -> PieceType {
    self.piece_type
  }
  pub fn get_square(&self) -> i32 {
    self.square
  }
  pub fn get_if_mouseonsprite(&self) -> bool {
    self.mouse_on_sprite
  }
}