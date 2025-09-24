use crate::Color;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub const ALL: [PieceType; 6] = [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ];

    #[inline]
    pub const fn index(self) -> usize {
        match self {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        }
    }

    #[inline]
    pub const fn from_index(index: usize) -> Option<PieceType> {
        match index {
            0 => Some(PieceType::Pawn),
            1 => Some(PieceType::Knight),
            2 => Some(PieceType::Bishop),
            3 => Some(PieceType::Rook),
            4 => Some(PieceType::Queen),
            5 => Some(PieceType::King),
            _ => None,
        }
    }

    pub const fn value(self) -> i32 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 20000,
        }
    }

    pub const fn is_sliding(self) -> bool {
        matches!(self, PieceType::Bishop | PieceType::Rook | PieceType::Queen)
    }

    pub const fn is_diagonal(self) -> bool {
        matches!(self, PieceType::Bishop | PieceType::Queen)
    }

    pub const fn is_orthogonal(self) -> bool {
        matches!(self, PieceType::Rook | PieceType::Queen)
    }

    pub const fn symbol(self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }

    pub const fn unicode_symbol(self, color: Color) -> char {
        match (self, color) {
            (PieceType::King, Color::White) => '♔',
            (PieceType::Queen, Color::White) => '♕',
            (PieceType::Rook, Color::White) => '♖',
            (PieceType::Bishop, Color::White) => '♗',
            (PieceType::Knight, Color::White) => '♘',
            (PieceType::Pawn, Color::White) => '♙',
            (PieceType::King, Color::Black) => '♚',
            (PieceType::Queen, Color::Black) => '♛',
            (PieceType::Rook, Color::Black) => '♜',
            (PieceType::Bishop, Color::Black) => '♝',
            (PieceType::Knight, Color::Black) => '♞',
            (PieceType::Pawn, Color::Black) => '♟',
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl std::str::FromStr for PieceType {
    type Err = crate::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().chars().next() {
            Some('P') => Ok(PieceType::Pawn),
            Some('N') => Ok(PieceType::Knight),
            Some('B') => Ok(PieceType::Bishop),
            Some('R') => Ok(PieceType::Rook),
            Some('Q') => Ok(PieceType::Queen),
            Some('K') => Ok(PieceType::King),
            _ => Err(crate::ChessError::ParseError(format!(
                "Invalid piece type: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    #[inline]
    pub const fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }

    #[inline]
    pub const fn white_pawn() -> Self {
        Piece::new(PieceType::Pawn, Color::White)
    }

    #[inline]
    pub const fn white_knight() -> Self {
        Piece::new(PieceType::Knight, Color::White)
    }

    #[inline]
    pub const fn white_bishop() -> Self {
        Piece::new(PieceType::Bishop, Color::White)
    }

    #[inline]
    pub const fn white_rook() -> Self {
        Piece::new(PieceType::Rook, Color::White)
    }

    #[inline]
    pub const fn white_queen() -> Self {
        Piece::new(PieceType::Queen, Color::White)
    }

    #[inline]
    pub const fn white_king() -> Self {
        Piece::new(PieceType::King, Color::White)
    }

    #[inline]
    pub const fn black_pawn() -> Self {
        Piece::new(PieceType::Pawn, Color::Black)
    }

    #[inline]
    pub const fn black_knight() -> Self {
        Piece::new(PieceType::Knight, Color::Black)
    }

    #[inline]
    pub const fn black_bishop() -> Self {
        Piece::new(PieceType::Bishop, Color::Black)
    }

    #[inline]
    pub const fn black_rook() -> Self {
        Piece::new(PieceType::Rook, Color::Black)
    }

    #[inline]
    pub const fn black_queen() -> Self {
        Piece::new(PieceType::Queen, Color::Black)
    }

    #[inline]
    pub const fn black_king() -> Self {
        Piece::new(PieceType::King, Color::Black)
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.piece_type.index() + self.color.index() * 6
    }

    #[inline]
    pub const fn from_index(index: usize) -> Option<Piece> {
        if index >= 12 {
            return None;
        }
        let color_index = index / 6;
        let piece_index = index % 6;

        match (
            Color::from_index(color_index),
            PieceType::from_index(piece_index),
        ) {
            (Some(color), Some(piece_type)) => Some(Piece::new(piece_type, color)),
            _ => None,
        }
    }

    pub const fn symbol(self) -> char {
        let base_symbol = self.piece_type.symbol();
        match self.color {
            Color::White => base_symbol,
            Color::Black => match base_symbol {
                'P' => 'p',
                'N' => 'n',
                'B' => 'b',
                'R' => 'r',
                'Q' => 'q',
                'K' => 'k',
                _ => base_symbol,
            },
        }
    }

    #[inline]
    pub const fn unicode_symbol(self) -> char {
        self.piece_type.unicode_symbol(self.color)
    }

    #[inline]
    pub const fn value(self) -> i32 {
        self.piece_type.value()
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl std::str::FromStr for Piece {
    type Err = crate::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(crate::ChessError::ParseError(format!(
                "Invalid piece: {}",
                s
            )));
        }

        let ch = s.chars().next().unwrap();
        let (piece_type, color) = match ch {
            'P' => (PieceType::Pawn, Color::White),
            'N' => (PieceType::Knight, Color::White),
            'B' => (PieceType::Bishop, Color::White),
            'R' => (PieceType::Rook, Color::White),
            'Q' => (PieceType::Queen, Color::White),
            'K' => (PieceType::King, Color::White),
            'p' => (PieceType::Pawn, Color::Black),
            'n' => (PieceType::Knight, Color::Black),
            'b' => (PieceType::Bishop, Color::Black),
            'r' => (PieceType::Rook, Color::Black),
            'q' => (PieceType::Queen, Color::Black),
            'k' => (PieceType::King, Color::Black),
            _ => {
                return Err(crate::ChessError::ParseError(format!(
                    "Invalid piece: {}",
                    s
                )))
            }
        };

        Ok(Piece::new(piece_type, color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_type_index() {
        assert_eq!(PieceType::Pawn.index(), 0);
        assert_eq!(PieceType::King.index(), 5);
        assert_eq!(PieceType::from_index(0), Some(PieceType::Pawn));
        assert_eq!(PieceType::from_index(5), Some(PieceType::King));
        assert_eq!(PieceType::from_index(6), None);
    }

    #[test]
    fn test_piece_type_value() {
        assert_eq!(PieceType::Pawn.value(), 100);
        assert_eq!(PieceType::Queen.value(), 900);
        assert_eq!(PieceType::King.value(), 20000);
    }

    #[test]
    fn test_piece_type_properties() {
        assert!(PieceType::Queen.is_sliding());
        assert!(PieceType::Queen.is_diagonal());
        assert!(PieceType::Queen.is_orthogonal());
        assert!(!PieceType::Knight.is_sliding());
        assert!(!PieceType::Knight.is_diagonal());
        assert!(!PieceType::Knight.is_orthogonal());
    }

    #[test]
    fn test_piece_creation() {
        let piece = Piece::new(PieceType::Queen, Color::White);
        assert_eq!(piece.piece_type, PieceType::Queen);
        assert_eq!(piece.color, Color::White);

        let white_queen = Piece::white_queen();
        assert_eq!(white_queen, piece);
    }

    #[test]
    fn test_piece_index() {
        let white_pawn = Piece::white_pawn();
        let black_king = Piece::black_king();

        assert_eq!(white_pawn.index(), 0);
        assert_eq!(black_king.index(), 11);
        assert_eq!(Piece::from_index(0), Some(white_pawn));
        assert_eq!(Piece::from_index(11), Some(black_king));
        assert_eq!(Piece::from_index(12), None);
    }

    #[test]
    fn test_piece_symbols() {
        assert_eq!(Piece::white_pawn().symbol(), 'P');
        assert_eq!(Piece::black_pawn().symbol(), 'p');
        assert_eq!(Piece::white_king().symbol(), 'K');
        assert_eq!(Piece::black_king().symbol(), 'k');
    }

    #[test]
    fn test_piece_from_str() {
        assert_eq!("P".parse::<Piece>().unwrap(), Piece::white_pawn());
        assert_eq!("p".parse::<Piece>().unwrap(), Piece::black_pawn());
        assert_eq!("K".parse::<Piece>().unwrap(), Piece::white_king());
        assert_eq!("k".parse::<Piece>().unwrap(), Piece::black_king());
        assert!("X".parse::<Piece>().is_err());
        assert!("PP".parse::<Piece>().is_err());
    }

    #[test]
    fn test_unicode_symbols() {
        assert_eq!(Piece::white_king().unicode_symbol(), '♔');
        assert_eq!(Piece::black_king().unicode_symbol(), '♚');
        assert_eq!(Piece::white_pawn().unicode_symbol(), '♙');
        assert_eq!(Piece::black_pawn().unicode_symbol(), '♟');
    }
}
