/* MODULES */
mod rendering;
mod utils;

/* IMPORTS */
use rendering::piece_sprite::*;
use rendering::square::*;
use utils::window_conf;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
  let texture_atlas = load_texture(TEXTURE_PATH).await.unwrap();

  let mut squares: [Square; 64] = [Square::default(); 64];
  let mut mouse_square = Square::default();

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
      squares[i].set_colour(LIGHTSQUARE);
    }

    x += 1;
    if x >= 8 {
      x = 0;
      y += 1;
    }
  }

  loop {
    clear_background(GRAY);

    for square in &squares {
      if square.handle_mouseover() {
        mouse_square = *square;
      }
      
      square.draw();
    }

    // piece_sprite logic
    for piece_sprite in piece_sprites.iter_mut() {
      piece_sprite.handle_mousedown();

      if piece_sprite.mouse_on_sprite {
        let (mouse_x, mouse_y) = mouse_position();
        piece_sprite.set_location_center(mouse_x, mouse_y);

        let mouse_square_index = squares.iter().position(|&r| r == mouse_square).unwrap() as i32;
        piece_sprite.square = mouse_square_index;
      }
      else {
        if piece_sprite.square == -1 {
          continue;
        }
        let piece_square = squares[piece_sprite.square as usize];
        piece_sprite.set_location(piece_square.rect.x, piece_square.rect.y);
      }
    }

    // piece_sprite rendering
    piece_sprites.sort_by(|a, b| a.mouse_on_sprite.cmp(&b.mouse_on_sprite)); // sorts the list so that the pieces that are affected by the mouse are last. this ensures that they are drawn on top of the other pieces
    for piece_sprite in &piece_sprites {
      piece_sprite.draw();
    }

    next_frame().await
  }
}