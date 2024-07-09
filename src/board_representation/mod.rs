#![allow(unused_parens)]

mod precompiled_bitboards;

use std::collections::HashMap;
use crate::utils::PieceType;
use precompiled_bitboards::*;

// from white's perspective
const TOP_RANK: u64 = 0xFF00000000000000;
const BOTTOM_RANK: u64 = 0x00000000000000FF;
const LEFT_FILE: u64 = 0x8080808080808080;
const RIGHT_FILE: u64 = 0x0101010101010101;

const RANK_SHIFT: i32 = 8; // value to shift if you want to move ranks
const FILE_SHIFT: i32 = 1; // value to shift if you want to move files


fn pawn_moves(bitboard: &u64, friendly_bitboard: &u64, enemy_bitboard: &u64, is_white: bool)  -> u64 { // TODO: promotion, en_passent
  let all_pieces = friendly_bitboard | enemy_bitboard;
  let mut moves: u64 = 0;
  let mut attacks: u64 = 0;

  if is_white {
    moves |= bitboard << RANK_SHIFT;
    if bitboard & (BOTTOM_RANK << RANK_SHIFT) != 0 { // if pawn is on 2nd rank
      moves |= bitboard << (RANK_SHIFT * 2);
    }

    if bitboard & RIGHT_FILE == 0 { // if piece is not on the left file
      attacks |= bitboard << (RANK_SHIFT - 1)
    }
    if bitboard & LEFT_FILE == 0 { // if piece is not on the right file
      attacks |= bitboard << (RANK_SHIFT + 1);
    }
  } else {
    moves |= bitboard >> RANK_SHIFT;
    if bitboard & (TOP_RANK >> RANK_SHIFT) != 0 { // if pawn is on 7th rank
      moves |= bitboard >> (RANK_SHIFT * 2);
    }

    if bitboard & RIGHT_FILE == 0 { // if piece is not on the left file
      attacks |= bitboard >> (RANK_SHIFT + 1)
    }
    if bitboard & LEFT_FILE == 0 { // if piece is not on the right file
      attacks |= bitboard >> (RANK_SHIFT - 1);
    }
  }

  moves ^= all_pieces & moves; // removes squares where another piece is. doesnt affect the pawn attacks
  attacks ^= attacks & friendly_bitboard; // removes attacks on friendly pieces
  if attacks & all_pieces == 0 { // if the pawn attacks nothing
    attacks = 0; // attacks mean nothing
  }

  moves |= attacks;
  moves
}

fn knight_moves(bitboard: &u64, friendly_bitboard: &u64) -> u64 {
  let mut moves = 0;

  if (bitboard & TOP_RANK == 0) && (bitboard & (LEFT_FILE | (LEFT_FILE >> FILE_SHIFT)) == 0) { // if not on top rank AND if not on the two left-most files\
    moves |= bitboard << 10; // up left left
  }
  if (bitboard & (TOP_RANK & (TOP_RANK >> RANK_SHIFT)) == 0) && (bitboard & LEFT_FILE == 0) { // if not on the two top-most ranks AND if not on the left file
    moves |= bitboard << 17; // up up left
  }
  if (bitboard & (TOP_RANK & (TOP_RANK >> RANK_SHIFT)) == 0) && (bitboard & RIGHT_FILE == 0) { // if not on the two top-most ranks AND if not on the right file
    moves |= bitboard << 15; // up up right
  }
  if (bitboard & TOP_RANK == 0) && (bitboard & (RIGHT_FILE | (RIGHT_FILE << FILE_SHIFT)) == 0) { // if not on top rank AND if not on the two right-most files
    moves |= bitboard << 6; // up right right
  }
  if (bitboard & BOTTOM_RANK == 0) && (bitboard & (RIGHT_FILE | (RIGHT_FILE << FILE_SHIFT)) == 0) { // if not on bottom rank AND if not on the two right-most files
    moves |= bitboard >> 10; // down right right
  }
  if (bitboard & (BOTTOM_RANK & (BOTTOM_RANK << RANK_SHIFT)) == 0) && (bitboard & RIGHT_FILE == 0) { // if not on the two bottom-most ranks AND if not on the right file
    moves |= bitboard >> 17; // down down right
  }
  if (bitboard & (BOTTOM_RANK & (BOTTOM_RANK << RANK_SHIFT)) == 0) && (bitboard & LEFT_FILE == 0) { // if not on the two bottom-most ranks AND if not on the left file
    moves |= bitboard >> 15; // down down left
  }
  if (bitboard & BOTTOM_RANK == 0) && (bitboard & (LEFT_FILE | (LEFT_FILE >> FILE_SHIFT)) == 0) { // if not on bottom rank AND if not on the two left-most files
    moves |= bitboard >> 6; // down left left
  }
  
  let false_attacks = moves & friendly_bitboard;
  if false_attacks != 0 {
    moves ^= false_attacks;
  }

  moves
}

