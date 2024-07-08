/* MODULES */
mod rendering;
mod board_representation;
mod utils;

/* IMPORTS */
use rendering::piece_sprite::*;
use rendering::square::*;
use board_representation::*;
use utils::*;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
  let mut board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

  let texture_atlas = load_texture(TEXTURE_PATH).await.unwrap();

  let mut squares: [Square; 64] = [Square::default(); 64];
  let mut mouse_square = Square::default();
  let mut captured_piece: Option<PieceSprite> = None;


  let mut piece_sprites: Vec<PieceSprite> = Vec::new();
  for piece_type in PieceType::iter() {
    for i in 0..64 {
      if board.get_bitboards()[piece_type as usize] & (1 << i) != 0 {
        let new_piece = PieceSprite::new(squares[0].rect.w, &texture_atlas, piece_type, i);
        piece_sprites.push(new_piece);
      }
    }
  }

  // square grid setup
  let mut x = 7;
  let mut y = 7;
  for i in 0..64 {
    squares[i].set_location(x as f32 * squares[i].rect.w, y as f32 * squares[i].rect.w);
    if (x + y) % 2 == 0 {
      squares[i].set_colour(LIGHTSQUARE);
    }

    x -= 1;
    if x < 0 {
      x = 7;
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

    let mut piecesprites_to_remove: Vec<usize> = Vec::new();

    piece_sprites.sort_by(|a, b| a.get_if_mouseonsprite().cmp(&b.get_if_mouseonsprite())); // sorts the list so that the pieces that are affected by the mouse are last. this ensures that they are drawn on top of the other pieces
    for (i, piece_sprite) in piece_sprites.iter_mut(  ).enumerate() {
      piece_sprite.handle_mousedown();

      if contains(piece_sprite.rect, mouse_position().into()) && !piece_sprite.get_if_mouseonsprite()  && !is_mouse_button_released(MouseButton::Left) { // i dont know why but it has to NOT be when the mouse button is released that frame
        captured_piece = Some(piece_sprite.clone());
      }

      if piece_sprite.get_if_mouseonsprite() { // move the sprite based on where the mouse is
        piece_sprite.moved_piece = true;
        let (mouse_x, mouse_y) = mouse_position();
        // piece_sprite.set_location_center(mouse_x - (self.rect.w / 2.0), mouse_y - (self.rect.w / 2.0));
        piece_sprite.rect.x = mouse_x - (piece_sprite.rect.w / 2.0);
        piece_sprite.rect.y = mouse_y - (piece_sprite.rect.w / 2.0);
      }

      else if piece_sprite.moved_piece && is_mouse_button_released(MouseButton::Left) { // make a mov
        let piece_moves = board.get_legal_moves(piece_sprite.get_square(), piece_sprite.get_piecetype());
        let mouse_square_index = squares.iter().position(|&r| r == mouse_square).unwrap() as i32;
        let mut piece_move = Move::new(piece_sprite.get_square(), mouse_square_index, piece_sprite.get_piecetype(), None);

        if let Some(piece) = &captured_piece {
          piece_move.captured_piece_type = Some(piece.get_piecetype());

          if piece.get_piecetype() as usize == piece_sprite.get_piecetype() as usize { // if the captured piece is the same as the piece making the move
            piece_move.captured_piece_type = None; // it's not actually the captured piece
          }

          if piece.get_square() != piece_move.get_end_square() {
            piece_move.captured_piece_type = None;
          }
        }

        if piece_moves.contains(&piece_move) {
          board.make_move(piece_move);
        }

        piece_sprite.moved_piece = false;
      }
  
      else if piece_sprite.get_square() != -1 { // set the sprites locations to the square
        let piecetype_squares = bits_to_indices(&board.get_bitboards()[piece_sprite.get_piecetype() as usize]);
        if !piecetype_squares.contains(&piece_sprite.get_square()) { // if the piece doesnt exist there, add it to a vector to be removed
          piecesprites_to_remove.push(i);
        }

        let piece_square = squares[piece_sprite.get_square() as usize];
        piece_sprite.rect.x = piece_square.rect.x;
        piece_sprite.rect.y = piece_square.rect.y;
      }

      
      if piecesprites_to_remove.contains(&i) { // if it's gonna be removed, dont draw it
        continue;
      }
      piece_sprite.draw();
    }

    for &index in piecesprites_to_remove.iter().rev() { // remove the piece_sprites that are needed to remove
      piece_sprites.remove(index);
    }

    for piece_type in PieceType::iter() { // add the piece if it doesnt exist
      let piecetype_squares = bits_to_indices(&board.get_bitboards()[piece_type as usize]);
  
      for square_index in piecetype_squares {
        if !piece_sprites.iter().any(|sprite| sprite.get_square() == square_index && sprite.get_piecetype() as usize == piece_type as usize) { // if the piece doesnt exist
          let new_piece_sprite: PieceSprite = PieceSprite::new(squares[0].rect.w, &texture_atlas, piece_type, square_index);
          piece_sprites.push(new_piece_sprite);
        }
      }
    }

    next_frame().await
  }
}