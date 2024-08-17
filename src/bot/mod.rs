use crate::board_representation::Board;
use crate::board_representation::Move;

// names are slightly misleading, but they might as well be as they are as high as high can be (for 32 bit integers)
const INIFINITY: i32 = i32::MAX;
const NEGATIVE_INIFINITY: i32 = i32::MIN;

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

fn minimax(board: &mut Board, move_to_search: Move, depth: i32, maximizing_player: bool) -> EvalMove { // thanks to Sebastian Lague!! https://www.youtube.com/watch?v=l-hh51ncgDI
  if depth == 0 || board.is_game_over() {
    return EvalMove::new(move_to_search, evaluate_position(board))
  }

  if maximizing_player {
    let mut max_eval = EvalMove::new(move_to_search, NEGATIVE_INIFINITY);

    for piece_move in board.get_all_moves() {
      board.make_move(piece_move);

      let eval_move = minimax(board, piece_move, depth - 1, false);
      let better_eval = if max_eval.eval < eval_move.eval {
        eval_move
      } else {
        max_eval
      };
      max_eval = better_eval;

      board.undo_move(piece_move);
    }

    return max_eval;
  }
  else {
    let mut min_eval = EvalMove::new(move_to_search, INIFINITY);

    for piece_move in board.get_all_moves() {
      board.make_move(piece_move);

      let eval_move = minimax(board, piece_move, depth - 1, true);
      let better_eval = if min_eval.eval > eval_move.eval {
        eval_move
      } else {
        min_eval
      };
      min_eval = better_eval;

      board.undo_move(piece_move);
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
    let best_move = minimax(board, moves[0], 3, self.is_white_player);
    best_move.board_move
    // moves[moves.len() - 1]
  }
}