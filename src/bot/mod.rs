use std::cmp;

use crate::board_representation::Board;
use crate::board_representation::Move;

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

fn evaluate_position(board: &Board) -> i32 {
  0 // lmaooooo best evaluation
}

fn minimax(board: &mut Board, move_to_search: Move, depth: i32, alpha: &mut i32, beta: &mut i32, maximizing_player: bool) -> EvalMove { // thanks to Sebastian Lague!! https://www.youtube.com/watch?v=l-hh51ncgDI
  if depth == 0 || board.is_game_over() {
    return EvalMove::new(move_to_search, evaluate_position(board))
  }

  if maximizing_player {
    let mut max_eval = EvalMove::new(move_to_search, NEGATIVE_INIFINITY);

    for piece_move in board.get_all_moves() {
      board.make_move(piece_move);

      let eval_move = minimax(board, piece_move, depth - 1, alpha, beta, false);
      let better_eval = if max_eval.eval <= eval_move.eval {
        eval_move
      } else {
        max_eval
      };
      max_eval = better_eval;

      board.undo_move(piece_move);

      *alpha = cmp::max(*alpha, eval_move.eval);
      if beta <= alpha {
        break;
      }
    }

    return max_eval;
  }
  else {
    let mut min_eval = EvalMove::new(move_to_search, INIFINITY);

    for piece_move in board.get_all_moves() {
      board.make_move(piece_move);

      let eval_move = minimax(board, piece_move, depth - 1, alpha, beta, true);
      let better_eval = if min_eval.eval >= eval_move.eval {
        eval_move
      } else {
        min_eval
      };
      min_eval = better_eval;

      board.undo_move(piece_move);
      
      *beta = cmp::min(*beta, eval_move.eval);
      if beta <= alpha {
        break;
      }
    }
    
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
    let best_move = minimax(board, moves[0], 3, &mut NEGATIVE_INIFINITY, &mut INIFINITY, self.is_white_player);
    // println!("{:b}", (1u64 << best_move.board_move.start_square) | (1u64 << best_move.board_move.end_square));
    best_move.board_move
    // moves[moves.len() - 1]
  }
}