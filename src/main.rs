/* MODULES */
mod rendering;
mod board;
mod utils;

/* IMPORTS */
use rendering::piece_sprite::*;
use rendering::square::*;
use board::Board;
use utils::window_conf;
use utils::PieceType;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
  let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

  let texture_atlas = load_texture(TEXTURE_PATH).await.unwrap();

  let mut squares: [Square; 64] = [Square::default(); 64];
  let mut mouse_square = Square::default();

  let mut piece_sprites: Vec<PieceSprite> = Vec::new();
  let bitboards = board.get_bitboards();
  for piece_type in PieceType::iter() {
    for i in 0..64 {
      let bitboard = bitboards[piece_type as usize];
      if bitboard & (1 << i) != 0 {
        let new_piece = PieceSprite::new(squares[0].rect.w, &texture_atlas, piece_type, i);
        piece_sprites.push(new_piece);
      }
    }
  }

  // square grid setup
  let mut x = 0;
  let mut y = 7;
  for i in 0..64 {
    squares[i].set_location(x as f32 * squares[i].rect.w, y as f32 * squares[i].rect.w);
    if (x + y) % 2 == 0 {
      squares[i].set_colour(LIGHTSQUARE);
    }

    x += 1;
    if x >= 8 {
      x = 0;
      y -= 1;
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

    piece_sprites.sort_by(|a, b| a.mouse_on_sprite.cmp(&b.mouse_on_sprite)); // sorts the list so that the pieces that are affected by the mouse are last. this ensures that they are drawn on top of the other pieces
    for piece_sprite in piece_sprites.iter_mut() {
      piece_sprite.handle_mousedown();

      if piece_sprite.mouse_on_sprite {
        let (mouse_x, mouse_y) = mouse_position();
        piece_sprite.set_location_center(mouse_x, mouse_y);

        let piece_bitboard = 1 << piece_sprite.square;
        let piece_moves = board.get_legal_moves(piece_bitboard, piece_sprite.piece_type);
        let mouse_square_index = squares.iter().position(|&r| r == mouse_square).unwrap() as i32;

        if piece_moves.contains(&mouse_square_index) {
          piece_sprite.square = mouse_square_index;
        }
      }
      else {
        if piece_sprite.square == -1 {
          continue;
        }
        let piece_square = squares[piece_sprite.square as usize];
        piece_sprite.set_location(piece_square.rect.x, piece_square.rect.y);
      }

      piece_sprite.draw();
    }

    next_frame().await
  }
}