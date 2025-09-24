use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[inline]
    pub const fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub const fn index(self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 1,
        }
    }

    #[inline]
    pub const fn from_index(index: usize) -> Option<Color> {
        match index {
            0 => Some(Color::White),
            1 => Some(Color::Black),
            _ => None,
        }
    }

    #[inline]
    pub const fn is_white(self) -> bool {
        matches!(self, Color::White)
    }

    #[inline]
    pub const fn is_black(self) -> bool {
        matches!(self, Color::Black)
    }

    pub const fn pawn_start_rank(self) -> u8 {
        match self {
            Color::White => 1,
            Color::Black => 6,
        }
    }

    pub const fn pawn_promotion_rank(self) -> u8 {
        match self {
            Color::White => 7,
            Color::Black => 0,
        }
    }

    pub const fn back_rank(self) -> u8 {
        match self {
            Color::White => 0,
            Color::Black => 7,
        }
    }

    pub const fn forward_direction(self) -> i8 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}

impl std::str::FromStr for Color {
    type Err = crate::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "w" | "white" => Ok(Color::White),
            "b" | "black" => Ok(Color::Black),
            _ => Err(crate::ChessError::ParseError(format!(
                "Invalid color: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(Color::Black.opposite(), Color::White);
    }

    #[test]
    fn test_index() {
        assert_eq!(Color::White.index(), 0);
        assert_eq!(Color::Black.index(), 1);
        assert_eq!(Color::from_index(0), Some(Color::White));
        assert_eq!(Color::from_index(1), Some(Color::Black));
        assert_eq!(Color::from_index(2), None);
    }

    #[test]
    fn test_is_color() {
        assert!(Color::White.is_white());
        assert!(!Color::White.is_black());
        assert!(Color::Black.is_black());
        assert!(!Color::Black.is_white());
    }

    #[test]
    fn test_ranks() {
        assert_eq!(Color::White.pawn_start_rank(), 1);
        assert_eq!(Color::Black.pawn_start_rank(), 6);
        assert_eq!(Color::White.pawn_promotion_rank(), 7);
        assert_eq!(Color::Black.pawn_promotion_rank(), 0);
        assert_eq!(Color::White.back_rank(), 0);
        assert_eq!(Color::Black.back_rank(), 7);
    }

    #[test]
    fn test_forward_direction() {
        assert_eq!(Color::White.forward_direction(), 1);
        assert_eq!(Color::Black.forward_direction(), -1);
    }

    #[test]
    fn test_from_str() {
        assert_eq!("w".parse::<Color>().unwrap(), Color::White);
        assert_eq!("white".parse::<Color>().unwrap(), Color::White);
        assert_eq!("b".parse::<Color>().unwrap(), Color::Black);
        assert_eq!("black".parse::<Color>().unwrap(), Color::Black);
        assert!("invalid".parse::<Color>().is_err());
    }
}