fn king_moves(bitboard: &u64, friendly_bitboard: &u64) -> u64 {
  let mut moves = 0;

  if bitboard & TOP_RANK == 0 { // if not on the top of the board
    moves |= bitboard << RANK_SHIFT; // up
    
    if bitboard & RIGHT_FILE == 0 { // if not on the right of the board
      moves |= bitboard << RANK_SHIFT - 1; // up right
    }
    if bitboard & LEFT_FILE == 0 { // if not on the left of the board
      moves |= bitboard << RANK_SHIFT + 1; // up left
    }
  }
  if bitboard & BOTTOM_RANK == 0 { // if not on the bottom of the board
    moves |= bitboard >> RANK_SHIFT; // down

    if bitboard & LEFT_FILE == 0 { // if not on the left of the board
      moves |= bitboard >> RANK_SHIFT - 1; // down left
    }
    if bitboard & RIGHT_FILE == 0 { // if not on the right of the board
      moves |= bitboard >> RANK_SHIFT + 1; // down right
    }
  }
  if bitboard & LEFT_FILE == 0 { // if not on the left of the board
    moves |= bitboard << FILE_SHIFT; // left
  }
  if bitboard & RIGHT_FILE == 0 { // if not on the right of the board
    moves |= bitboard >> FILE_SHIFT; // right
  }
  
  let false_attacks = moves & friendly_bitboard;
  if false_attacks != 0 {
    moves ^= false_attacks;
  }

  moves
}

fn get_magic_index(magic: u64, index_bits: u32, mask: u64, population: &u64) -> usize {
  let blockers = population & mask;
  let hash = blockers.wrapping_mul(magic);
  
  (hash >> index_bits) as usize
}

fn get_bishop_moves(square_index: i32, friendly_bitboard: &u64, enemy_bitboard: &u64) -> u64 {
  let population = friendly_bitboard & enemy_bitboard;
  
  let magic = &BISHOP_MAGICS[square_index as usize];
  let mask = &BISHOP_MASKS[square_index as usize];
  let relevant_bits = &BISHOP_BITS[square_index as usize];

  let mut moves = BISHOP_MOVES[square_index as usize][get_magic_index(*magic, *relevant_bits, *mask, &population)];
  moves ^= friendly_bitboard;
  
  moves
}
fn get_rook_moves(square_index: i32, friendly_bitboard: &u64, enemy_bitboard: &u64) -> u64 {
  let population = friendly_bitboard & enemy_bitboard;
  
  let magic = &ROOK_MAGICS[square_index as usize];
  let mask = &ROOK_MASKS[square_index as usize];
  let relevant_bits = &ROOK_BITS[square_index as usize];

  let mut moves = ROOK_MOVES[square_index as usize][get_magic_index(*magic, *relevant_bits, *mask, &population)];
  moves ^= friendly_bitboard;
  
  moves
}

pub fn bits_to_indices(bitboard: &u64) -> Vec<i32> {
  let mut indices = Vec::new();
  for i in 0..64 {
    if *bitboard & (1 << i) != 0 {
      indices.push(i);
    }
  }
  indices
}

pub struct Move {
  start_square: i32,
  end_square: i32,
  moved_piece_type: PieceType,

  pub captured_piece_type: Option<PieceType>,
  // add other flags later
}
impl Move {
  pub fn new(move_start_square: i32, move_end_square: i32, piece: PieceType, piece_captured: Option<PieceType>) -> Self {
    Self {
      start_square: move_start_square,
      end_square: move_end_square,
      moved_piece_type: piece,
      captured_piece_type: piece_captured
    }
  }

