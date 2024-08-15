use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub enum PieceType {
  WhiteKing,
  WhiteQueen,
  WhiteBishop,
  WhiteKnight,
  WhiteRook,
  WhitePawn,
  BlackKing,
  BlackQueen,
  BlackBishop,
  BlackKnight,
  BlackRook,
  BlackPawn
}
impl PieceType {
  pub fn iter() -> impl Iterator<Item = Self> {
    const VARIANTS: &[PieceType; 12] = &[
      PieceType::WhiteKing,
      PieceType::WhiteQueen,
      PieceType::WhiteBishop,
      PieceType::WhiteKnight,
      PieceType::WhiteRook,
      PieceType::WhitePawn,
      PieceType::BlackKing,
      PieceType::BlackQueen,
      PieceType::BlackBishop,
      PieceType::BlackKnight,
      PieceType::BlackRook,
      PieceType::BlackPawn
    ];
    VARIANTS.iter().copied()
  }

  pub fn get_colour_types(is_white: bool) -> [Self; 6] {
    return if is_white {
      [
        PieceType::WhiteKing,
        PieceType::WhiteQueen,
        PieceType::WhiteBishop,
        PieceType::WhiteKnight,
        PieceType::WhiteRook,
        PieceType::WhitePawn
      ]
    } else {
      [
        PieceType::BlackKing,
        PieceType::BlackQueen,
        PieceType::BlackBishop,
        PieceType::BlackKnight,
        PieceType::BlackRook,
        PieceType::BlackPawn
      ]
    }
  }
}
impl PartialEq for PieceType {
  fn eq(&self, other: &Self) -> bool {
  *self as usize == *other as usize
}
}

pub fn contains(rect: Rect, point: Vec2) -> bool {
  (point.x >= rect.x && point.x <= rect.x + rect.w) && (point.y >= rect.y && point.y <= rect.y + rect.h)
}