use std::collections::HashMap;
use macroquad::prelude::*;

pub const TEXTURE_PATH: &str = "assets/pieces.png";
pub const TEXTURE_SIZE: i32 = 133;
pub const LIGHTSQUARE: Color = Color::new(0.95, 0.86, 0.71, 1.00);
pub const DARKSQUARE: Color = Color::new(0.71, 0.55, 0.4, 1.00);

fn draw_from_atlas(atlas: &Texture2D, sprite_rect: Rect, texture_mask: Rect) {
  let params = DrawTextureParams {
    dest_size: Some(vec2(sprite_rect.w, sprite_rect.h)), // sets onscreen size
    source: Some(Rect {x: texture_mask.x as f32, y: texture_mask.y as f32, w: texture_mask.w, h: texture_mask.h}), // gets the sprite from the atlas
    ..Default::default() // sets the rest of the parameters to their default
  };

  draw_texture_ex(atlas, sprite_rect.x, sprite_rect.y, WHITE, params);
}

pub struct PieceSprite {
  rect: Rect,
  texture: Texture2D,
  piece_type: char, // change to the dedicated piece enum later // maybe not
  
  pub square: i32
}
impl PieceSprite {
  pub fn new(sprite_x: f32, sprite_y: f32, sprite_size: f32, sprite_texture: &Texture2D, sprite_type: char, sprite_square: i32) -> Self {
    Self {
      rect: Rect::new(sprite_x, sprite_y, sprite_size as f32, sprite_size as f32),
      texture: sprite_texture.clone(),
      piece_type: sprite_type,
      square: sprite_square
    }
  }
  fn get_sprite_coords(key: char) -> (i32, i32) { // retufns coordinates of sprite on the atlas
    let sprite_map: HashMap<char, (i32, i32)> = HashMap::from([
      ('K', (0, 0)),
      ('Q', (1, 0)),
      ('B', (2, 0)),
      ('N', (3, 0)),
      ('R', (4, 0)),
      ('P', (5, 0)),
      ('k', (0, 1)),
      ('q', (1, 1)),
      ('b', (2, 1)),
      ('n', (3, 1)),
      ('r', (4, 1)),
      ('p', (5, 1)),
    ]);
    if let Some(&(x, y)) = sprite_map.get(&key) {
      return (x, y) // retufned this way because rust-analyzer doesnt like the way shown below
    }
    (-1, -1) // if this is retufned, something is wrong
  }

  pub fn draw(&self) {
    let (x, y) = Self::get_sprite_coords(self.piece_type);
    let texture_mask = Rect::new((x * TEXTURE_SIZE) as f32, (y * TEXTURE_SIZE) as f32 , TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
    draw_from_atlas(&self.texture, self.rect, texture_mask);
  }

  pub fn set_location(&mut self, x: f32, y: f32) {
    self.rect.x = x;
    self.rect.y = y;
  }
}

#[derive(Copy, Clone)]
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