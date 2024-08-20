use crate::board_representation::Board;
use crate::utils::PieceType;

const PAWN_VALUE: i32 = 1;
const KNIGHT_VALUE: i32 = 3;
const BISHOP_VALUE: i32 = 3;
const ROOK_VALUE: i32 = 5;
const QUEEN_VALUE: i32 = 9;

fn get_piece_value(piece_type: PieceType) -> i32 {
  match piece_type {
    PieceType::WhiteQueen => {
      QUEEN_VALUE
    },
    PieceType::BlackQueen => {
      -QUEEN_VALUE
    },
    PieceType::WhiteBishop => {
      BISHOP_VALUE
    },
    PieceType::BlackBishop => {
      -BISHOP_VALUE
    },
    PieceType::WhiteKnight => {
      KNIGHT_VALUE
    },
    PieceType::BlackKnight => {
      -KNIGHT_VALUE
    },
    PieceType::WhiteRook => {
      ROOK_VALUE
    },
    PieceType::BlackRook => {
      -ROOK_VALUE
    },
    PieceType::WhitePawn=> {
      PAWN_VALUE
    },
    PieceType::BlackPawn => {
      -PAWN_VALUE
    },
    _ => {
      0
    }
  }
}

pub fn evaluate_position(board: Board) -> i32 {
  let mut eval = 0;

  for piece_type in PieceType::iter() {
    let bitboard = board.get_bitboards()[piece_type as usize];
    eval += bitboard.count_ones() as i32 * get_piece_value(piece_type);
  }

  eval
}