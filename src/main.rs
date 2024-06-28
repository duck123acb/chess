use std::collections::HashMap;
use macroquad::prelude::*;

const TEXTURE_PATH: &str = "assets/pieces.png";
const TEXTURE_SIZE: i32 = 133;
const LIGHTBROWN: Color = Color::new(0.95, 0.86, 0.71, 1.00);
const DARKBROWN: Color = Color::new(0.71, 0.55, 0.4, 1.00);

fn window_conf() -> Conf {
  Conf {
    window_title: "chess".to_string(),
    window_width: 800,
    window_height: 800,
    window_resizable: false,
    ..Default::default()
  }
}
fn draw_from_atlas(atlas: &Texture2D, sprite_rect: Rect, texture_mask: Rect) {
  let params = DrawTextureParams {
    dest_size: Some(vec2(sprite_rect.w, sprite_rect.h)), // sets onscreen size
    source: Some(Rect {x: texture_mask.x as f32, y: texture_mask.y as f32, w: texture_mask.w, h: texture_mask.h}), // gets the sprite from the atlas
    ..Default::default() // sets the rest of the parameters to their default
  };

  draw_texture_ex(atlas, sprite_rect.x, sprite_rect.y, WHITE, params);
}

struct PieceSprite {
  rect: Rect,
  texture: Texture2D,
  piece_type: char, // change to the dedicated piece enum later // maybe not
  square: i32
}
impl PieceSprite {
  fn new(sprite_x: f32, sprite_y: f32, sprite_size: f32, sprite_texture: &Texture2D, sprite_type: char, sprite_square: i32) -> Self {
    Self {
      rect: Rect::new(sprite_x, sprite_y, sprite_size as f32, sprite_size as f32),
      texture: sprite_texture.clone(),
      piece_type: sprite_type,
      square: sprite_square
    }
  }
  fn get_sprite_coords(key: char) -> (i32, i32) { // returns coordinates of sprite on the atlas
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
      return (x, y) // returned this way because rust-analyzer doesnt like the way shown below
    }
    (-1, -1) // if this is returned, something is wrong
  }

  fn draw(&self) {
    let (x, y) = Self::get_sprite_coords(self.piece_type);
    let texture_mask = Rect::new((x * TEXTURE_SIZE) as f32, (y * TEXTURE_SIZE) as f32 , TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
    draw_from_atlas(&self.texture, self.rect, texture_mask);
  }

  fn set_location(&mut self, x: f32, y: f32) {
    self.rect.x = x;
    self.rect.y = y;
  }
}

#[derive(Copy, Clone)]
struct Square {
  rect: Rect,
  colour: Color,
}
impl Square {
  fn new(square_x: f32, square_y: f32, square_size: f32, square_colour: Color) -> Self {
    Self {
      rect: Rect::new(square_x, square_y, square_size, square_size),
      colour: square_colour,
    }
  }

  fn draw(&self) {
    draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.colour);
  }

  fn set_location(&mut self, x: f32, y: f32) {
    self.rect.x = x;
    self.rect.y = y;
  }
  fn set_colour(&mut self, colour: Color) {
    self.colour = colour;
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let texture_atlas = load_texture(TEXTURE_PATH).await.unwrap();

  let base_square = Square::new(0.0, 0.0, screen_width() / 8.0, DARKBROWN);
  let mut squares: [Square; 64] = [base_square; 64];

  let mut piece_sprites: Vec<PieceSprite> = Vec::new(); // make a way to do this from a list of chars/squares
  let piece_sprite = PieceSprite::new(0.0, 0.0, squares[0].rect.w, &texture_atlas, 'B', 9);
  let piece_sprite1 = PieceSprite::new(0.0, 0.0, squares[0].rect.w, &texture_atlas, 'r', 0);
  let piece_sprite2 = PieceSprite::new(0.0, 0.0, squares[0].rect.w, &texture_atlas, 'r', 2);
  let piece_sprite3 = PieceSprite::new(0.0, 0.0, squares[0].rect.w, &texture_atlas, 'r', 16);
  let piece_sprite4 = PieceSprite::new(0.0, 0.0, squares[0].rect.w, &texture_atlas, 'r', 18);
  piece_sprites.push(piece_sprite);
  piece_sprites.push(piece_sprite1);
  piece_sprites.push(piece_sprite2);
  piece_sprites.push(piece_sprite3);
  piece_sprites.push(piece_sprite4);

  // square grid setup
  let mut x = 0;
  let mut y = 0;
  for i in 0..64 {
    squares[i].set_location(x as f32 * squares[i].rect.w, y as f32 * squares[i].rect.w);
    if (x + y) % 2 == 0 {
      squares[i].set_colour(LIGHTBROWN);
    }

    x += 1;
    if x >= 8 {
      x = 0;
      y += 1;
    }
  }

  loop {
    /* LOGIC */
    for piece_sprite in piece_sprites.iter_mut() {
      if piece_sprite.square == -1 {
        continue;
      }

      let piece_square = squares[piece_sprite.square as usize];
      piece_sprite.set_location(piece_square.rect.x, piece_square.rect.y);
    }
  
    /* RENDERING */
    clear_background(GRAY);

    for square in squares {
      square.draw();
    }
    for piece_sprite in &piece_sprites {
      piece_sprite.draw();
    }

    next_frame().await
  }
}