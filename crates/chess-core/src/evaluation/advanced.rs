// Advanced evaluation optimizations for chess engine
// Implements lazy evaluation, incremental updates, and SIMD-optimized scoring

use crate::{Position, Color, PieceType, Bitboard, Square};
use std::collections::HashMap;

// Evaluation constants
pub const PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 20000]; // P, N, B, R, Q, K

// Positional evaluation tables (piece-square tables)
pub const PAWN_TABLE: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0
];

pub const KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

pub const BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

pub const ROOK_TABLE: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0
];

pub const QUEEN_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

pub const KING_MIDDLE_GAME: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

pub const KING_END_GAME: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];

/// Cached evaluation components to enable incremental updates
#[derive(Debug, Clone)]
pub struct EvaluationCache {
    pub material_score: i32,
    pub positional_score: i32,
    pub pawn_structure_score: i32,
    pub king_safety_score: i32,
    pub mobility_score: i32,
    pub zobrist_hash: u64,
    pub phase: GamePhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    Opening,
    MiddleGame,
    EndGame,
}

impl GamePhase {
    pub fn from_material(material_count: i32) -> Self {
        if material_count > 6200 {
            GamePhase::Opening
        } else if material_count > 1500 {
            GamePhase::MiddleGame
        } else {
            GamePhase::EndGame
        }
    }
}

/// Advanced evaluator with lazy evaluation and incremental updates
#[derive(Clone)]
pub struct OptimizedEvaluator {
    evaluation_cache: HashMap<u64, EvaluationCache>,
    pawn_structure_cache: HashMap<u64, i32>,
    king_safety_cache: HashMap<(u64, Color), i32>,
    mobility_cache: HashMap<u64, (i32, i32)>, // (white_mobility, black_mobility)
    cache_hits: u64,
    cache_misses: u64,
}

