mod move_gen;
mod precompiled_bitboards;

use std::collections::HashMap;
use move_gen::*;
use crate::utils::PieceType;

const EMPTY_VEC: Vec<Move> = Vec::new(); // have to store Vec::new() as a const as to allow for the copying of it

const H1: u64 = 0x1;
const A1: u64 = 0x80;
const H8: u64 = 0x100000000000000;
const A8: u64 = 0x8000000000000000;

pub fn bits_to_indices(bitboard: &u64) -> Vec<i32> {
  let mut indices = Vec::new();
  for i in 0..64 {
    if *bitboard & (1 << i) != 0 {
      indices.push(i);
    }
  }
  indices
}

#[derive(Clone)]
struct CastlingRights {
  white_kingside: bool,
  white_queenside: bool,
  black_kingside: bool,
  black_queenside: bool
}
impl CastlingRights {
  fn new() -> Self {
    Self {
      white_kingside: true,
      white_queenside: true,
      black_kingside: true,
      black_queenside: true
    }
  }
}

#[derive(Clone)]
struct CastlingFlags {
  king_moved: bool,
  rook_kingside_moved: bool,
  rook_queenside_moved: bool,
}
impl CastlingFlags {
  fn new() -> Self {
    Self {
      king_moved: false,
      rook_kingside_moved: false,
      rook_queenside_moved: false,
    }
  }
}

#[derive(Copy, Clone)]
pub struct MoveFlags {
  passented_square: Option<u64>,
  can_be_en_passent: bool,

  kingside_castle_square: Option<u64>,
  queenside_castle_square: Option<u64>,

  is_promotion: bool,
}
impl MoveFlags {
  pub fn new() -> Self {
    Self {
      passented_square: None,
      can_be_en_passent: false,
  
      kingside_castle_square: None,
      queenside_castle_square: None,
  
      is_promotion: false
    }
  }
}

#[derive(Copy, Clone)]
pub struct Move {
  pub start_square: i32,
  pub end_square: i32,
  moved_piece_type: PieceType,

  // flags
  captured_piece_type: Option<PieceType>,
  flags: MoveFlags,
  pub promotion_piece: Option<PieceType>
}
impl Move {
  pub fn new(move_start_square: i32, move_end_square: i32, piece: PieceType, move_flags: MoveFlags) -> Self {
    Self {
      start_square: move_start_square,
      end_square: move_end_square,
      moved_piece_type: piece,

      captured_piece_type: None,
      flags: move_flags,
      promotion_piece: None
    }
  }
}
impl PartialEq for Move {
  fn eq(&self, other: &Self) -> bool {
    self.start_square == other.start_square && self.end_square == other.end_square && self.promotion_piece == other.promotion_piece
  }
}

#[derive(Clone)]
pub struct Board {
  bitboards: [u64; 12],
  white_to_move: bool,
  castling_rights: CastlingRights,
  en_passent_square: Option<u64>,
  halfmove_clock: i32,
  fullmove_num: i32,
  white_castling_flags: CastlingFlags,
  black_castling_flags: CastlingFlags,

