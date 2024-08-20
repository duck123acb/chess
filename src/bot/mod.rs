mod evaluation;

use std::cmp;
use crate::board_representation::Board;
use crate::board_representation::Move;
use evaluation::*;

// names are slightly misleading, but they might as well be as they are as high as high can be (for 32 bit integers)
const INIFINITY: i32 = i32::MAX;
const NEGATIVE_INIFINITY: i32 = i32::MIN;

fn minimax(board: Board, depth: i32, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> (i32, Option<Move>) { 
  if depth == 0 || board.is_checkmate() {
    return (evaluate_position(board), None);
  }

  let mut best_move: Option<Move> = None;

  if maximizing_player {
    let mut max_eval = NEGATIVE_INIFINITY;

    for piece_move in board.get_all_moves() {
      let mut iteration_board = board.clone();
      iteration_board.make_move(piece_move);

      let (eval, _) = minimax(iteration_board, depth - 1, alpha, beta, false);
      if eval > max_eval {
        max_eval = eval;
        best_move = Some(piece_move);
      }

      alpha = cmp::max(alpha, eval);
      if beta <= alpha {
        break;
      }
    }

    return (max_eval, best_move);
  }
  else {
    let mut min_eval = INIFINITY;

    for piece_move in board.get_all_moves() {
      let mut iteration_board = board.clone();
      iteration_board.make_move(piece_move);

      let (eval, _) = minimax(iteration_board, depth - 1, alpha, beta, true);
      if eval < min_eval {
        min_eval = eval;
        best_move = Some(piece_move);
      }

      beta = cmp::min(beta, eval);
      if beta <= alpha {
        break;
      }
    }

    return (min_eval, best_move);
  }
}


pub struct Bot {
  is_white_player: bool
}
impl Bot {
  pub fn new(is_white: bool) -> Self {
    Self {
      is_white_player: is_white,
    }
  }

  pub fn get_best_move(&mut self, board: Board) -> Move {
    let (_score, best_move) = minimax(board, 3, NEGATIVE_INIFINITY, INIFINITY, self.is_white_player);
    best_move.unwrap()
  }
}