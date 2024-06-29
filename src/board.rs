#![allow(unused_parens)]

use std::collections::HashMap;
use crate::utils::PieceType;

struct Board {
  bitboards: [u64; 12],
  // add other flags when needed
}
impl Board {
  fn parse_fen(&mut self, fen: &str) {
    let mut parts = fen.split(' '); // do the rest of the flags later
    let position = parts.next().unwrap();

    let char_to_piecetype: HashMap<char, PieceType> = HashMap::from([
      ('K', PieceType::WhiteKing),
      ('Q', PieceType::WhiteQueen),
      ('B', PieceType::WhiteBishop),
      ('N', PieceType::WhiteKnight),
      ('R', PieceType::WhiteRook),
      ('P', PieceType::WhitePawn),
      ('k', PieceType::BlackKing),
      ('q', PieceType::BlackQueen),
      ('b', PieceType::BlackBishop),
      ('n', PieceType::BlackKnight),
      ('r', PieceType::BlackRook),
      ('p', PieceType::BlackPawn),
    ]);

    let mut x = 0;
    let mut y = 7;
    for c in position.chars() {
      match c {
        '/' => {
          y -= 1;
          x = 0;
        },
        '1'..='8' => {
          x += c.to_digit(10).unwrap() as usize;
          
          if x > 8 {
            panic!("Invalid FEN: rank {} does not have exactly 8 columns", 8 - x);
          }
        },
        'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
          let bitboard_type = char_to_piecetype[&c];
          let square_index = y * 8 + x;
          self.bitboards[bitboard_type as usize] |= (1 << square_index);
          x += 1;
        },
        _ => panic!("Unexpected character in FEN"),
      }
    }
  }
  fn export() { // return board as an array for the renderer to draw
    
  }
}