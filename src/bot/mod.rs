mod evaluation;

use std::cmp;
use crate::board_representation::Board;
use crate::board_representation::Move;
use evaluation::*;

pub struct Bot {
  is_white_player: bool,
  best_move: Option<Move>
}
impl Bot {
  pub fn new(is_white: bool) -> Self {
    Self {
      is_white_player: is_white,
      best_move: None
    }
  }

  fn minimax(&mut self, board: Board, depth: i32, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> i32 { 
    let is_mate = board.is_checkmate();
    if depth == 0 || is_mate {
      return evaluate_position(board, is_mate, maximizing_player, depth);
    }
  
    let mut best_move: Option<Move> = None;
  
    if maximizing_player {
      let mut max_eval = NEGATIVE_INFINITY;
  
      for piece_move in board.get_all_moves() {
        let mut iteration_board = board.clone();
        iteration_board.make_move(piece_move);
  
        let eval = self.minimax(iteration_board, depth - 1, alpha, beta, false);
        if eval >= max_eval {
          max_eval = eval;
          best_move = Some(piece_move);
        }
  
        alpha = cmp::max(alpha, eval);
        if beta <= alpha {
          break;
        }
      }
    }
    else {
      let mut min_eval = INFINITY;
  
      for piece_move in board.get_all_moves() {
        let mut iteration_board = board.clone();
        iteration_board.make_move(piece_move);
  
        let eval = self.minimax(iteration_board, depth - 1, alpha, beta, true);
        if eval <= min_eval {
          min_eval = eval;
          best_move = Some(piece_move);
        }
  
        beta = cmp::min(beta, eval);
        if beta <= alpha {
          break;
        }
      }
    }

    self.best_move = best_move;
    return 0;
  }

  pub fn get_best_move(&mut self, board: Board) -> Move {
    self.minimax(board, STARTING_DEPTH, NEGATIVE_INFINITY, INFINITY, self.is_white_player);
    self.best_move.unwrap()
  }
}