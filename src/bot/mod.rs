mod evaluation;

use std::cmp;
use crate::board_representation::Board;
use crate::board_representation::Move;
use evaluation::*;

// names are slightly misleading, but they might as well be as they are as high as high can be (for 32 bit integers)
const INIFINITY: i32 = i32::MAX;
const NEGATIVE_INIFINITY: i32 = i32::MIN;

#[derive(Copy, Clone)]
struct EvalMove {
  pub board_move: Move,
  pub eval: i32
}
impl EvalMove {
  pub fn new(base_move: Move, value: i32) -> Self {
    Self {
      board_move: base_move,
      eval: value
    }
  }
}

fn minimax(board: &mut Board, move_to_search: Move, depth: i32, alpha: &mut i32, beta: &mut i32, maximizing_player: bool) -> EvalMove { 
  if depth == 0 || board.is_checkmate() {
    return EvalMove::new(move_to_search, evaluate_position(board));
  }

  if maximizing_player {
    let mut max_eval = EvalMove::new(move_to_search, NEGATIVE_INIFINITY);

    for piece_move in board.get_all_moves() {
      let mut itteration_board = board.clone();
      itteration_board.make_move(piece_move);

      let eval_move = minimax(&mut itteration_board, piece_move, depth - 1, alpha, beta, false);
      if eval_move.eval > max_eval.eval {
        max_eval = EvalMove::new(piece_move, eval_move.eval);
      }

      // board.undo_move(piece_move);

      *alpha = cmp::max(*alpha, max_eval.eval);
      if *beta <= *alpha {
        break;
      }
    }

    return max_eval;
  }
  else {
    let mut min_eval = EvalMove::new(move_to_search, INIFINITY);

    for piece_move in board.get_all_moves() {
      let mut itteration_board = board.clone();
      itteration_board.make_move(piece_move);

      let eval_move = minimax(&mut itteration_board, piece_move, depth - 1, alpha, beta, true);
      if eval_move.eval < min_eval.eval {
        min_eval = EvalMove::new(piece_move, eval_move.eval);
      }

      // board.undo_move(piece_move);
      
      *beta = cmp::min(*beta, min_eval.eval);
      if *beta <= *alpha {
        break;
      }
    }

    println!("{}, {}, {}", min_eval.board_move.start_square, min_eval.board_move.end_square, min_eval.eval);
    return min_eval;
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

  pub fn get_best_move(&self, board: &mut Board) -> Move {
    let moves = board.get_all_moves();
    let mut alpha = NEGATIVE_INIFINITY;
    let mut beta = INIFINITY;
    
    let best_move = minimax(board, moves[0], 1, &mut alpha, &mut beta, self.is_white_player);
    println!("{}, {}, {}", best_move.board_move.start_square, best_move.board_move.end_square, best_move.eval);
    best_move.board_move
  }
}