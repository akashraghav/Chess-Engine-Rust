use crate::Bitboard;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Square(u8);

impl Square {
    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);

    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);

    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);

    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);

    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);

    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);

    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);

    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);

    pub const ALL: [Square; 64] = [
        Square::A1,
        Square::B1,
        Square::C1,
        Square::D1,
        Square::E1,
        Square::F1,
        Square::G1,
        Square::H1,
        Square::A2,
        Square::B2,
        Square::C2,
        Square::D2,
        Square::E2,
        Square::F2,
        Square::G2,
        Square::H2,
        Square::A3,
        Square::B3,
        Square::C3,
        Square::D3,
        Square::E3,
        Square::F3,
        Square::G3,
        Square::H3,
        Square::A4,
        Square::B4,
        Square::C4,
        Square::D4,
        Square::E4,
        Square::F4,
        Square::G4,
        Square::H4,
        Square::A5,
        Square::B5,
        Square::C5,
        Square::D5,
        Square::E5,
        Square::F5,
        Square::G5,
        Square::H5,
        Square::A6,
        Square::B6,
        Square::C6,
        Square::D6,
        Square::E6,
        Square::F6,
        Square::G6,
        Square::H6,
        Square::A7,
        Square::B7,
        Square::C7,
        Square::D7,
        Square::E7,
        Square::F7,
        Square::G7,
        Square::H7,
        Square::A8,
        Square::B8,
        Square::C8,
        Square::D8,
        Square::E8,
        Square::F8,
        Square::G8,
        Square::H8,
    ];

    #[inline]
    pub const fn new(index: u8) -> Option<Square> {
        if index < 64 {
            Some(Square(index))
        } else {
            None
        }
    }

    #[inline]
    pub const fn from_file_rank(file: u8, rank: u8) -> Option<Square> {
        if file < 8 && rank < 8 {
            Some(Square(rank * 8 + file))
        } else {
            None
        }
    }

    #[inline]
    pub const fn index(self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn file(self) -> u8 {
        self.0 & 7
    }

    #[inline]
    pub const fn rank(self) -> u8 {
        self.0 >> 3
    }

    #[inline]
    pub const fn file_char(self) -> char {
        match self.file() {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => '?',
        }
    }

    #[inline]
    pub const fn rank_char(self) -> char {
        match self.rank() {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => '?',
        }
    }

    #[inline]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard::new(1u64 << self.0)
    }

    #[inline]
    pub const fn distance(self, other: Square) -> u8 {
        let file_diff = if self.file() > other.file() {
            self.file() - other.file()
        } else {
            other.file() - self.file()
        };
        let rank_diff = if self.rank() > other.rank() {
            self.rank() - other.rank()
        } else {
            other.rank() - self.rank()
        };
        if file_diff > rank_diff {
            file_diff
        } else {
            rank_diff
        }
    }

    #[inline]
    pub const fn manhattan_distance(self, other: Square) -> u8 {
        let file_diff = if self.file() > other.file() {
            self.file() - other.file()
        } else {
            other.file() - self.file()
        };
        let rank_diff = if self.rank() > other.rank() {
            self.rank() - other.rank()
        } else {
            other.rank() - self.rank()
        };
        file_diff + rank_diff
    }

    #[inline]
    pub const fn is_light(self) -> bool {
        (self.file() + self.rank()) & 1 == 1
    }

    #[inline]
    pub const fn is_dark(self) -> bool {
        (self.file() + self.rank()) & 1 == 0
    }

    pub const fn north(self) -> Option<Square> {
        if self.rank() < 7 {
            Some(Square(self.0 + 8))
        } else {
            None
        }
    }

    pub const fn south(self) -> Option<Square> {
        if self.rank() > 0 {
            Some(Square(self.0 - 8))
        } else {
            None
        }
    }

    pub const fn east(self) -> Option<Square> {
        if self.file() < 7 {
            Some(Square(self.0 + 1))
        } else {
            None
        }
    }

    pub const fn west(self) -> Option<Square> {
        if self.file() > 0 {
            Some(Square(self.0 - 1))
        } else {
            None
        }
    }

    pub const fn northeast(self) -> Option<Square> {
        if self.rank() < 7 && self.file() < 7 {
            Some(Square(self.0 + 9))
        } else {
            None
        }
    }

    pub const fn northwest(self) -> Option<Square> {
        if self.rank() < 7 && self.file() > 0 {
            Some(Square(self.0 + 7))
        } else {
            None
        }
    }

    pub const fn southeast(self) -> Option<Square> {
        if self.rank() > 0 && self.file() < 7 {
            Some(Square(self.0 - 7))
        } else {
            None
        }
    }

    pub const fn southwest(self) -> Option<Square> {
        if self.rank() > 0 && self.file() > 0 {
            Some(Square(self.0 - 9))
        } else {
            None
        }
    }

    pub fn knight_moves(self) -> impl Iterator<Item = Square> {
        const KNIGHT_DELTAS: [(i8, i8); 8] = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];

        let file = self.file() as i8;
        let rank = self.rank() as i8;

        KNIGHT_DELTAS.iter().filter_map(move |(df, dr)| {
            let new_file = file + df;
            let new_rank = rank + dr;
            if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
                Square::from_file_rank(new_file as u8, new_rank as u8)
            } else {
                None
            }
        })
    }

    pub fn king_moves(self) -> impl Iterator<Item = Square> {
        const KING_DELTAS: [(i8, i8); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let file = self.file() as i8;
        let rank = self.rank() as i8;

        KING_DELTAS.iter().filter_map(move |(df, dr)| {
            let new_file = file + df;
            let new_rank = rank + dr;
            if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
                Square::from_file_rank(new_file as u8, new_rank as u8)
            } else {
                None
            }
        })
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file_char(), self.rank_char())
    }
}

