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

#[derive(Clone)]
pub struct Move {
  start_square: i32,
  end_square: i32,
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

  non_sliding_checks: Vec<u64>
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
      white_castling_flags: CastlingFlags::new(), // init these based on FEN somehow
      black_castling_flags: CastlingFlags::new(),

      moves: [EMPTY_VEC; 64],
      enemy_attacks: 0,

      non_sliding_checks: Vec::new()
    };
    new_board.parse_fen(fen);
    new_board.get_opponents_attacks();
    new_board.castle_checks();
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
    if self.bitboards[PieceType::WhiteKing as usize] & 0x8 == 0 {
      self.white_castling_flags.king_moved = true;
    }
    if self.bitboards[PieceType::WhiteRook as usize] & H1 == 0 {
      self.white_castling_flags.rook_kingside_moved = true;
    }
    if self.bitboards[PieceType::WhiteRook as usize] & A1 == 0 {
      self.white_castling_flags.rook_queenside_moved = true;
    }
    if self.bitboards[PieceType::BlackKing as usize] & 0x800000000000000 == 0 {
      self.black_castling_flags.king_moved = true;
    }
    if self.bitboards[PieceType::BlackRook as usize] & H8 == 0 {
      self.black_castling_flags.rook_kingside_moved = true;
    }
    if self.bitboards[PieceType::BlackRook as usize] & A8 == 0 {
      self.black_castling_flags.rook_queenside_moved = true;
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
  fn is_in_check(&self) -> bool {
    let king = if self.white_to_move { self.bitboards[PieceType::WhiteKing as usize] } else { self.bitboards[PieceType::BlackKing as usize] };
    king & self.enemy_attacks != 0
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

  /* MOVE GEN */
  fn get_opponents_attacks(&mut self) {
    self.enemy_attacks = 0;

    let piece_types = if self.white_to_move { PieceType::all_black() } else { PieceType::all_white() };
    for piece_type in piece_types {
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
  fn find_check_moves(&mut self) {
    for i in 0..63 {
      for piece_type in PieceType::iter() {
        let moves = self.get_legal_moves(i, piece_type, false).0;
        let enemy_king = if self.white_to_move { self.bitboards[PieceType::BlackKing as usize] } else { self.bitboards[PieceType::WhiteKing as usize] };
        let check_move = moves & enemy_king;
        if check_move == 0 {
          continue;
        }
        match piece_type {
          PieceType::WhiteKnight | PieceType::BlackKnight | PieceType::WhitePawn | PieceType::BlackPawn => {
            self.non_sliding_checks.push(check_move | (1 << i));
          },
          PieceType::WhiteQueen | PieceType::BlackQueen | PieceType::WhiteBishop | PieceType::BlackBishop | PieceType::WhiteRook | PieceType::BlackRook => {
            //
            // push the check ray to the boards check_rays vector
          }
          _ => {
            // do nothin
          }
        }
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
        let orthogonal_moves = get_rook_moves(square_index, &occupancy);
        moves = diagonal_moves | orthogonal_moves;

        if !only_attacks {
          moves ^= moves & self.all_white_pieces(); 
        }
      },
      PieceType::BlackQueen => {
        let diagonal_moves = get_bishop_moves(square_index, &occupancy);
        let orthogonal_moves = get_rook_moves(square_index, &occupancy);
        moves = diagonal_moves | orthogonal_moves;

        if !only_attacks {
          moves ^= moves & self.all_black_pieces(); 
        }
      },
      PieceType::WhiteBishop => {
        moves = get_bishop_moves(square_index, &occupancy);
        
        if !only_attacks {
          moves ^= moves & self.all_white_pieces(); 
        }
      },
      PieceType::BlackBishop => {
        moves = get_bishop_moves(square_index, &occupancy);

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
        moves = get_rook_moves(square_index, &occupancy);

        if !only_attacks {
          moves ^= moves & self.all_white_pieces(); 
        }
      },
      PieceType::BlackRook => {
        moves = get_rook_moves(square_index, &occupancy);

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
    let mut moves: [Vec<Move>; 64] = [EMPTY_VEC; 64];

    let types = if self.white_to_move { // which side's moves to generate
      PieceType::all_white()
    }
    else {
      PieceType::all_black()
    };
    
    for piece_type in types {
      for i in 0..64 {
        let bitboard = self.bitboards[piece_type as usize];

        if bitboard & (1 << i) != 0 {
          let piece_moves = self.get_legal_moves(i, piece_type, false);
          moves[i as usize] = self.generate_moves_from_bitboard(i, piece_moves.0, piece_type, piece_moves.1);
        }
      }
    }

    self.moves = moves;
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
      self.bitboards[move_to_make.moved_piece_type as usize] ^= old_piece_bitboard | new_piece_bitboard; // move the piece in its own bitboard
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

    self.white_to_move = !self.white_to_move;
    self.get_opponents_attacks();
    self.get_all_legal_moves();
  }
}