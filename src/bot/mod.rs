mod evaluation;

use std::cmp;
use crate::board_representation::Board;
use crate::board_representation::Move;
use evaluation::*;

// names are slightly misleading, but they might as well be as they are as high as high can be (for 32 bit integers)
const INIFINITY: i32 = i32::MAX;
const NEGATIVE_INIFINITY: i32 = i32::MIN;

pub struct Bot {
  is_white_player: bool,
  best_move: Move
}
impl Bot {
  pub fn new(is_white: bool) -> Self {
    Self {
      is_white_player: is_white,
      best_move: Move::default()
    }
  }

  fn minimax(&mut self, board: Board, depth: i32, alpha: &mut i32, beta: &mut i32, maximizing_player: bool) -> i32 { 
    if depth == 0 || board.is_checkmate() {
      return evaluate_position(board);
    }
  
    if maximizing_player {
      let mut max_eval = NEGATIVE_INIFINITY;
  
      for piece_move in board.get_all_moves() {
        let mut itteration_board = board.clone();
        itteration_board.make_move(piece_move);
  
        let eval = self.minimax(itteration_board, depth - 1, alpha, beta, false);
        if eval >= max_eval {
          max_eval = eval;
          self.best_move = piece_move;
        }
  
        *alpha = cmp::max(*alpha, max_eval);
        if *beta <= *alpha {
          break;
        }
      }
  
      return max_eval;
    }
    else {
      let mut min_eval = INIFINITY;
  
      for piece_move in board.get_all_moves() {
        let mut itteration_board = board.clone();
        itteration_board.make_move(piece_move);
  
        let eval = self.minimax(itteration_board, depth - 1, alpha, beta, true);
        if eval <= min_eval {
          min_eval = eval;
          self.best_move = piece_move;
        }
  
        *beta = cmp::min(*beta, min_eval);
        if *beta <= *alpha {
          break;
        }
      }
  
      return min_eval;
    }
  }

  pub fn get_best_move(&mut self, board: Board) -> Move {
    let mut alpha = NEGATIVE_INIFINITY;
    let mut beta = INIFINITY;
    
    self.minimax(board, 3, &mut alpha, &mut beta, self.is_white_player); // the initial move passed in here doesnt matter
    self.best_move
  }
}