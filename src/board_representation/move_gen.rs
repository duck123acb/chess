use crate::board_representation::precompiled_bitboards::*;

// from white's perspective
const TOP_RANK: u64 = 0xFF00000000000000;
const BOTTOM_RANK: u64 = 0x00000000000000FF;
const LEFT_FILE: u64 = 0x8080808080808080;
const RIGHT_FILE: u64 = 0x0101010101010101;

const RANK_SHIFT: i32 = 8; // value to shift if you want to move ranks
const FILE_SHIFT: i32 = 1; // value to shift if you want to move files

pub fn pawn_moves(bitboard: &u64, friendly_bitboard: &u64, enemy_bitboard: &u64, is_white: bool, en_passent_square: Option<u64>) -> (u64, Option<u64>, Option<u64>, bool) {
  let all_pieces = friendly_bitboard | enemy_bitboard;
  let mut moves: u64 = 0;
  let mut attacks: u64 = 0;

  // flags
  let mut can_be_passented_square = None; // square that pawns can be passented  on (https://www.youtube.com/shorts/wOdObmJ-q9A)
  let mut is_passenting_square = None; // square that the passenting piece ends up on
  let mut is_promotion = false;

  if is_white {
    let pawn_move = bitboard << RANK_SHIFT;
    moves |= pawn_move;

    if pawn_move & TOP_RANK != 0 {
      is_promotion = true;
    }

    if bitboard & (BOTTOM_RANK << RANK_SHIFT) != 0 { // if pawn is on 2nd rank
      let move_square  = bitboard << (RANK_SHIFT * 2);
      moves |= move_square;
      can_be_passented_square = Some(move_square);
    }

    if bitboard & RIGHT_FILE == 0 { // if piece is not on the left file
      attacks |= bitboard << (RANK_SHIFT - 1)
    }
    if bitboard & LEFT_FILE == 0 { // if piece is not on the right file
      attacks |= bitboard << (RANK_SHIFT + 1);
    }
  } else {
    let pawn_move = bitboard >> RANK_SHIFT;
    moves |= pawn_move;

    if pawn_move & BOTTOM_RANK != 0 {
      is_promotion = true
    }
    if bitboard & (TOP_RANK >> RANK_SHIFT) != 0 { // if pawn is on 7th rank
      let move_square  = bitboard >> (RANK_SHIFT * 2);
      moves |= move_square;
      can_be_passented_square = Some(move_square);
    }

    if bitboard & RIGHT_FILE == 0 { // if piece is not on the left file
      attacks |= bitboard >> (RANK_SHIFT + 1)
    }
    if bitboard & LEFT_FILE == 0 { // if piece is not on the right file
      attacks |= bitboard >> (RANK_SHIFT - 1);
    }
  }

  moves ^= all_pieces & moves; // removes squares where another piece is. doesnt affect the pawn attacks
  if let Some(square) = en_passent_square { // allows for the capture of en_passent
    let en_passent_attack = attacks & square;
    if en_passent_attack != 0 {
      moves |= en_passent_attack;
      is_passenting_square = Some(en_passent_attack);
    }
  }
  
  attacks ^= attacks & friendly_bitboard; // removes attacks on friendly pieces

  

  if attacks & all_pieces == 0 { // if the pawn attacks nothing
    attacks = 0; // attacks mean nothing
  }

  moves |= attacks;
  (moves, can_be_passented_square, is_passenting_square, is_promotion)
}

pub fn knight_moves(bitboard: &u64, friendly_bitboard: &u64) -> u64 {
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
pub fn king_moves(bitboard: &u64, friendly_bitboard: &u64, castle_kingside: bool, castle_queenside: bool) -> (u64, Option<u64>, Option<u64>) {
  let mut moves = 0;
  let mut is_castles_kingside = None;
  let mut is_castles_queenside = None;

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

  if castle_kingside {
    let castle = bitboard >> 2;
    moves |= castle;
    is_castles_kingside = Some(castle);
  }
  if castle_queenside {
    let castle = bitboard << 2;
    moves |= castle;
    is_castles_queenside = Some(castle)
  }

  (moves, is_castles_kingside, is_castles_queenside)
}

pub fn get_magic_index(magic: u64, index_bits: u32, mask: u64, population: &u64) -> usize {
  let blockers = population & mask;

  (blockers.wrapping_mul(magic) >> index_bits) as usize
}
// FIXME: sliding pieces can capture each other
pub fn get_bishop_moves(square_index: i32, friendly_bitboard: &u64, enemy_bitboard: &u64) -> u64 {
  let population = friendly_bitboard | enemy_bitboard;
  
  let magic = &BISHOP_MAGICS[square_index as usize];
  let mask = &BISHOP_MASKS[square_index as usize];
  let relevant_bits = &BISHOP_BITS[square_index as usize];

  let mut moves = BISHOP_MOVES[square_index as usize][get_magic_index(*magic, *relevant_bits, *mask, &population)];
  moves ^= moves & friendly_bitboard;
  
  moves
}
pub fn get_rook_moves(square_index: i32, friendly_bitboard: &u64, enemy_bitboard: &u64) -> u64 {
  let population = friendly_bitboard | enemy_bitboard;
  
  let magic = &ROOK_MAGICS[square_index as usize];
  let mask = &ROOK_MASKS[square_index as usize];
  let relevant_bits = &ROOK_BITS[square_index as usize];

  let mut moves = ROOK_MOVES[square_index as usize][get_magic_index(*magic, *relevant_bits, *mask, &population)];
  moves ^= moves & friendly_bitboard;
  
  moves
}