mod move_gen;
mod precompiled_bitboards;

use std::collections::HashMap;
use move_gen::*;
use crate::utils::PieceType;

const VEC: Vec<Move> = Vec::new(); // using Vec::new() directly or in a let doesnt satisfy copy trait, and you cant return references

pub fn bits_to_indices(bitboard: &u64) -> Vec<i32> {
  let mut indices = Vec::new();
  for i in 0..64 {
    if *bitboard & (1 << i) != 0 {
      indices.push(i);
    }
  }
  indices
}

struct CastlingRights {
  white_kingside: bool,
  white_queenside: bool,
  black_kingside: bool,
  black_queenside: bool
}
impl CastlingRights {
  fn new() -> Self {
    Self {
      white_kingside: false,
      white_queenside: false,
      black_kingside: false,
      black_queenside: false
    }
  }
}

#[derive(Clone)]
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
  white_to_move: bool,
  castling_rights: CastlingRights,
  en_passent_square: Option<i32>,
  halfmove_clock: i32,
  fullmove_num: i32,

  moves: [Vec<Move>; 64]
}
impl Board {
  pub fn new(fen: &str) -> Self {
    let mut new_board = Self {
      bitboards: [0; 12],
      white_to_move: true,
      castling_rights: CastlingRights::new(),
      en_passent_square: None,
      halfmove_clock: 0,
      fullmove_num: 0,

      moves: [VEC; 64]
    };
    new_board.parse_fen(fen);
    new_board.get_all_legal_moves();
    new_board
  }

  fn parse_fen(&mut self, fen: &str) {
    let mut parts = fen.split(' '); // do the rest of the flags later
    let position = parts.next().unwrap();
    let side_to_move = parts.next().unwrap();
    let castling_rights  = parts.next().unwrap();
    let en_passent_square  = parts.next().unwrap();
    let halfmove_clock  = parts.next().unwrap();
    let fullmove_num  = parts.next().unwrap();

    // position
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
          self.bitboards[bitboard_type as usize] |= 1 << square_index;
          x += 1;
        },
        _ => panic!("Unexpected character in position field of FEN string"),
      }
    }

    // side to move
    let side_to_move_chars: Vec<char> = side_to_move.chars().collect();
    if side_to_move_chars.len() != 1 {
      panic!("More than one character in side_to_move field of FEN string");
    }
    self.white_to_move = side_to_move_chars[0] == 'w';

    // castling rights
    for c in castling_rights.chars() {
      match c {
        '-' => { // no castling rights
          break;
        },
        'K' => {
          self.castling_rights.white_kingside = true;
        },
        'Q' => {
          self.castling_rights.white_queenside = true;
        },
        'k' => {
          self.castling_rights.black_kingside = true;
        },
        'q' => {
          self.castling_rights.black_queenside = true;
        },
        _ =>  {
          panic!("Unexpected character in castling_rights field of FEN string");
        }
      }
    }

    // en passent
    if en_passent_square != "-" {
      let en_passent_chars: Vec<char> = en_passent_square.chars().collect();
      let char_to_int: HashMap<char, i32> = HashMap::from([
        ('h', 0),
        ('g', 1),
        ('f', 2),
        ('e', 3),
        ('d', 4),
        ('c', 5),
        ('b', 6),
        ('a', 7)
      ]);
      
      let mut square = char_to_int[&en_passent_chars[0]];
      if let Some(square_num) = en_passent_chars[1].to_digit(10) {
        square += (square_num as i32) * 8;
      }

      self.en_passent_square = Some(square);
    }

    // halfmove_clock
    self.halfmove_clock = halfmove_clock.parse().unwrap();

    // fullmove_num
    self.fullmove_num = fullmove_num.parse().unwrap();
  }

  pub fn generate_moves_from_bitboard(&self, piece_bitboard: &u64, moves_bitboard: &u64, piece_type: PieceType) -> Vec<Move>{
    let mut moves: Vec<Move> = Vec::new();
    let mut piece_square = 0;
    for i in 0..64 {
      if piece_bitboard & 1 << i != 0 {
        piece_square = i;
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
      if piece_square == square {
        continue;
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

    self.white_to_move = !self.white_to_move;
    self.get_all_legal_moves();
  }

  fn all_white_pieces(&self) -> u64 {
    self.bitboards[PieceType::WhiteKing as usize] | self.bitboards[PieceType::WhiteQueen as usize] | self.bitboards[PieceType::WhiteBishop as usize] | self.bitboards[PieceType::WhiteKnight as usize] | self.bitboards[PieceType::WhiteRook as usize] | self.bitboards[PieceType::WhitePawn as usize]
  }
  fn all_black_pieces(&self) -> u64 {
    self.bitboards[PieceType::BlackKing as usize] | self.bitboards[PieceType::BlackQueen as usize] | self.bitboards[PieceType::BlackBishop as usize] | self.bitboards[PieceType::BlackKnight as usize] | self.bitboards[PieceType::BlackRook as usize] | self.bitboards[PieceType::BlackPawn as usize]
  }

  pub fn get_legal_moves(&self, square_index: i32, piece_type: PieceType) -> Vec<Move> {
    let moves;
    let en_passent_flag; // replace with flags struct
    let bitboard = 1 << square_index;

    match piece_type {
      PieceType::WhiteKing => {
        moves = king_moves(&bitboard, &self.all_white_pieces());
      },
      PieceType::BlackKing => {
        moves = king_moves(&bitboard, &self.all_black_pieces());
      },
      PieceType::WhiteQueen => {
        moves = get_bishop_moves(square_index, &self.all_white_pieces(), &self.all_black_pieces()) | get_rook_moves(square_index, &self.all_white_pieces(), &self.all_black_pieces());
      },
      PieceType::BlackQueen => {
        moves = get_bishop_moves(square_index, &self.all_black_pieces(), &self.all_white_pieces()) | get_rook_moves(square_index, &self.all_black_pieces(), &self.all_white_pieces());
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
        (moves, en_passent_flag) = pawn_moves(&bitboard, &self.all_white_pieces(), &self.all_black_pieces(), true, self.en_passent_square);
      },
      PieceType::BlackPawn => {
        (moves, en_passent_flag) = pawn_moves(&bitboard, &self.all_black_pieces(), &self.all_white_pieces(), false, self.en_passent_square);
      }
    }

    self.generate_moves_from_bitboard(&bitboard, &moves, piece_type) // take in an optional flags type then based onthe flag do stuff
  }

  fn get_all_legal_moves(&mut self) {
    let mut moves: [Vec<Move>; 64] = [VEC; 64];

    let piece_types = if self.white_to_move {
      PieceType::all_white()
    }
    else {
      PieceType::all_black()
    };

    for piece_type in piece_types {
      for i in 0..64 {
        let bitboard = self.bitboards[piece_type as usize];

        if bitboard & (1 << i) != 0 {
          moves[i as usize] = self.get_legal_moves(i, piece_type);
        }
      }
    }

    self.moves = moves;
  }

  pub fn get_bitboards(&self) -> [u64; 12] {
    self.bitboards
  }
  pub fn get_moves(&self, index: i32) -> &Vec<Move> {
    &self.moves[index as usize]
  }
}