  pub fn get_end_square(&self) -> i32 {
    self.end_square
  }
}
impl PartialEq for Move {
  fn eq(&self, other: &Self) -> bool {
    self.start_square == other.start_square && self.end_square == other.end_square
  }
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
          let square_index = y * 8 + (7 - x); // oh my god this line of code took me like 30 minutes to figure out holy what the muffin | this isnt a really useful comment but it's kinda funny in my opinion
          self.bitboards[bitboard_type as usize] |= (1 << square_index);
          x += 1;
        },
        _ => panic!("Unexpected character in FEN"),
      }
    }
  }

  pub fn generate_moves(&self, piece_bitboard: &u64, moves_bitboard: &u64, piece_type: PieceType) -> Vec<Move>{
    let mut moves: Vec<Move> = Vec::new();
    let mut piece_square = 0;
    for i in 0..64 {
      if piece_bitboard & 1 << i != 0 {
        piece_square = i as i32;
        break;
      }
    }

    for square in bits_to_indices(moves_bitboard) {
      let mut new_move = Move::new(piece_square, square, piece_type, None);

      for piece_type in PieceType::iter() {
        if self.bitboards[piece_type as usize] & 1 << square != 0 { // if the square already has something on it
          new_move.captured_piece_type = Some(piece_type);
        }
      }

      moves.push(new_move);
    }

    moves
  }

  pub fn make_move(&mut self, move_to_make: Move) { // move is a keyword in rust for some reason
    let new_piece_bitboard = 1 << move_to_make.end_square;
    let curr_piece_bitboard = 1 << move_to_make.start_square;

    self.bitboards[move_to_make.moved_piece_type as usize] ^= curr_piece_bitboard | new_piece_bitboard;


    if let Some(piece_type) = move_to_make.captured_piece_type {
      self.bitboards[piece_type as usize] ^= new_piece_bitboard;
    }
  }

  fn all_white_pieces(&self) -> u64 {
    self.bitboards[PieceType::WhiteKing as usize] | self.bitboards[PieceType::WhiteQueen as usize] | self.bitboards[PieceType::WhiteBishop as usize] | self.bitboards[PieceType::WhiteKnight as usize] | self.bitboards[PieceType::WhiteRook as usize] | self.bitboards[PieceType::WhitePawn as usize]
  }
  fn all_black_pieces(&self) -> u64 {
    self.bitboards[PieceType::BlackKing as usize] | self.bitboards[PieceType::BlackQueen as usize] | self.bitboards[PieceType::BlackBishop as usize] | self.bitboards[PieceType::BlackKnight as usize] | self.bitboards[PieceType::BlackRook as usize] | self.bitboards[PieceType::BlackPawn as usize]
  }

  pub fn get_legal_moves(&self, square_index: i32, piece_type: PieceType) -> Vec<Move> {
    let moves;
    let bitboard = 1 << square_index;

    match piece_type {
      PieceType::WhiteKing => {
        moves = king_moves(&bitboard, &self.all_white_pieces());
      },
      PieceType::BlackKing => {
        moves = king_moves(&bitboard, &self.all_black_pieces());
      },
      PieceType::WhiteBishop => {
        moves = get_bishop_moves(square_index, &self.all_white_pieces(), &self.all_black_pieces());
      },
      PieceType::BlackBishop => {
        moves = get_bishop_moves(square_index, &self.all_black_pieces(), &self.all_white_pieces());
      },
      PieceType::WhiteKnight => {
        moves = knight_moves(&bitboard, &self.all_white_pieces());
      },
      PieceType::BlackKnight => {
        moves = knight_moves(&bitboard, &self.all_black_pieces());
      },
      PieceType::WhiteRook => {
        moves = get_rook_moves(square_index, &self.all_white_pieces(), &self.all_black_pieces());
      },
      PieceType::BlackRook => {
        moves = get_rook_moves(square_index, &self.all_black_pieces(), &self.all_white_pieces());
      },
      PieceType::WhitePawn => {
        moves = pawn_moves(&bitboard, &self.all_white_pieces(), &self.all_black_pieces(), true);
      },
      PieceType::BlackPawn => {
        moves = pawn_moves(&bitboard, &self.all_black_pieces(), &self.all_white_pieces(), false);
      },
      _ => {
        panic!("Piece type not found");
      }
    }

    self.generate_moves(&bitboard, &moves, piece_type)
  }

  pub fn get_bitboards(&self) -> [u64; 12] {
    self.bitboards
  }
}