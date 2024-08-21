mod evaluation;

use std::cmp;
use crate::board_representation::Board;
use crate::board_representation::Move;
use evaluation::*;

pub struct Bot {
  is_white_player: bool
}
impl Bot {
  pub fn new(is_white: bool) -> Self {
    Self {
      is_white_player: is_white,
    }
  }

  fn minimax(&self, board: Board, depth: i32, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> (i32, Option<Move>) { 
    let is_mate = board.is_checkmate();
    if depth == 0 || is_mate {
      return (evaluate_position(board, is_mate, maximizing_player), None);
    }
  
    let mut best_move: Option<Move> = None;
  
    if maximizing_player {
      let mut max_eval = NEGATIVE_INFINITY;
  
      for piece_move in board.get_all_moves() {
        let mut iteration_board = board.clone();
        iteration_board.make_move(piece_move);
  
        let (eval, _) = self.minimax(iteration_board, depth - 1, alpha, beta, false);
        if eval >= max_eval {
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
      let mut min_eval = INFINITY;
  
      for piece_move in board.get_all_moves() {
        let mut iteration_board = board.clone();
        iteration_board.make_move(piece_move);
  
        let (eval, _) = self.minimax(iteration_board, depth - 1, alpha, beta, true);
        if eval <= min_eval {
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

  pub fn get_best_move(&mut self, board: Board) -> Move {
    let (_score, best_move) = self.minimax(board, 3, NEGATIVE_INFINITY, INFINITY, self.is_white_player);
    best_move.unwrap()
  }
}