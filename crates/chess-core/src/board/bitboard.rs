use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(0xFFFFFFFFFFFFFFFF);

    pub const RANK_1: Bitboard = Bitboard(0x00000000000000FF);
    pub const RANK_2: Bitboard = Bitboard(0x000000000000FF00);
    pub const RANK_3: Bitboard = Bitboard(0x0000000000FF0000);
    pub const RANK_4: Bitboard = Bitboard(0x00000000FF000000);
    pub const RANK_5: Bitboard = Bitboard(0x000000FF00000000);
    pub const RANK_6: Bitboard = Bitboard(0x0000FF0000000000);
    pub const RANK_7: Bitboard = Bitboard(0x00FF000000000000);
    pub const RANK_8: Bitboard = Bitboard(0xFF00000000000000);

    pub const FILE_A: Bitboard = Bitboard(0x0101010101010101);
    pub const FILE_B: Bitboard = Bitboard(0x0202020202020202);
    pub const FILE_C: Bitboard = Bitboard(0x0404040404040404);
    pub const FILE_D: Bitboard = Bitboard(0x0808080808080808);
    pub const FILE_E: Bitboard = Bitboard(0x1010101010101010);
    pub const FILE_F: Bitboard = Bitboard(0x2020202020202020);
    pub const FILE_G: Bitboard = Bitboard(0x4040404040404040);
    pub const FILE_H: Bitboard = Bitboard(0x8080808080808080);

    pub const LIGHT_SQUARES: Bitboard = Bitboard(0x55AA55AA55AA55AA);
    pub const DARK_SQUARES: Bitboard = Bitboard(0xAA55AA55AA55AA55);

    #[inline(always)]
    pub const fn new(value: u64) -> Self {
        Bitboard(value)
    }

    #[inline(always)]
    pub const fn value(self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub const fn is_not_empty(self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    pub fn count_bits(self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn trailing_zeros(self) -> u32 {
        self.0.trailing_zeros()
    }

    #[inline]
    pub fn leading_zeros(self) -> u32 {
        self.0.leading_zeros()
    }

    #[inline]
    pub fn pop_lsb(&mut self) -> Option<u32> {
        if self.is_empty() {
            None
        } else {
            let lsb = self.trailing_zeros();
            self.0 &= self.0 - 1;
            Some(lsb)
        }
    }

    #[inline]
    pub fn lsb(self) -> Option<u32> {
        if self.is_empty() {
            None
        } else {
            Some(self.trailing_zeros())
        }
    }

    #[inline]
    pub fn msb(self) -> Option<u32> {
        if self.is_empty() {
            None
        } else {
            Some(63 - self.leading_zeros())
        }
    }

    #[inline]
    pub const fn shift_north(self) -> Bitboard {
        Bitboard(self.0 << 8)
    }

    #[inline]
    pub const fn shift_south(self) -> Bitboard {
        Bitboard(self.0 >> 8)
    }

    #[inline]
    pub const fn shift_east(self) -> Bitboard {
        Bitboard((self.0 << 1) & !Self::FILE_A.0)
    }

    #[inline]
    pub const fn shift_west(self) -> Bitboard {
        Bitboard((self.0 >> 1) & !Self::FILE_H.0)
    }

    #[inline]
    pub const fn shift_northeast(self) -> Bitboard {
        Bitboard((self.0 << 9) & !Self::FILE_A.0)
    }

    #[inline(always)]
    pub const fn shift_northwest(self) -> Bitboard {
        Bitboard((self.0 << 7) & !Self::FILE_H.0)
    }

    #[inline(always)]
    pub const fn shift_southeast(self) -> Bitboard {
        Bitboard((self.0 >> 7) & !Self::FILE_A.0)
    }

    #[inline(always)]
    pub const fn shift_southwest(self) -> Bitboard {
        Bitboard((self.0 >> 9) & !Self::FILE_H.0)
    }

    pub fn iter(self) -> BitboardIterator {
        BitboardIterator { bb: self }
    }

    pub fn squares(self) -> Vec<u32> {
        let mut squares = Vec::new();
        let mut bb = self;
        while let Some(square) = bb.pop_lsb() {
            squares.push(square);
        }
        squares
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  a b c d e f g h")?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = if (self.0 >> square) & 1 == 1 { '1' } else { '.' };
                write!(f, "{} ", bit)?;
            }
            writeln!(f, "{}", rank + 1)?;
        }
        writeln!(f, "  a b c d e f g h")?;
        writeln!(f, "Bitboard: 0x{:016X}", self.0)?;
        Ok(())
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

pub struct BitboardIterator {
    bb: Bitboard,
}

impl Iterator for BitboardIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.bb.pop_lsb()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_bitboard() {
        let bb = Bitboard::EMPTY;
        assert!(bb.is_empty());
        assert!(!bb.is_not_empty());
        assert_eq!(bb.count_bits(), 0);
    }

    #[test]
    fn test_full_bitboard() {
        let bb = Bitboard::FULL;
        assert!(!bb.is_empty());
        assert!(bb.is_not_empty());
        assert_eq!(bb.count_bits(), 64);
    }

    #[test]
    fn test_bit_operations() {
        let bb1 = Bitboard::new(0x0F);
        let bb2 = Bitboard::new(0xF0);

        assert_eq!(bb1 | bb2, Bitboard::new(0xFF));
        assert_eq!(bb1 & bb2, Bitboard::EMPTY);
        assert_eq!(bb1 ^ bb2, Bitboard::new(0xFF));
        assert_eq!(!bb1, Bitboard::new(!0x0F));
    }

    #[test]
    fn test_shifts() {
        let bb = Bitboard::new(1);
        assert_eq!(bb.shift_north(), Bitboard::new(1 << 8));
        assert_eq!(bb.shift_east(), Bitboard::new(1 << 1));

        let bb_a_file = Bitboard::new(1);
        assert_eq!(bb_a_file.shift_west(), Bitboard::EMPTY);

        let bb_h_file = Bitboard::new(1 << 7);
        assert_eq!(bb_h_file.shift_east(), Bitboard::EMPTY);
    }

    #[test]
    fn test_lsb_msb() {
        let bb = Bitboard::new(0b1010);
        assert_eq!(bb.lsb(), Some(1));
        assert_eq!(bb.msb(), Some(3));

        let empty = Bitboard::EMPTY;
        assert_eq!(empty.lsb(), None);
        assert_eq!(empty.msb(), None);
    }

    #[test]
    fn test_pop_lsb() {
        let mut bb = Bitboard::new(0b1010);
        assert_eq!(bb.pop_lsb(), Some(1));
        assert_eq!(bb, Bitboard::new(0b1000));
        assert_eq!(bb.pop_lsb(), Some(3));
        assert_eq!(bb, Bitboard::EMPTY);
        assert_eq!(bb.pop_lsb(), None);
    }

    #[test]
    fn test_iterator() {
        let bb = Bitboard::new(0b1010);
        let squares: Vec<u32> = bb.iter().collect();
        assert_eq!(squares, vec![1, 3]);
    }
}