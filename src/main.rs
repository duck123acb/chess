use std::collections::HashMap;

use macroquad::prelude::*;

const TEXTURE_PATH: &str = "assets/pieces.png";
const SPRITE_SIZE: i32 = 133;

fn window_conf() -> Conf {
  Conf {
    window_title: "chess".to_string(),
    window_width: 800,
    window_height: 600,
    window_resizable: false,
    ..Default::default()
  }
}
fn draw_from_atlas(texture: &Texture2D, texture_x: i32, texture_y: i32, texture_rect: Rect) { // macroquad doesn't have a built in function for drawing from an atlas
  let params = DrawTextureParams {
    dest_size: Some(vec2(texture_rect.w, texture_rect.h)),
    source: Some(Rect {x: texture_x as f32, y: texture_y as f32, w: texture_rect.w, h: texture_rect.h}),
    ..Default::default()
  };

  draw_texture_ex(texture, texture_rect.x, texture_rect.y, WHITE, params);
}
fn get_sprite_coords(key: char) -> (i32, i32) {
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
    return (x, y)
  }
  return (-1, -1) // if this is returned, something is wrong
}

struct PieceSprite {
  rect: Rect,
  texture: Texture2D,
  piece_type: char // change to the dedicated piece enum later
}
impl PieceSprite {
  fn new(sprite_x: i32, sprite_y: i32, sprite_texture: &Texture2D, sprite_type: char) -> Self {
    Self {
      rect: Rect::new(sprite_x as f32, sprite_y as f32, SPRITE_SIZE as f32, SPRITE_SIZE as f32), // 133 comes from my spritesheet
      texture: sprite_texture.clone(),
      piece_type: sprite_type
    }
  }

  fn update(&mut self) {

  }
  fn draw(&self) {
    let (x, y) = get_sprite_coords(self.piece_type);
    draw_from_atlas(&self.texture, x * SPRITE_SIZE, y * SPRITE_SIZE, self.rect);
  }
}
struct Square {
  rect: Rect,
  piece: PieceSprite
}

#[macroquad::main(window_conf)]
async fn main() {
  let texture_atlas = load_texture(TEXTURE_PATH).await.unwrap();
  texture_atlas.set_filter(FilterMode::Nearest);
  let piece = PieceSprite::new(0, 0, &texture_atlas, 'Q'); // MY CODE WORKS YESSSS!!!!!!!!
  let piece2 = PieceSprite::new(200, 200, &texture_atlas, 'r'); // MY CODE WORKS YESSSS!!!!!!!!

  loop {
    /* LOGIC */

    /* RENDERING */
    clear_background(GRAY);

    piece.draw();
    piece2.draw();
    
    next_frame().await
  }
}