  moves: [Vec<Move>; 64],
  enemy_attacks: u64,
  checks: Vec<u64>,
  pinned_pieces: u64
}
impl Board {
  /* BOARD SETUP */
  pub fn new(fen: &str) -> Self {
    let mut new_board = Self {
      bitboards: [0; 12],
      white_to_move: true,
      castling_rights: CastlingRights::new(),
      en_passent_square: None,
      halfmove_clock: 0,
      fullmove_num: 0,
      white_castling_flags: CastlingFlags::new(),
      black_castling_flags: CastlingFlags::new(),

      moves: [EMPTY_VEC; 64],
      enemy_attacks: 0,
      checks: Vec::new(),
      pinned_pieces: 0
    };
    new_board.parse_fen(fen);
    new_board.detect_check();
    new_board.get_opponents_attacks();
    new_board.castle_checks();
    new_board.find_pinned_pieces();
    new_board.get_all_legal_moves();
    new_board
  }
  fn parse_fen(&mut self, fen: &str) {
    let mut parts = fen.split(' ');
    let position = parts.next().unwrap();
    let side_to_move = parts.next().unwrap();
    let _  = parts.next().unwrap(); // dont care about this part because thats not how my engine implements castling
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
    self.white_castling_flags.king_moved = self.bitboards[PieceType::WhiteKing as usize] & 0x8 == 0;
    self.white_castling_flags.rook_kingside_moved = self.bitboards[PieceType::WhiteRook as usize] & H1 == 0;
    self.white_castling_flags.rook_queenside_moved = self.bitboards[PieceType::WhiteRook as usize] & A1 == 0;
    self.black_castling_flags.king_moved = self.bitboards[PieceType::BlackKing as usize] & 0x800000000000000 == 0;
    self.black_castling_flags.rook_kingside_moved = self.bitboards[PieceType::BlackRook as usize] & H8 == 0;
    self.black_castling_flags.rook_queenside_moved = self.bitboards[PieceType::BlackRook as usize] & A8 == 0;

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

      self.en_passent_square = Some(1 << square);
    }

    // halfmove_clock
    self.halfmove_clock = halfmove_clock.parse().unwrap();

