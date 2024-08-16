use crate::board_representation::Board;
use crate::board_representation::Move;

pub struct Bot {
  is_white_player: bool
}
impl Bot {
  pub fn new(is_white: bool) -> Self {
    Self {
      is_white_player: is_white,
    }
  }

  pub fn get_best_move(&self, board: Board) -> Move {
    let moves = board.get_all_moves();
    moves[0] // for now, return the first move
  }
}