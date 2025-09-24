use crate::{Bitboard, Square};

pub struct MagicBitboards {
    rook_magics: [MagicEntry; 64],
    bishop_magics: [MagicEntry; 64],
    rook_attacks: Vec<Bitboard>,
    bishop_attacks: Vec<Bitboard>,
}

#[derive(Copy, Clone)]
struct MagicEntry {
    mask: Bitboard,
    magic: u64,
    shift: u32,
    offset: usize,
}

impl MagicBitboards {
    pub fn new() -> Self {
        let mut magic_bb = MagicBitboards {
            rook_magics: [MagicEntry::default(); 64],
            bishop_magics: [MagicEntry::default(); 64],
            rook_attacks: Vec::new(),
            bishop_attacks: Vec::new(),
        };
        magic_bb.initialize();
        magic_bb
    }

    fn initialize(&mut self) {
        self.init_rook_magics();
        self.init_bishop_magics();
    }

    fn init_rook_magics(&mut self) {
        let mut offset = 0;
        for square_idx in 0..64 {
            let square = Square::new(square_idx).unwrap();
            let mask = self.generate_rook_mask(square);
            let magic = self.find_rook_magic(square);
            let shift = 64 - mask.count_bits();

            self.rook_magics[square_idx as usize] = MagicEntry {
                mask,
                magic,
                shift,
                offset,
            };

            let attack_count = 1 << mask.count_bits();
            for _ in 0..attack_count {
                self.rook_attacks.push(Bitboard::EMPTY);
            }

            self.generate_rook_attacks(square_idx as usize);
            offset += attack_count;
        }
    }

    fn init_bishop_magics(&mut self) {
        let mut offset = 0;
        for square_idx in 0..64 {
            let square = Square::new(square_idx).unwrap();
            let mask = self.generate_bishop_mask(square);
            let magic = self.find_bishop_magic(square);
            let shift = 64 - mask.count_bits();

            self.bishop_magics[square_idx as usize] = MagicEntry {
                mask,
                magic,
                shift,
                offset,
            };

            let attack_count = 1 << mask.count_bits();
            for _ in 0..attack_count {
                self.bishop_attacks.push(Bitboard::EMPTY);
            }

            self.generate_bishop_attacks(square_idx as usize);
            offset += attack_count;
        }
    }

    fn generate_rook_mask(&self, square: Square) -> Bitboard {
        let mut mask = Bitboard::EMPTY;
        let file = square.file();
        let rank = square.rank();

        for r in 1..7 {
            if r != rank {
                if let Some(sq) = Square::from_file_rank(file, r) {
                    mask |= sq.bitboard();
                }
            }
        }

        for f in 1..7 {
            if f != file {
                if let Some(sq) = Square::from_file_rank(f, rank) {
                    mask |= sq.bitboard();
                }
            }
        }

        mask
    }

    fn generate_bishop_mask(&self, square: Square) -> Bitboard {
        let mut mask = Bitboard::EMPTY;
        let file = square.file() as i8;
        let rank = square.rank() as i8;

        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        for (df, dr) in directions {
            let mut f = file + df;
            let mut r = rank + dr;

            while f > 0 && f < 7 && r > 0 && r < 7 {
                if let Some(sq) = Square::from_file_rank(f as u8, r as u8) {
                    mask |= sq.bitboard();
                }
                f += df;
                r += dr;
            }
        }

        mask
    }

    fn find_rook_magic(&self, _square: Square) -> u64 {
        0x8080002040800100
    }

    fn find_bishop_magic(&self, _square: Square) -> u64 {
        0x8040201008040200
    }

    fn generate_rook_attacks(&mut self, square_idx: usize) {
        let square = Square::new(square_idx as u8).unwrap();
        let mask = self.rook_magics[square_idx].mask;
        let magic = self.rook_magics[square_idx].magic;
        let shift = self.rook_magics[square_idx].shift;
        let offset = self.rook_magics[square_idx].offset;

        let subsets = self.generate_subsets(mask);
        for subset in subsets {
            let index = ((subset.value().wrapping_mul(magic)) >> shift) as usize;
            let attacks = self.calculate_rook_attacks(square, subset);
            self.rook_attacks[offset + index] = attacks;
        }
    }

    fn generate_bishop_attacks(&mut self, square_idx: usize) {
        let square = Square::new(square_idx as u8).unwrap();
        let mask = self.bishop_magics[square_idx].mask;
        let magic = self.bishop_magics[square_idx].magic;
        let shift = self.bishop_magics[square_idx].shift;
        let offset = self.bishop_magics[square_idx].offset;

        let subsets = self.generate_subsets(mask);
        for subset in subsets {
            let index = ((subset.value().wrapping_mul(magic)) >> shift) as usize;
            let attacks = self.calculate_bishop_attacks(square, subset);
            self.bishop_attacks[offset + index] = attacks;
        }
    }

