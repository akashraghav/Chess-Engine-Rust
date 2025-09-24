use crate::{Piece, Square};
use std::collections::HashMap;

pub struct ZobristHash {
    piece_keys: [[u64; 64]; 12],
    side_key: u64,
    castling_keys: [u64; 16],
    en_passant_keys: [u64; 8],
}

impl ZobristHash {
    pub fn new() -> Self {
        let mut zobrist = ZobristHash {
            piece_keys: [[0; 64]; 12],
            side_key: 0,
            castling_keys: [0; 16],
            en_passant_keys: [0; 8],
        };
        zobrist.initialize();
        zobrist
    }

    fn initialize(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        for piece_idx in 0..12 {
            for square_idx in 0..64 {
                format!("piece_{}_{}", piece_idx, square_idx).hash(&mut hasher);
                self.piece_keys[piece_idx][square_idx] = hasher.finish();
                hasher = DefaultHasher::new();
            }
        }

        "side_to_move".hash(&mut hasher);
        self.side_key = hasher.finish();
        hasher = DefaultHasher::new();

        for castle_idx in 0..16 {
            format!("castling_{}", castle_idx).hash(&mut hasher);
            self.castling_keys[castle_idx] = hasher.finish();
            hasher = DefaultHasher::new();
        }

        for file in 0..8 {
            format!("en_passant_{}", file).hash(&mut hasher);
            self.en_passant_keys[file] = hasher.finish();
            hasher = DefaultHasher::new();
        }
    }

    pub fn hash_piece(&self, piece: Piece, square: Square) -> u64 {
        self.piece_keys[piece.index()][square.index() as usize]
    }

    pub fn hash_side(&self) -> u64 {
        self.side_key
    }

    pub fn hash_castling(&self, castling_rights: u8) -> u64 {
        self.castling_keys[castling_rights as usize & 15]
    }

    pub fn hash_en_passant(&self, file: u8) -> u64 {
        if file < 8 {
            self.en_passant_keys[file as usize]
        } else {
            0
        }
    }
}

impl Default for ZobristHash {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct TranspositionTable {
    table: HashMap<u64, TranspositionEntry>,
    max_size: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct TranspositionEntry {
    pub hash: u64,
    pub depth: u8,
    pub score: i32,
    pub node_type: NodeType,
    pub best_move: Option<crate::Move>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Exact,
    Alpha,
    Beta,
}

impl TranspositionTable {
    pub fn new(max_size: usize) -> Self {
        TranspositionTable {
            table: HashMap::new(),
            max_size,
        }
    }

    pub fn get(&self, hash: u64) -> Option<&TranspositionEntry> {
        self.table.get(&hash)
    }

    pub fn store(&mut self, entry: TranspositionEntry) {
        if self.table.len() >= self.max_size {
            if let Some((&oldest_hash, _)) = self.table.iter().next() {
                self.table.remove(&oldest_hash);
            }
        }
        self.table.insert(entry.hash, entry);
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }

    pub fn size(&self) -> usize {
        self.table.len()
    }

    pub fn is_full(&self) -> bool {
        self.table.len() >= self.max_size
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new(1_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Piece, Square};

    #[test]
    fn test_zobrist_hash_creation() {
        let zobrist = ZobristHash::new();
        let piece_hash = zobrist.hash_piece(Piece::white_pawn(), Square::E4);
        assert_ne!(piece_hash, 0);
    }

    #[test]
    fn test_zobrist_hash_different_pieces() {
        let zobrist = ZobristHash::new();
        let pawn_hash = zobrist.hash_piece(Piece::white_pawn(), Square::E4);
        let knight_hash = zobrist.hash_piece(Piece::white_knight(), Square::E4);
        assert_ne!(pawn_hash, knight_hash);
    }

    #[test]
    fn test_zobrist_hash_different_squares() {
        let zobrist = ZobristHash::new();
        let e4_hash = zobrist.hash_piece(Piece::white_pawn(), Square::E4);
        let e5_hash = zobrist.hash_piece(Piece::white_pawn(), Square::E5);
        assert_ne!(e4_hash, e5_hash);
    }

    #[test]
    fn test_transposition_table() {
        let mut tt = TranspositionTable::new(100);
        let entry = TranspositionEntry {
            hash: 12345,
            depth: 5,
            score: 100,
            node_type: NodeType::Exact,
            best_move: None,
        };

        tt.store(entry);
        assert_eq!(tt.size(), 1);

        let retrieved = tt.get(12345);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().score, 100);
    }

    #[test]
    fn test_transposition_table_overflow() {
        let mut tt = TranspositionTable::new(2);

        for i in 0..5 {
            let entry = TranspositionEntry {
                hash: i,
                depth: 1,
                score: i as i32,
                node_type: NodeType::Exact,
                best_move: None,
            };
            tt.store(entry);
        }

        assert_eq!(tt.size(), 2);
    }
}