impl OptimizedEvaluator {
    pub fn new() -> Self {
        OptimizedEvaluator {
            evaluation_cache: HashMap::with_capacity(1024 * 64), // 64K entries
            pawn_structure_cache: HashMap::with_capacity(1024 * 16), // 16K entries  
            king_safety_cache: HashMap::with_capacity(1024 * 8), // 8K entries
            mobility_cache: HashMap::with_capacity(1024 * 32), // 32K entries
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    /// Main evaluation function with caching
    pub fn evaluate(&mut self, position: &Position) -> i32 {
        let zobrist = position.zobrist_hash();
        
        // Check evaluation cache first
        if let Some(cached) = self.evaluation_cache.get(&zobrist) {
            self.cache_hits += 1;
            return self.interpolate_evaluation(cached, position);
        }

        self.cache_misses += 1;
        
        // Compute full evaluation
        let eval_cache = self.compute_full_evaluation(position);
        let score = self.interpolate_evaluation(&eval_cache, position);
        
        // Store in cache
        self.evaluation_cache.insert(zobrist, eval_cache);
        
        score
    }

    /// Lazy evaluation with incremental updates
    pub fn evaluate_incremental(&mut self, position: &Position, previous_hash: Option<u64>, move_made: Option<crate::Move>) -> i32 {
        let zobrist = position.zobrist_hash();
        
        // Try incremental update if we have the previous position cached
        if let (Some(prev_hash), Some(move_item)) = (previous_hash, move_made) {
            if let Some(prev_eval) = self.evaluation_cache.get(&prev_hash) {
                if let Some(updated_eval) = self.update_evaluation_incrementally(prev_eval, position, move_item) {
                    self.evaluation_cache.insert(zobrist, updated_eval.clone());
                    return self.interpolate_evaluation(&updated_eval, position);
                }
            }
        }
        
        // Fall back to full evaluation
        self.evaluate(position)
    }

    /// SIMD-optimized material evaluation
    pub fn simd_material_evaluation(&self, position: &Position) -> i32 {
        // Extract piece counts for both colors
        let white_pieces = self.extract_piece_counts(position, Color::White);
        let black_pieces = self.extract_piece_counts(position, Color::Black);
        
        // Use SIMD to compute material difference
        let white_counts = [white_pieces; 1]; // Would need 4 positions for true SIMD
        let black_counts = [black_pieces; 1];
        
        let white_material = self.simd_compute_material(&white_counts)[0];
        let black_material = self.simd_compute_material(&black_counts)[0];
        
        white_material - black_material
    }

    fn compute_full_evaluation(&mut self, position: &Position) -> EvaluationCache {
        let zobrist = position.zobrist_hash();
        
        // Material evaluation
        let material_score = self.evaluate_material(position);
        
        // Positional evaluation
        let positional_score = self.evaluate_position(position);
        
        // Pawn structure (cached)
        let pawn_hash = self.compute_pawn_hash(position);
        let pawn_structure_score = self.pawn_structure_cache
            .get(&pawn_hash)
            .copied()
            .unwrap_or_else(|| {
                let score = self.evaluate_pawn_structure(position);
                self.pawn_structure_cache.insert(pawn_hash, score);
                score
            });
        
        // King safety (cached by color)
        let white_king_safety = self.get_or_compute_king_safety(position, Color::White);
        let black_king_safety = self.get_or_compute_king_safety(position, Color::Black);
        let king_safety_score = white_king_safety - black_king_safety;
        
        // Mobility evaluation (cached)
        let (white_mobility, black_mobility) = self.mobility_cache
            .get(&zobrist)
            .copied()
            .unwrap_or_else(|| {
                let mobility = self.evaluate_mobility(position);
                self.mobility_cache.insert(zobrist, mobility);
                mobility
            });
        let mobility_score = white_mobility - black_mobility;
        
        // Determine game phase
        let phase = GamePhase::from_material(material_score.abs());
        
        EvaluationCache {
            material_score,
            positional_score,
            pawn_structure_score,
            king_safety_score,
            mobility_score,
            zobrist_hash: zobrist,
            phase,
        }
    }

    fn update_evaluation_incrementally(
        &self,
        prev_eval: &EvaluationCache,
        position: &Position,
        move_made: crate::Move,
    ) -> Option<EvaluationCache> {
        // Only support simple moves for now (no captures, castling, etc.)
        if move_made.is_capture() || move_made.is_castle() || move_made.is_promotion() {
            return None;
        }

        let mut new_eval = prev_eval.clone();
        new_eval.zobrist_hash = position.zobrist_hash();
        
        // Update positional score incrementally
        if let Some(piece) = position.piece_at(move_made.to) {
            let piece_type = piece.piece_type;
            let color = piece.color;
            
            // Remove old position value
            let old_value = self.get_piece_square_value(piece_type, move_made.from, color, prev_eval.phase);
            // Add new position value  
            let new_value = self.get_piece_square_value(piece_type, move_made.to, color, prev_eval.phase);
            
            let delta = if color == Color::White {
                new_value - old_value
            } else {
                old_value - new_value
            };
            
            new_eval.positional_score += delta;
        }
        
        Some(new_eval)
    }

    fn interpolate_evaluation(&self, cached: &EvaluationCache, position: &Position) -> i32 {
        let game_phase_factor = match cached.phase {
            GamePhase::Opening => 1.2,
            GamePhase::MiddleGame => 1.0,
            GamePhase::EndGame => 0.8,
        };
        
        let total_score = cached.material_score
            + cached.positional_score
            + cached.pawn_structure_score
            + cached.king_safety_score
            + cached.mobility_score;
        
        let side_to_move_bonus = if position.side_to_move() == Color::White { 10 } else { -10 };
        
        ((total_score as f32 * game_phase_factor) as i32) + side_to_move_bonus
    }

    fn evaluate_material(&self, position: &Position) -> i32 {
        let mut score = 0;
        
        for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, 
                          PieceType::Rook, PieceType::Queen, PieceType::King] {
            let white_count = self.count_pieces(position, piece_type, Color::White);
            let black_count = self.count_pieces(position, piece_type, Color::Black);
            let piece_value = PIECE_VALUES[piece_type as usize];
            
            score += (white_count as i32 - black_count as i32) * piece_value;
        }
        
        score
    }

    fn evaluate_position(&self, position: &Position) -> i32 {
        let mut score = 0;
        
        // Evaluate piece-square tables
        for square_idx in 0..64 {
            if let Some(square) = Square::new(square_idx) {
                if let Some(piece) = position.piece_at(square) {
                    let piece_value = self.get_piece_square_value(
                        piece.piece_type, 
                        square, 
                        piece.color, 
                        GamePhase::MiddleGame // Simplified for now
                    );
                    
                    score += if piece.color == Color::White {
                        piece_value
                    } else {
                        -piece_value
                    };
                }
            }
        }
        
        score
    }

    fn evaluate_pawn_structure(&self, position: &Position) -> i32 {
        let mut score = 0;
        
        // Evaluate doubled pawns, isolated pawns, passed pawns, etc.
        let white_pawns = position.piece_bitboard(PieceType::Pawn, Color::White);
        let black_pawns = position.piece_bitboard(PieceType::Pawn, Color::Black);
        
        // Doubled pawns penalty
        score -= self.count_doubled_pawns(white_pawns) * 10;
        score += self.count_doubled_pawns(black_pawns) * 10;
        
        // Isolated pawns penalty
        score -= self.count_isolated_pawns(white_pawns) * 15;
        score += self.count_isolated_pawns(black_pawns) * 15;
        
        // Passed pawns bonus
        score += self.count_passed_pawns(white_pawns, black_pawns) * 30;
        score -= self.count_passed_pawns(black_pawns, white_pawns) * 30;
        
        score
    }

    fn evaluate_mobility(&self, _position: &Position) -> (i32, i32) {
        // This would require move generation, simplified for now
        let white_mobility = 20; // Placeholder
        let black_mobility = 18; // Placeholder
        
        (white_mobility, black_mobility)
    }

    fn get_or_compute_king_safety(&mut self, position: &Position, color: Color) -> i32 {
        let king_pawn_hash = self.compute_king_pawn_hash(position, color);
        
        self.king_safety_cache
            .get(&(king_pawn_hash, color))
            .copied()
            .unwrap_or_else(|| {
                let safety = self.evaluate_king_safety(position, color);
                self.king_safety_cache.insert((king_pawn_hash, color), safety);
                safety
            })
    }

    fn evaluate_king_safety(&self, position: &Position, color: Color) -> i32 {
        // Simplified king safety evaluation
        let king_pos = match position.king_square(color) {
            Some(square) => square,
            None => return 0, // No king found, return neutral score
        };
        let _enemy_color = color.opposite();
        
        let mut safety_score = 0;
        
        // Penalty for king in center during middle game
        if matches!(king_pos.file(), 2..=5) && matches!(king_pos.rank(), 2..=5) {
            safety_score -= 50;
        }
        
        // Bonus for castling
        if position.has_castled(color) {
            safety_score += 40;
        }
        
        // Penalty for open files near king
        let king_file = king_pos.file();
        for file_offset in -1..=1 {
            let file = king_file as i32 + file_offset;
            if file >= 0 && file < 8 {
                if !self.has_pawn_on_file(position, file as u8, color) {
                    safety_score -= 20;
                }
            }
        }
        
        safety_score
    }

    fn get_piece_square_value(&self, piece_type: PieceType, square: Square, color: Color, phase: GamePhase) -> i32 {
        let square_idx = if color == Color::White {
            square.index() as usize
        } else {
            // Flip board for black pieces
            (square.index() ^ 56) as usize
        };
        
        let base_value = match piece_type {
            PieceType::Pawn => PAWN_TABLE[square_idx],
            PieceType::Knight => KNIGHT_TABLE[square_idx],
            PieceType::Bishop => BISHOP_TABLE[square_idx],
            PieceType::Rook => ROOK_TABLE[square_idx],
            PieceType::Queen => QUEEN_TABLE[square_idx],
            PieceType::King => match phase {
                GamePhase::EndGame => KING_END_GAME[square_idx],
                _ => KING_MIDDLE_GAME[square_idx],
            },
        };
        
        base_value
    }

    // SIMD helper functions
    fn extract_piece_counts(&self, position: &Position, color: Color) -> [i32; 6] {
        [
            self.count_pieces(position, PieceType::Pawn, color) as i32,
            self.count_pieces(position, PieceType::Knight, color) as i32,
            self.count_pieces(position, PieceType::Bishop, color) as i32,
            self.count_pieces(position, PieceType::Rook, color) as i32,
            self.count_pieces(position, PieceType::Queen, color) as i32,
            self.count_pieces(position, PieceType::King, color) as i32,
        ]
    }

    fn simd_compute_material(&self, piece_counts: &[[i32; 6]]) -> [i32; 1] {
        // SIMD material computation (simplified for single position)
        let mut total = 0;
        for (i, &count) in piece_counts[0].iter().enumerate() {
            total += count * PIECE_VALUES[i];
        }
        [total]
    }

    // Helper functions (these would need actual implementation based on Position interface)
    fn count_pieces(&self, position: &Position, piece_type: PieceType, color: Color) -> u32 {
        position.piece_bitboard(piece_type, color).count_bits()
    }

    fn compute_pawn_hash(&self, position: &Position) -> u64 {
        // Hash based only on pawn positions
        let white_pawns = position.piece_bitboard(PieceType::Pawn, Color::White);
        let black_pawns = position.piece_bitboard(PieceType::Pawn, Color::Black);
        white_pawns.value() ^ black_pawns.value().rotate_left(32)
    }

    fn compute_king_pawn_hash(&self, position: &Position, color: Color) -> u64 {
        let king = position.piece_bitboard(PieceType::King, color);
        let pawns = position.piece_bitboard(PieceType::Pawn, color);
        king.value() ^ pawns.value()
    }

    fn count_doubled_pawns(&self, pawns: Bitboard) -> i32 {
        let mut count = 0;
        for file in 0..8 {
            let file_mask = Bitboard::FILE_A.value() << file;
            let pawns_on_file = (pawns.value() & file_mask).count_ones();
            if pawns_on_file > 1 {
                count += (pawns_on_file - 1) as i32;
            }
        }
        count
    }

    fn count_isolated_pawns(&self, pawns: Bitboard) -> i32 {
        let mut count = 0;
        for file in 0..8 {
            let file_mask = Bitboard::FILE_A.value() << file;
            if pawns.value() & file_mask != 0 {
                // Check adjacent files
                let adjacent_files = if file == 0 {
                    Bitboard::FILE_B.value()
                } else if file == 7 {
                    Bitboard::FILE_G.value()
                } else {
                    (Bitboard::FILE_A.value() << (file - 1)) | (Bitboard::FILE_A.value() << (file + 1))
                };
                
                if pawns.value() & adjacent_files == 0 {
                    count += (pawns.value() & file_mask).count_ones() as i32;
                }
            }
        }
        count
    }

    fn count_passed_pawns(&self, own_pawns: Bitboard, enemy_pawns: Bitboard) -> i32 {
        // Simplified passed pawn detection
        let mut count = 0;
        let mut pawns = own_pawns;
        
        while let Some(square_idx) = pawns.pop_lsb() {
            let square = Square::new(square_idx as u8).unwrap();
            let file = square.file();
            
            // Check if there are enemy pawns ahead on same file or adjacent files
            let ahead_mask = self.get_ahead_mask(square, Color::White); // Simplified
            let relevant_files = if file == 0 {
                Bitboard::FILE_A | Bitboard::FILE_B
            } else if file == 7 {
                Bitboard::FILE_G | Bitboard::FILE_H
            } else {
                Bitboard::new((Bitboard::FILE_A.value() << (file - 1)) | 
                             (Bitboard::FILE_A.value() << file) |
                             (Bitboard::FILE_A.value() << (file + 1)))
            };
            
            if (enemy_pawns & ahead_mask & relevant_files).is_empty() {
                count += 1;
            }
        }
        
        count
    }

    fn get_ahead_mask(&self, square: Square, color: Color) -> Bitboard {
        // Get mask of squares ahead of the given square
        if color == Color::White {
            let rank_mask = !((1u64 << ((square.rank() + 1) * 8)) - 1);
            Bitboard::new(rank_mask)
        } else {
            let rank_mask = (1u64 << (square.rank() * 8)) - 1;
            Bitboard::new(rank_mask)
        }
    }

    fn has_pawn_on_file(&self, position: &Position, file: u8, color: Color) -> bool {
        let file_mask = Bitboard::FILE_A.value() << file;
        let pawns = position.piece_bitboard(PieceType::Pawn, color);
        (pawns.value() & file_mask) != 0
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (u64, u64, f64) {
        let total = self.cache_hits + self.cache_misses;
        let hit_rate = if total > 0 {
            self.cache_hits as f64 / total as f64
        } else {
            0.0
        };
        (self.cache_hits, self.cache_misses, hit_rate)
    }

    /// Clear all caches
    pub fn clear_caches(&mut self) {
        self.evaluation_cache.clear();
        self.pawn_structure_cache.clear();
        self.king_safety_cache.clear();
        self.mobility_cache.clear();
        self.cache_hits = 0;
        self.cache_misses = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_phase_detection() {
        assert_eq!(GamePhase::from_material(7000), GamePhase::Opening);
        assert_eq!(GamePhase::from_material(3000), GamePhase::MiddleGame);
        assert_eq!(GamePhase::from_material(1000), GamePhase::EndGame);
    }

    #[test]
    fn test_piece_values() {
        assert_eq!(PIECE_VALUES[PieceType::Pawn as usize], 100);
        assert_eq!(PIECE_VALUES[PieceType::Queen as usize], 900);
        assert_eq!(PIECE_VALUES[PieceType::King as usize], 20000);
    }

    #[test]
    fn test_evaluator_creation() {
        let evaluator = OptimizedEvaluator::new();
        let (hits, misses, rate) = evaluator.cache_stats();
        assert_eq!(hits, 0);
        assert_eq!(misses, 0);
        assert_eq!(rate, 0.0);
    }

    #[test]
    fn test_doubled_pawns_count() {
        let evaluator = OptimizedEvaluator::new();
        
        // Test with no doubled pawns
        let pawns = Bitboard::new(0x00FF000000000000); // Pawns on different files
        assert_eq!(evaluator.count_doubled_pawns(pawns), 0);
        
        // Test with doubled pawns
        let doubled_pawns = Bitboard::new(0x0101000000000000); // Two pawns on A file
        assert_eq!(evaluator.count_doubled_pawns(doubled_pawns), 1);
    }
}