    // fullmove_num
    self.fullmove_num = fullmove_num.parse().unwrap();
  }

  /* HELPER FUNCTIONS */
  fn all_white_pieces(&self) -> u64 {
    self.bitboards[PieceType::WhiteKing as usize] | self.bitboards[PieceType::WhiteQueen as usize] | self.bitboards[PieceType::WhiteBishop as usize] | self.bitboards[PieceType::WhiteKnight as usize] | self.bitboards[PieceType::WhiteRook as usize] | self.bitboards[PieceType::WhitePawn as usize]
  }
  fn all_black_pieces(&self) -> u64 {
    self.bitboards[PieceType::BlackKing as usize] | self.bitboards[PieceType::BlackQueen as usize] | self.bitboards[PieceType::BlackBishop as usize] | self.bitboards[PieceType::BlackKnight as usize] | self.bitboards[PieceType::BlackRook as usize] | self.bitboards[PieceType::BlackPawn as usize]
  }
  fn are_squares_attacked(&self, squares: u64) -> bool {
    self.enemy_attacks & squares != 0
  }
  // getters
  pub fn get_bitboards(&self) -> [u64; 12] {
    self.bitboards
  }
  pub fn get_if_white_to_move(&self) -> bool {
    self.white_to_move
  }
  pub fn get_moves(&self, index: i32) -> &Vec<Move> {
    &self.moves[index as usize]
  }
  pub fn get_all_moves(&self) -> Vec<Move> {
    let mut moves = Vec::new();

    for piece_moves in &self.moves {
      for piece_move in piece_moves {
        moves.push(piece_move.clone());
      }
    }

    moves
  }

  /* MOVE GEN */
  fn get_opponents_attacks(&mut self) {
    self.enemy_attacks = 0;

    for piece_type in PieceType::get_colour_types(!self.white_to_move) {
      for square in bits_to_indices(&self.bitboards[piece_type as usize]) {
        self.enemy_attacks |= self.get_legal_moves(square, piece_type, true).0;
      }
    }
  }
  fn castle_checks(&mut self) {
    if !self.white_castling_flags.king_moved {
      self.castling_rights.white_kingside = !self.white_castling_flags.rook_kingside_moved && self.all_white_pieces() & 0x6 == 0 && self.are_squares_attacked(0x6);       
      self.castling_rights.white_queenside = !self.white_castling_flags.rook_queenside_moved && self.all_white_pieces() & 0x70 == 0 && self.are_squares_attacked(0x70);
    }
    else {
      self.castling_rights.white_kingside = false;
      self.castling_rights.white_queenside = false;
    }
  
    if !self.black_castling_flags.king_moved {
      self.castling_rights.black_kingside = !self.black_castling_flags.rook_kingside_moved && self.all_black_pieces() & 0x600000000000000 == 0 && self.are_squares_attacked(0x600000000000000);
      self.castling_rights.black_queenside = !self.black_castling_flags.rook_queenside_moved && self.all_black_pieces() & 0x7000000000000000 == 0 && self.are_squares_attacked(0x7000000000000000);
    }
    else {
      self.castling_rights.black_kingside = false;
      self.castling_rights.black_queenside = false;
    }
    
  }
  fn detect_check(&mut self) {
    self.checks = Vec::new();

    for i in 0..63 {
      for piece_type in PieceType::get_colour_types(self.white_to_move) {
        if 1 << i & self.bitboards[piece_type as usize] == 0 {
          continue;
        }
        let moves_bitboards = self.get_legal_moves(i, piece_type, false).0;
        let enemy_king = if self.white_to_move { self.bitboards[PieceType::BlackKing as usize] } else { self.bitboards[PieceType::WhiteKing as usize] };
        let check_move = moves_bitboards & enemy_king; // if the move is capturing the king
        if check_move == 0 {
          continue;
        }
        match piece_type {
          PieceType::WhiteKnight | PieceType::BlackKnight | PieceType::WhitePawn | PieceType::BlackPawn => {
            self.checks.push(1 << i);
          },
          PieceType::WhiteQueen | PieceType::BlackQueen | PieceType::WhiteBishop | PieceType::BlackBishop | PieceType::WhiteRook | PieceType::BlackRook  => {
            let delta = i - enemy_king.trailing_zeros() as i32;
            let direction = match delta {
              d if d % 8 == 0 => 8, // vertical
              d if d % 7 == 0 => 7, // diagonal /
              d if d % 9 == 0 => 9, // diagonal \
              d if d.abs() < 8 => 1, // horizontal
              _ => 0,
            };
            let mut ray = 0;
            let mut pos = i as i32;
            while pos >= 0 && pos < 64 {
              if pos == enemy_king.trailing_zeros() as i32 {
                break;
              }
              ray |= 1 << pos;
              pos += direction;
            }
            self.checks.push(ray | (1 << i));
          }
          _ => {
            // do nothin
          }
        }
      }
    }
  }
  fn find_pinned_pieces(&mut self) {
    self.pinned_pieces = 0;

    let king = if self.white_to_move { self.bitboards[PieceType::WhiteKing as usize] } else { self.bitboards[PieceType::BlackKing as usize] };
    let king_square = king.trailing_zeros() as i32;
    let sliders = if self.white_to_move {
      [PieceType::BlackQueen, PieceType::BlackRook, PieceType::BlackQueen, PieceType::BlackBishop]
    } else {
      [PieceType::WhiteQueen, PieceType::WhiteRook, PieceType::WhiteQueen, PieceType::WhiteBishop]
    };
    let orthogonal_rays = get_rook_moves(king_square, &0);
    let diagonal_rays = get_bishop_moves(king_square, &0);
    
    for (i, slider_type) in sliders.iter().enumerate() {
      let is_diagonal = i > 1; // if the itteration count is past the second one, its diagonal

      let slider_type_bitboard = self.bitboards[*slider_type as usize];
      if slider_type_bitboard & (orthogonal_rays | diagonal_rays) == 0 {
        continue;
      }
      
      for i in 0..63 {
        let piece_bitboard = 1 << i;
        if piece_bitboard & slider_type_bitboard == 0 {
          continue;
        }
        
        let delta = king_square - i;
        let directional_mask = if delta < 0 {
          if is_diagonal {
            if delta % 9 == 0 {
              // up left
              let mut mask = 0;
              let mut square = king;
          
              while square & (TOP_RANK | LEFT_FILE) == 0 {
                square = square << 9;
                if square & piece_bitboard != 0 {
                  break;
                }

                mask |= square;
              }
          
              mask
            }
            else if delta % 7 == 0 {
              // up right
              let mut mask = 0;
              let mut square = king;
          
              while square & (TOP_RANK | RIGHT_FILE) == 0 {
                square = square << 7;
                if square & piece_bitboard != 0 {
                  break;
                }

                mask |= square;
              }
          
              mask
            }
            else {
              0
            }
          }
          else {
            if delta > 7 {
              // up
              let mut mask = 0;
              let mut square = king;
          
              while square & TOP_RANK == 0 {
                square = square << 8;
                if square & piece_bitboard != 0 {
                  break;
                }

                mask |= square;
              }
            
              mask
            }
            else {
              // left
              let mut mask = 0;
              let mut square = king;
          
              while square & LEFT_FILE == 0 {
                square = square << 1;
                if square & piece_bitboard != 0 {
                  break;
                }

                mask |= square;
              }
            
              mask
            }
          }
        }
        else {
          if is_diagonal {
            if delta.abs() % 9 == 0 {
              // down right
              let mut mask = 0;
              let mut square = king;
          
              while square & (BOTTOM_RANK | RIGHT_FILE) == 0 {
                square = square >> 9;
                if square & piece_bitboard != 0 {
                  break;
                }

                mask |= square;
              }
          
              mask
            }
            else if delta.abs() % 7 == 0{
              // down left
              let mut mask = 0;
              let mut square = king;
          
              while square & (BOTTOM_RANK | LEFT_FILE) == 0 {
                square = square >> 7;
                if square & piece_bitboard != 0 {
                  break;
                }

                mask |= square;
              }
          
              mask
            }
            else {
              0
            }
          }
          else {
            if delta.abs() > 7 {
              // down
              let mut mask = 0;
              let mut square = king;
          
              while square & BOTTOM_RANK == 0 {
                square = square >> 8;
                if square & piece_bitboard != 0 {
                  break;
                }

                mask |= square;
              }
            
              mask
            }
            else {
              // right
              let mut mask = 0;
              let mut square = king;
          
              while square & RIGHT_FILE == 0 {
                square = square >> 1;
                if square & piece_bitboard != 0 {
                  break;
                }

                mask |= square;
              }
            
              mask
            }
          }
        };
        let rays = if is_diagonal { diagonal_rays } else { orthogonal_rays };
        let ray = rays & directional_mask;
        
        let enemy_occupation = if self.white_to_move { self.all_black_pieces() } else { self.all_white_pieces() };
        if enemy_occupation & ray != 0 {
          continue;
        }
        
        let friendly_occupation = if self.white_to_move { self.all_white_pieces() } else { self.all_black_pieces() };
        let friendly_blockers = friendly_occupation & ray;
        if friendly_blockers == 0 {
          continue;
        }
        if friendly_blockers.count_ones() > 1 {
          continue;
        }

        self.pinned_pieces |= friendly_blockers;
      }
    }
  }

  fn generate_moves_from_bitboard(&self, piece_square: i32, moves_bitboard: u64, piece_type: PieceType, flags: MoveFlags) -> Vec<Move>{
    let mut moves: Vec<Move> = Vec::new();

    for square in bits_to_indices(&moves_bitboard) {
      let mut new_move = Move::new(piece_square, square, piece_type, flags);

      for piece_type in PieceType::iter() {
        if self.bitboards[piece_type as usize] & 1 << square != 0 { // if the square already has something on it
          new_move.captured_piece_type = Some(piece_type);
        }
      }

      if flags.is_promotion {
        if self.white_to_move {
          new_move.promotion_piece = Some(PieceType::WhiteQueen);
          moves.push(new_move.clone());
          new_move.promotion_piece = Some(PieceType::WhiteKnight);
          moves.push(new_move.clone());
          new_move.promotion_piece = Some(PieceType::WhiteBishop);
          moves.push(new_move.clone());
          new_move.promotion_piece = Some(PieceType::WhiteRook);
          moves.push(new_move.clone());
        }
        else {
          new_move.promotion_piece = Some(PieceType::BlackQueen);
          moves.push(new_move.clone());
          new_move.promotion_piece = Some(PieceType::BlackKnight);
          moves.push(new_move.clone());
          new_move.promotion_piece = Some(PieceType::BlackBishop);
          moves.push(new_move.clone());
          new_move.promotion_piece = Some(PieceType::BlackRook);
          moves.push(new_move.clone());
        }
      }
      else {
        moves.push(new_move);
      }
    }

    moves
  }
  fn get_legal_moves(&mut self, square_index: i32, piece_type: PieceType, only_attacks: bool) -> (u64, MoveFlags) {
    let mut moves = 0;
    let mut flags = MoveFlags::new();

    let bitboard = 1 << square_index;
    let occupancy = self.all_white_pieces() | self.all_black_pieces();

    match piece_type {
      PieceType::WhiteKing => {
        
        (moves, flags.kingside_castle_square, flags.queenside_castle_square) = king_moves(&bitboard, self.castling_rights.white_kingside, self.castling_rights.white_queenside);
        
        if !only_attacks {
          moves ^= moves & self.all_white_pieces(); 
          moves ^= moves & self.enemy_attacks;
        }
      },
      PieceType::BlackKing => {
        (moves, flags.kingside_castle_square, flags.queenside_castle_square) = king_moves(&bitboard, self.castling_rights.black_kingside, self.castling_rights.black_queenside);

        if !only_attacks {
          moves ^= moves & self.all_black_pieces(); 
          moves ^= moves & self.enemy_attacks;
        }
      },
      PieceType::WhiteQueen => {
        let diagonal_moves = get_bishop_moves(square_index, &occupancy);
        let orthogonal_moves = if !only_attacks {
          get_rook_moves(square_index, &occupancy)
        } else {
          get_rook_moves(square_index, &0)
        };
        moves = diagonal_moves | orthogonal_moves;

        if !only_attacks {
          moves ^= moves & self.all_white_pieces(); 
        }
      },
      PieceType::BlackQueen => {
        let diagonal_moves = if !only_attacks {
          get_bishop_moves(square_index, &occupancy)
        } else {
          get_bishop_moves(square_index, &0)
        };
        let orthogonal_moves = if !only_attacks {
          get_rook_moves(square_index, &occupancy)
        } else {
          get_rook_moves(square_index, &0)
        };
        moves = diagonal_moves | orthogonal_moves;

        if !only_attacks {
          moves ^= moves & self.all_black_pieces(); 
        }
      },
      PieceType::WhiteBishop => {
        moves = if !only_attacks {
          get_bishop_moves(square_index, &occupancy)
        } else {
          get_bishop_moves(square_index, &0)
        };
        
        if !only_attacks {
          moves ^= moves & self.all_white_pieces(); 
        }
      },
      PieceType::BlackBishop => {
        moves = if !only_attacks {
          get_bishop_moves(square_index, &occupancy)
        } else {
          get_bishop_moves(square_index, &0)
        };

        if !only_attacks {
          moves ^= moves & self.all_black_pieces(); 
        }
      },
      PieceType::WhiteKnight => {
        moves = knight_moves(&bitboard);

        if !only_attacks {
          moves ^= moves & self.all_white_pieces(); 
        }
      },
      PieceType::BlackKnight => {
        moves = knight_moves(&bitboard);

        if !only_attacks {
          moves ^= moves & self.all_black_pieces(); 
        }
      },
      PieceType::WhiteRook => {
        moves = if !only_attacks {
          get_rook_moves(square_index, &occupancy)
        } else {
          get_rook_moves(square_index, &0)
        };

        if !only_attacks {
          moves ^= moves & self.all_white_pieces(); 
        }
      },
      PieceType::BlackRook => {
        moves = if !only_attacks {
          get_rook_moves(square_index, &occupancy)
        } else {
          get_rook_moves(square_index, &0)
        };

        if !only_attacks {
          moves ^= moves & self.all_black_pieces(); 
        }
      },
      PieceType::WhitePawn => {
        let mut attacks;
        let mut is_move_promotion = false;
        let is_attack_promotion;

        (attacks, flags.can_be_en_passent, is_attack_promotion) = pawn_attacks(&bitboard, true, self.en_passent_square);
        if !only_attacks {
          (moves, flags.passented_square, is_move_promotion) = pawn_moves(&bitboard, &occupancy, true);
        
          if flags.can_be_en_passent {
            attacks &= self.all_black_pieces() | self.en_passent_square.unwrap();
          }
          else {
            attacks &= self.all_black_pieces();
          }
        }

        flags.is_promotion = is_move_promotion || is_attack_promotion;
        moves |= attacks;
         
      },
      PieceType::BlackPawn => {
        let mut attacks;
        let mut is_move_promotion = false;
        let is_attack_promotion;

        (attacks, flags.can_be_en_passent, is_attack_promotion) = pawn_attacks(&bitboard, false, self.en_passent_square);
        if !only_attacks {
          (moves, flags.passented_square, is_move_promotion) = pawn_moves(&bitboard, &occupancy, false);

          if flags.can_be_en_passent {
            attacks &= self.all_white_pieces() | self.en_passent_square.unwrap();
          }
          else {
            attacks &= self.all_white_pieces();
          }
        }
        
        flags.is_promotion = is_move_promotion || is_attack_promotion;
        moves |= attacks;
      }
    }

    (moves, flags)
  }
  fn get_all_legal_moves(&mut self) {
    let mut all_pseudo_legal_moves: [Vec<Move>; 64] = [EMPTY_VEC; 64];
    
    for piece_type in PieceType::get_colour_types(self.white_to_move) {
      for i in 0..64 {
        let square_bitboard = 1 << i;
        if square_bitboard & self.pinned_pieces != 0 {
          continue;
        }

        let bitboard = self.bitboards[piece_type as usize];

        if bitboard & square_bitboard != 0 {
          let piece_moves = self.get_legal_moves(i, piece_type, false);
          all_pseudo_legal_moves[i as usize] = self.generate_moves_from_bitboard(i, piece_moves.0, piece_type, piece_moves.1);
        }
      }
    }

    let mut all_moves: [Vec<Move>; 64] = [EMPTY_VEC; 64];
    for check in &self.checks {
      for (i, piece_moves) in all_pseudo_legal_moves.iter().enumerate() {
        let mut moves = Vec::new();
        for piece_move in piece_moves {
          if piece_move.moved_piece_type == PieceType::WhiteKing || piece_move.moved_piece_type == PieceType::BlackKing || check & 1 << piece_move.end_square != 0 {
            moves.push(*piece_move);
          }
        }
        all_moves[i] = moves;
      }
    }

    if self.checks.len() == 0 {
      let is_all_empty = all_moves.iter().all(|m| m.is_empty());
      if is_all_empty {
        self.moves = all_pseudo_legal_moves;
        return;
      }
    }
    self.moves = all_moves;
  }

  pub fn make_move(&mut self, move_to_make: Move) {
    self.castle_checks();
    let new_piece_bitboard = 1 << move_to_make.end_square;
    let old_piece_bitboard = 1 << move_to_make.start_square;

    if move_to_make.flags.is_promotion {
      self.bitboards[move_to_make.moved_piece_type as usize] ^= old_piece_bitboard;
      self.bitboards[move_to_make.promotion_piece.unwrap() as usize] |= new_piece_bitboard; 
    }
    else {
      self.bitboards[move_to_make.moved_piece_type as usize] ^= old_piece_bitboard | new_piece_bitboard;
    }
    
    if let Some(piece_type) = move_to_make.captured_piece_type {
      self.bitboards[piece_type as usize] ^= new_piece_bitboard;
    }

    // remove the passented piece
    if move_to_make.flags.can_be_en_passent {
      if self.en_passent_square.unwrap() & new_piece_bitboard != 0 {
        if self.white_to_move {
          self.bitboards[PieceType::BlackPawn as usize] ^= self.en_passent_square.unwrap() >> 8;
        }
        else {
          self.bitboards[PieceType::WhitePawn as usize] ^= self.en_passent_square.unwrap() << 8;
        }
        self.en_passent_square = None;
      }
    }

    if let Some(square) = move_to_make.flags.passented_square {
      if new_piece_bitboard & square != 0 {
        self.en_passent_square = if move_to_make.moved_piece_type == PieceType::WhitePawn {
          Some(square >> 8)
        }
        else {
          Some(square << 8)
        };
      }
    }
    else {
      self.en_passent_square = None;
    }

    // castling
    if let Some(castle_square) = move_to_make.flags.kingside_castle_square {
      if castle_square & new_piece_bitboard != 0 {
        if self.white_to_move {
          self.bitboards[PieceType::WhiteRook as usize] ^= 0x5;
        }
        else {
          self.bitboards[PieceType::BlackRook as usize] ^= 0x500000000000000;
        }
      }
    }
    if let Some(castle_square) = move_to_make.flags.queenside_castle_square {
      if castle_square & new_piece_bitboard != 0 {
        if self.white_to_move {
          self.bitboards[PieceType::WhiteRook as usize] ^= 0x90;
        }
        else {
          self.bitboards[PieceType::BlackRook as usize] ^= 0x9000000000000000;
        }
      }
    }

    if !self.white_castling_flags.king_moved { // remove unneccecary checks
      if PieceType::WhiteKing == move_to_make.moved_piece_type {
        self.white_castling_flags.king_moved = true;
      }
      else if PieceType::WhiteRook == move_to_make.moved_piece_type { // if the rook moves
        if old_piece_bitboard & H1 != 0 {
          self.white_castling_flags.rook_kingside_moved = true;
        }
        else if old_piece_bitboard & A1 != 0 {
          self.white_castling_flags.rook_queenside_moved = true;
        }
      }
      else if move_to_make.captured_piece_type.is_some() && PieceType::WhiteRook == move_to_make.captured_piece_type.unwrap() { // if the rook is captured
        if new_piece_bitboard & H1 != 0 {
          self.white_castling_flags.rook_kingside_moved = true;
        }
        else if new_piece_bitboard & A1 != 0 {
          self.white_castling_flags.rook_queenside_moved = true;
        }
      }
    }
    if !self.black_castling_flags.king_moved { // remove unneccecary checks
      if PieceType::BlackKing == move_to_make.moved_piece_type {
        self.black_castling_flags.king_moved = true;
      }
      else if PieceType::BlackRook == move_to_make.moved_piece_type { // if the rook moves
        if old_piece_bitboard & H1 != 0 {
          self.white_castling_flags.rook_kingside_moved = true;
        }
        else if old_piece_bitboard & A1 != 0 {
          self.white_castling_flags.rook_queenside_moved = true;
        }
      }
      else if move_to_make.captured_piece_type.is_some() && PieceType::BlackRook == move_to_make.captured_piece_type.unwrap() { // if the rook is captured
        if new_piece_bitboard & H8 != 0 {
          self.white_castling_flags.rook_kingside_moved = true;
        }
        else if new_piece_bitboard & A8 != 0 {
          self.white_castling_flags.rook_queenside_moved = true;
        }
      }
    }

    self.detect_check();
    self.white_to_move = !self.white_to_move;
    self.find_pinned_pieces();
    self.get_opponents_attacks();
    self.get_all_legal_moves();
  }

  pub fn is_checkmate(&self) -> bool {
    self.get_all_moves().len() == 0 && self.checks.len() != 0
  }
}