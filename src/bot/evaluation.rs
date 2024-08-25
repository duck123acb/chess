use crate::board_representation::Board;
use crate::utils::PieceType;

// names are slightly misleading, but they might as well be as they are as high as high can be (for 32 bit integers)
pub const INFINITY: i32 = i32::MAX;
pub const NEGATIVE_INFINITY: i32 = i32::MIN;

pub const STARTING_DEPTH: i32 = 5;

const BISHOP_PAIR_VALUE: i32 = 50;

const WHITE_PAWN_PIECE_TABLE: [i32; 64] = [
	000,  000,  000,  000,  000,  000,  000,  000,
	100,  100,  100,  100,  100,  100,  100,  100,
	100,  100,  100,  100,  100,  100,  100,  100,
	100,  100,  100,  125,  125,  100,  100,  100,
	100,  100,  125,  150,  150,  125,  100,  100,
	150,  125,  100,  100,  100,  100,  125,  150,
	150,  150,  150,  100,  100,  150,  150,  150,
	000,  000,  000,  000,  000,  000,  000,  000,
];
const BLACK_PAWN_PIECE_TABLE: [i32; 64] = [
	000,  000,  000,  000,  000,  000,  000,  000,
	150,  150,  150,  100,  100,  150,  150,  150,
	150,  125,  100,  100,  100,  100,  125,  150,
	100,  100,  125,  150,  150,  125,  100,  100,
	100,  100,  100,  100,  100,  100,  100,  100,
	100,  100,  100,  125,  125,  100,  100,  100,
	100,  100,  100,  100,  100,  100,  100,  100,
	000,  000,  000,  000,  000,  000,  000,  000,
];
const KNIGHT_PIECE_TABLE: [i32; 64] = [
  225,  250,  250,  250,  250,  250,  250,  225,
  250,  275,  275,  275,  275,  275,  275,  250,
  250,  275,  300,  300,  300,  300,  275,  250,
  250,  275,  300,  325,  325,  300,  275,  250,
  250,  275,  300,  325,  325,  300,  275,  250,
  250,  275,  300,  300,  300,  300,  275,  250,
  250,  275,  275,  275,  275,  275,  275,  250,
  225,  250,  250,  250,  250,  250,  250,  225,
];
const BISHOP_PIECE_TABLE: [i32; 64] = [
  275,  275,  275,  275,  275,  275,  275,  275,
  300,  350,  300,  325,  325,  300,  350,  300,
  300,  300,  300,  300,  300,  300,  300,  300,
  300,  325,  350,  300,  300,  350,  325,  300,
  300,  300,  350,  300,  300,  350,  300,  300,
  300,  300,  300,  300,  300,  300,  300,  300,
  300,  300,  300,  325,  325,  300,  350,  300,
  275,  275,  275,  275,  275,  275,  275,  275,
];
const ROOK_PIECE_TABLE: [i32; 64] = [
  475,  475,  500,  525,  525,  500,  475,  475,
  400,  500,  500,  500,  500,  500,  500,  400,
  400,  500,  500,  500,  500,  500,  500,  400,
  400,  500,  500,  500,  500,  500,  500,  400,
  400,  500,  500,  500,  500,  500,  500,  400,
  400,  500,  500,  500,  500,  500,  500,  400,
  400,  500,  500,  500,  500,  500,  500,  400,
  475,  475,  500,  525,  525,  500,  475,  475,
];
const QUEEN_PIECE_TABLE: [i32; 64] = [
  900,  900,  925,  950,  950,  925,  900,  900,
  900,  900,  900,  900,  900,  900,  900,  900,
  900,  900,  875,  875,  875,  875,  900,  900,
  900,  900,  875,  800,  800,  875,  900,  900,
  900,  900,  875,  800,  800,  875,  900,  900,
  900,  900,  900,  900,  875,  875,  900,  900,
  900,  900,  900,  900,  900,  900,  900,  900,
  900,  900,  925,  950,  950,  925,  900,  900,
];

fn get_piece_value(piece_type: PieceType, square_index: usize) -> i32 {
  match piece_type {
    PieceType::WhiteQueen => {
      QUEEN_PIECE_TABLE[square_index]
    },
    PieceType::BlackQueen => {
      -QUEEN_PIECE_TABLE[square_index]
    },
    PieceType::WhiteBishop => {
      BISHOP_PIECE_TABLE[square_index]
    },
    PieceType::BlackBishop => {
      -BISHOP_PIECE_TABLE[square_index]
    },
    PieceType::WhiteKnight => {
      KNIGHT_PIECE_TABLE[square_index]
    },
    PieceType::BlackKnight => {
      -KNIGHT_PIECE_TABLE[square_index]
    },
    PieceType::WhiteRook => {
      ROOK_PIECE_TABLE[square_index]
    },
    PieceType::BlackRook => {
      -ROOK_PIECE_TABLE[square_index]
    },
    PieceType::WhitePawn=> {
      WHITE_PAWN_PIECE_TABLE[square_index]
    },
    PieceType::BlackPawn => {
      -BLACK_PAWN_PIECE_TABLE[square_index]
    },
    _ => {
      0
    }
  }
}

/*
I played this short game to come up with this list https://www.chess.com/analysis/game/live/118012214443?tab=analysis&move=57
things to add to evaluation:
- prevent checkmatre √
- mate the opponent √
- added bonus for the bishop pair
- passed pawn + protected passed pawn bonus
- piece activity (like rooks on the 7th/2nd, octopus knights, sniper bishops)√
- penalty for split pawns
- king safety
*/
pub fn evaluate_position(board: Board, is_mate:bool, is_white: bool, depth: i32) -> i32 {
  let bitboards = board.get_bitboards();
  let mut eval = 0;
  if is_mate {
    eval = NEGATIVE_INFINITY;

    let increment = 5;
    eval += increment * (STARTING_DEPTH - depth);
    if !is_white {
      eval *= 1;
    }

    return eval;
  }

  for piece_type in PieceType::iter() {
    let bitboard = bitboards[piece_type as usize];
    for square_index in 0..64 {
      if (1 << square_index) & bitboard == 0 {
        continue;
      }
      eval += get_piece_value(piece_type, square_index);
    }
  }
  if bitboards[PieceType::WhiteBishop as usize].count_ones() >= 2 {
    eval += BISHOP_PAIR_VALUE;
  }
  if bitboards[PieceType::BlackBishop as usize].count_ones() >= 2 {
    eval -= BISHOP_PAIR_VALUE;
  }
  eval
}