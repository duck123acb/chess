#![allow(unused_parens)]
#![allow(dead_code)]

use std::collections::HashMap;
use crate::utils::PieceType;

// from white's perspective
const TOP_RANK: u64 = 0xFF00000000000000;
const BOTTOM_RANK: u64 = 0x00000000000000FF;
const LEFT_FILE: u64 = 0x0101010101010101;
const RIGHT_FILE: u64 = 0x8080808080808080;

const RANK_SHIFT: i32 = 8; // value to shift if you want to move ranks
const FILE_SHIFT: i32 = 1; // value to shift if you want to move files

fn pawn_moves(bitboard: u64, friendly_bitboard: u64, enemy_bitboard: u64, is_white: bool)  -> u64 { // TODO: promotion
  let all_pieces = friendly_bitboard | enemy_bitboard;
  let mut moves: u64 = 0;
  let mut attacks: u64 = 0;

  if is_white {
    moves |= bitboard << RANK_SHIFT;
    if bitboard & (BOTTOM_RANK << RANK_SHIFT) != 0 { // if pawn is on 2nd rank
      moves |= bitboard << (RANK_SHIFT * 2);
    }

    attacks |= bitboard << (RANK_SHIFT - 1) | bitboard << (RANK_SHIFT + 1);
  } else {
    moves |= bitboard >> RANK_SHIFT;
    if bitboard & (TOP_RANK >> RANK_SHIFT) != 0 { // if pawn is on 7th rank
      moves |= bitboard >> (RANK_SHIFT * 2);
    }

    attacks |= bitboard >> (RANK_SHIFT - 1) | bitboard >> (RANK_SHIFT + 1);
  }

  moves ^= all_pieces & moves; // removes squares where another piece is. doesnt affect the pawn attacks
  attacks ^= attacks & friendly_bitboard; // removes attacks on friendly pieces
  if attacks & all_pieces == 0 { // if the pawn attacks nothing
    attacks = 0; // attacks mean nothing
  }

  moves |= attacks;
  moves
}

fn bits_to_indices(bitboard: &u64) -> Vec<i32> {
  let mut indices = Vec::new();
  for i in 0..64 {
    if *bitboard & (1 << i) != 0 {
      indices.push(i);
    }
  }
  indices
}

pub struct Board {
  bitboards: [u64; 12], // TODO: highlighted squares
  // add other flags when needed
}
impl Board {
  pub fn new(fen: &str) -> Self {
    let mut new_board = Self {
      bitboards: [0; 12]
    };
    new_board.parse_fen(fen);
    new_board
  }

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
        'P' | 'N' | 'B' | 'R' | 'K' | 'Q' | 'p' | 'n' | 'b' | 'r' | 'k' | 'q' => {
          let bitboard_type = char_to_piecetype[&c];
          let square_index = y * 8 + x; // oh my god this line of code took me like 30 minutes to figure out holy what the muffin | this isnt a really useful comment but it's kinda funny in my opinion
          self.bitboards[bitboard_type as usize] |= (1 << square_index);
          x += 1;
        },
        _ => panic!("Unexpected character in FEN"),
      }
    }
  }

  fn all_white_pieces(&self) -> u64 {
    self.bitboards[PieceType::WhiteKing as usize] | self.bitboards[PieceType::WhiteQueen as usize] | self.bitboards[PieceType::WhiteBishop as usize] | self.bitboards[PieceType::WhiteKnight as usize] | self.bitboards[PieceType::WhiteRook as usize] | self.bitboards[PieceType::WhitePawn as usize]
  }
  fn all_black_pieces(&self) -> u64 {
    self.bitboards[PieceType::BlackKing as usize] | self.bitboards[PieceType::BlackQueen as usize] | self.bitboards[PieceType::BlackBishop as usize] | self.bitboards[PieceType::BlackKnight as usize] | self.bitboards[PieceType::BlackRook as usize] | self.bitboards[PieceType::BlackPawn as usize]
  }

  pub fn get_legal_moves(&self, bitboard: u64, piece_type: PieceType) -> Vec<i32> {
    match piece_type {
      PieceType::WhitePawn => {
        let moves = pawn_moves(bitboard, self.all_white_pieces(), self.all_black_pieces(), true);
        return bits_to_indices(&moves);
      },
      _ => {
        panic!("Piece type not found");
      }
    }
  }

  pub fn get_bitboards(&self) -> [u64; 12] {
    return  self.bitboards;
  }

  // DEBUGING
  pub fn print(&self, index: PieceType) { //TODO: remove this later
    println!("{:b}", self.bitboards[index as usize]);
  }
}