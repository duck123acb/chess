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

  pub fn all_white() -> [Self; 6] {
    [
      PieceType::WhiteKing,
      PieceType::WhiteQueen,
      PieceType::WhiteBishop,
      PieceType::WhiteKnight,
      PieceType::WhiteRook,
      PieceType::WhitePawn
    ]
  }
  pub fn all_black() -> [Self; 6] {
    [
      PieceType::BlackKing,
      PieceType::BlackQueen,
      PieceType::BlackBishop,
      PieceType::BlackKnight,
      PieceType::BlackRook,
      PieceType::BlackPawn
    ]
  }
  pub fn all_sliding() -> [Self; 6] {
    [
      PieceType::WhiteQueen,
      PieceType::BlackQueen,
      PieceType::WhiteBishop,
      PieceType::BlackBishop,
      PieceType::WhiteRook,
      PieceType::BlackRook
    ]
  }
}
impl PartialEq for PieceType {
  fn eq(&self, other: &Self) -> bool {
  *self as usize == *other as usize
}
}

pub fn window_conf() -> Conf {
  Conf {
    window_title: "chess".to_string(),
    window_width: 800,
    window_height: 800,
    window_resizable: false,
    ..Default::default()
  }
}

pub fn contains(rect: Rect, point: Vec2) -> bool {
  (point.x >= rect.x && point.x <= rect.x + rect.w) && (point.y >= rect.y && point.y <= rect.y + rect.h)
}