impl std::str::FromStr for Square {
    type Err = crate::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(crate::ChessError::ParseError(format!(
                "Invalid square: {}",
                s
            )));
        }

        let mut chars = s.chars();
        let file_char = chars.next().unwrap().to_ascii_lowercase();
        let rank_char = chars.next().unwrap();

        let file = match file_char {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {
                return Err(crate::ChessError::ParseError(format!(
                    "Invalid file: {}",
                    file_char
                )))
            }
        };

        let rank = match rank_char {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => {
                return Err(crate::ChessError::ParseError(format!(
                    "Invalid rank: {}",
                    rank_char
                )))
            }
        };

        Ok(Square::from_file_rank(file, rank).unwrap())
    }
}

impl From<u32> for Square {
    fn from(index: u32) -> Self {
        Square::new(index as u8).expect("Square index out of bounds")
    }
}

impl From<Square> for u32 {
    fn from(square: Square) -> Self {
        square.index() as u32
    }
}

impl From<Square> for usize {
    fn from(square: Square) -> Self {
        square.index() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_creation() {
        assert_eq!(Square::new(0), Some(Square::A1));
        assert_eq!(Square::new(63), Some(Square::H8));
        assert_eq!(Square::new(64), None);

        assert_eq!(Square::from_file_rank(0, 0), Some(Square::A1));
        assert_eq!(Square::from_file_rank(7, 7), Some(Square::H8));
        assert_eq!(Square::from_file_rank(8, 0), None);
        assert_eq!(Square::from_file_rank(0, 8), None);
    }

    #[test]
    fn test_square_properties() {
        assert_eq!(Square::A1.index(), 0);
        assert_eq!(Square::H8.index(), 63);

        assert_eq!(Square::A1.file(), 0);
        assert_eq!(Square::H8.file(), 7);
        assert_eq!(Square::A1.rank(), 0);
        assert_eq!(Square::H8.rank(), 7);

        assert_eq!(Square::A1.file_char(), 'a');
        assert_eq!(Square::H8.file_char(), 'h');
        assert_eq!(Square::A1.rank_char(), '1');
        assert_eq!(Square::H8.rank_char(), '8');
    }

    #[test]
    fn test_square_colors() {
        assert!(Square::A1.is_dark());
        assert!(!Square::A1.is_light());
        assert!(Square::A2.is_light());
        assert!(!Square::A2.is_dark());
    }

    #[test]
    fn test_square_distance() {
        assert_eq!(Square::A1.distance(Square::A1), 0);
        assert_eq!(Square::A1.distance(Square::H8), 7);
        assert_eq!(Square::A1.distance(Square::B2), 1);

        assert_eq!(Square::A1.manhattan_distance(Square::A1), 0);
        assert_eq!(Square::A1.manhattan_distance(Square::H8), 14);
        assert_eq!(Square::A1.manhattan_distance(Square::B2), 2);
    }

    #[test]
    fn test_square_directions() {
        assert_eq!(Square::E4.north(), Some(Square::E5));
        assert_eq!(Square::E4.south(), Some(Square::E3));
        assert_eq!(Square::E4.east(), Some(Square::F4));
        assert_eq!(Square::E4.west(), Some(Square::D4));

        assert_eq!(Square::E4.northeast(), Some(Square::F5));
        assert_eq!(Square::E4.northwest(), Some(Square::D5));
        assert_eq!(Square::E4.southeast(), Some(Square::F3));
        assert_eq!(Square::E4.southwest(), Some(Square::D3));

        assert_eq!(Square::H8.north(), None);
        assert_eq!(Square::A1.south(), None);
        assert_eq!(Square::H1.east(), None);
        assert_eq!(Square::A1.west(), None);
    }

    #[test]
    fn test_knight_moves() {
        let moves: Vec<Square> = Square::E4.knight_moves().collect();
        assert_eq!(moves.len(), 8);
        assert!(moves.contains(&Square::D2));
        assert!(moves.contains(&Square::F2));
        assert!(moves.contains(&Square::C3));
        assert!(moves.contains(&Square::G3));
        assert!(moves.contains(&Square::C5));
        assert!(moves.contains(&Square::G5));
        assert!(moves.contains(&Square::D6));
        assert!(moves.contains(&Square::F6));

        let corner_moves: Vec<Square> = Square::A1.knight_moves().collect();
        assert_eq!(corner_moves.len(), 2);
        assert!(corner_moves.contains(&Square::B3));
        assert!(corner_moves.contains(&Square::C2));
    }

    #[test]
    fn test_king_moves() {
        let moves: Vec<Square> = Square::E4.king_moves().collect();
        assert_eq!(moves.len(), 8);
        assert!(moves.contains(&Square::D3));
        assert!(moves.contains(&Square::E3));
        assert!(moves.contains(&Square::F3));
        assert!(moves.contains(&Square::D4));
        assert!(moves.contains(&Square::F4));
        assert!(moves.contains(&Square::D5));
        assert!(moves.contains(&Square::E5));
        assert!(moves.contains(&Square::F5));

        let corner_moves: Vec<Square> = Square::A1.king_moves().collect();
        assert_eq!(corner_moves.len(), 3);
        assert!(corner_moves.contains(&Square::A2));
        assert!(corner_moves.contains(&Square::B1));
        assert!(corner_moves.contains(&Square::B2));
    }

    #[test]
    fn test_square_from_str() {
        assert_eq!("a1".parse::<Square>().unwrap(), Square::A1);
        assert_eq!("h8".parse::<Square>().unwrap(), Square::H8);
        assert_eq!("e4".parse::<Square>().unwrap(), Square::E4);
        assert!("i1".parse::<Square>().is_err());
        assert!("a9".parse::<Square>().is_err());
        assert!("a".parse::<Square>().is_err());
        assert!("a11".parse::<Square>().is_err());
    }

    #[test]
    fn test_square_bitboard() {
        assert_eq!(Square::A1.bitboard(), Bitboard::new(1));
        assert_eq!(Square::H8.bitboard(), Bitboard::new(1u64 << 63));
    }

    #[test]
    fn test_square_display() {
        assert_eq!(format!("{}", Square::A1), "a1");
        assert_eq!(format!("{}", Square::H8), "h8");
        assert_eq!(format!("{}", Square::E4), "e4");
    }
}