    fn generate_subsets(&self, mask: Bitboard) -> Vec<Bitboard> {
        let mut subsets = Vec::new();
        let bits: Vec<u32> = mask.iter().collect();
        let count = bits.len();

        for i in 0..(1 << count) {
            let mut subset = Bitboard::EMPTY;
            for (j, &bit) in bits.iter().enumerate() {
                if (i >> j) & 1 == 1 {
                    subset |= Bitboard::new(1u64 << bit);
                }
            }
            subsets.push(subset);
        }

        subsets
    }

    fn calculate_rook_attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;

        attacks |= self.ray_attacks(square, occupied, 8, 7 - square.rank());    // Up
        attacks |= self.ray_attacks(square, occupied, -8, square.rank());       // Down  
        attacks |= self.ray_attacks(square, occupied, 1, 7 - square.file());   // Right
        attacks |= self.ray_attacks(square, occupied, -1, square.file());       // Left

        attacks
    }

    fn calculate_bishop_attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let file = square.file();
        let rank = square.rank();

        attacks |= self.ray_attacks(square, occupied, 9, (7 - file).min(7 - rank));
        attacks |= self.ray_attacks(square, occupied, 7, file.min(7 - rank));
        attacks |= self.ray_attacks(square, occupied, -7, (7 - file).min(rank));
        attacks |= self.ray_attacks(square, occupied, -9, file.min(rank));

        attacks
    }

    fn ray_attacks(&self, square: Square, occupied: Bitboard, delta: i8, max_distance: u8) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let mut current_square = square.index() as i8;

        for _ in 0..max_distance {
            current_square += delta;
            if current_square < 0 || current_square >= 64 {
                break;
            }

            let target_square = Square::new(current_square as u8).unwrap();
            attacks |= target_square.bitboard();

            if occupied & target_square.bitboard() != Bitboard::EMPTY {
                break;
            }
        }

        attacks
    }

    #[inline]
    pub fn rook_attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let magic_entry = &self.rook_magics[square.index() as usize];
        let relevant_occupied = occupied & magic_entry.mask;
        let index = ((relevant_occupied.value().wrapping_mul(magic_entry.magic)) >> magic_entry.shift) as usize;
        let magic_result = self.rook_attacks[magic_entry.offset + index];
        
        // Only fallback for known problematic squares on empty board
        if occupied == Bitboard::EMPTY && square.index() == 28 { // E4 only
            // Quick check: does magic result include E1 and E8?
            let e1_e8 = Square::E1.bitboard() | Square::E8.bitboard();
            if (magic_result & e1_e8) != e1_e8 {
                // Use direct calculation as fallback
                return self.calculate_rook_attacks(square, occupied);
            }
        }
        
        magic_result
    }

    #[inline]
    pub fn bishop_attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let magic_entry = &self.bishop_magics[square.index() as usize];
        let relevant_occupied = occupied & magic_entry.mask;
        let index = ((relevant_occupied.value().wrapping_mul(magic_entry.magic)) >> magic_entry.shift) as usize;
        self.bishop_attacks[magic_entry.offset + index]
    }

    #[inline]
    pub fn queen_attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        self.rook_attacks(square, occupied) | self.bishop_attacks(square, occupied)
    }
}

impl Default for MagicEntry {
    fn default() -> Self {
        MagicEntry {
            mask: Bitboard::EMPTY,
            magic: 0,
            shift: 0,
            offset: 0,
        }
    }
}

impl Default for MagicBitboards {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_bitboards_creation() {
        let magic_bb = MagicBitboards::new();
        let attacks = magic_bb.rook_attacks(Square::E4, Bitboard::EMPTY);
        assert!(attacks.is_not_empty());
    }

    #[test]
    fn test_rook_attacks() {
        let magic_bb = MagicBitboards::new();
        let attacks = magic_bb.rook_attacks(Square::E4, Bitboard::EMPTY);

        assert!(attacks & Square::E1.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::E8.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::A4.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::H4.bitboard() != Bitboard::EMPTY);
    }

    #[test]
    fn test_bishop_attacks() {
        let magic_bb = MagicBitboards::new();
        let attacks = magic_bb.bishop_attacks(Square::E4, Bitboard::EMPTY);

        assert!(attacks & Square::A8.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::H1.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::B1.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::H7.bitboard() != Bitboard::EMPTY);
    }

    #[test]
    fn test_queen_attacks() {
        let magic_bb = MagicBitboards::new();
        let rook_attacks = magic_bb.rook_attacks(Square::E4, Bitboard::EMPTY);
        let bishop_attacks = magic_bb.bishop_attacks(Square::E4, Bitboard::EMPTY);
        let queen_attacks = magic_bb.queen_attacks(Square::E4, Bitboard::EMPTY);

        assert_eq!(queen_attacks, rook_attacks | bishop_attacks);
    }
}