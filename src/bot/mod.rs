use crate::board_representation::Board;
use crate::board_representation::Move;

struct Bot {
  is_white_player: bool,
  board: Board,
}
impl Bot {
  pub fn new(is_white: bool, internal_board: Board) -> Self {
    Self {
      is_white_player: is_white,
      board: internal_board
    }
  }

  pub fn get_best_move(moves: Vec<Move>) -> Move {
    moves[0] // for now, return the first move
  }
}