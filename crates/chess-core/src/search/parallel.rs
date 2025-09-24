// Parallel processing for chess engine using Rayon
// Implements multi-threaded move generation, search, and evaluation

use crate::{Bitboard, Move, MoveGenerator, OptimizedEvaluator, Position};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Configuration for parallel processing
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    pub num_threads: usize,
    pub chunk_size: usize,
    pub enable_parallel_moves: bool,
    pub enable_parallel_eval: bool,
    pub enable_parallel_search: bool,
}

#[derive(Debug, Clone)]
struct TranspositionEntry {
    #[allow(dead_code)]
    zobrist_hash: u64,
    depth: u8,
    evaluation: i32,
    #[allow(dead_code)]
    best_move: Option<Move>,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        ParallelConfig {
            num_threads: rayon::current_num_threads(),
            chunk_size: 64,
            enable_parallel_moves: true,
            enable_parallel_eval: true,
            enable_parallel_search: true,
        }
    }
}

/// Parallel move generation engine
pub struct ParallelMoveGenerator {
    generator: MoveGenerator,
    config: ParallelConfig,
}

impl ParallelMoveGenerator {
    pub fn new(config: ParallelConfig) -> Self {
        ParallelMoveGenerator {
            generator: MoveGenerator::new(),
            config,
        }
    }

    /// Generate moves for multiple positions in parallel
    pub fn bulk_generate_moves(&self, positions: &[Position]) -> Vec<Vec<Move>> {
        if !self.config.enable_parallel_moves || positions.len() < self.config.chunk_size {
            // Use sequential generation for small batches
            return positions
                .iter()
                .map(|pos| self.generator.generate_legal_moves(pos))
                .collect();
        }

        positions
            .par_chunks(self.config.chunk_size)
            .flat_map(|chunk| {
                chunk
                    .par_iter()
                    .map(|pos| self.generator.generate_legal_moves(pos))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Parallel move validation for multiple move-position pairs
    pub fn bulk_validate_moves(&self, move_position_pairs: &[(Move, Position)]) -> Vec<bool> {
        if move_position_pairs.len() < self.config.chunk_size {
            return move_position_pairs
                .iter()
                .map(|(move_item, pos)| self.is_legal_move(move_item, pos))
                .collect();
        }

        move_position_pairs
            .par_iter()
            .map(|(move_item, pos)| self.is_legal_move(move_item, pos))
            .collect()
    }

    /// Parallel move generation with SIMD optimization
    pub fn simd_parallel_move_generation(&self, positions: &[Position]) -> Vec<Vec<Move>> {
        // Process positions in groups of 4 for SIMD optimization
        positions
            .par_chunks(4)
            .flat_map(|chunk| self.process_position_chunk_simd(chunk))
            .collect()
    }

    fn process_position_chunk_simd(&self, chunk: &[Position]) -> Vec<Vec<Move>> {
        // Extract bitboards for SIMD processing
        let mut piece_boards = Vec::new();
        let mut occupancy_boards = Vec::new();

        for pos in chunk {
            piece_boards.push(self.extract_piece_bitboards(pos));
            occupancy_boards.push(pos.all_pieces());
        }

        // Process up to 4 positions with SIMD
        let padded_chunk_size = chunk.len().min(4);
        let mut results = Vec::with_capacity(padded_chunk_size);

        for position in chunk.iter().take(padded_chunk_size) {
            // For now, use regular move generation - SIMD integration would require more complex bitboard operations
            results.push(self.generator.generate_legal_moves(position));
        }

        results
    }

    fn extract_piece_bitboards(&self, _position: &Position) -> [Bitboard; 12] {
        // Extract all piece bitboards for SIMD processing
        // This would need to be implemented based on your Position structure
        [Bitboard::EMPTY; 12] // Placeholder
    }

    fn is_legal_move(&self, move_item: &Move, position: &Position) -> bool {
        // Check if a move is legal in the given position
        let legal_moves = self.generator.generate_legal_moves(position);
        legal_moves.contains(move_item)
    }
}

/// Parallel evaluation engine
pub struct ParallelEvaluator {
    evaluator: OptimizedEvaluator,
    config: ParallelConfig,
}

impl ParallelEvaluator {
    pub fn new(config: ParallelConfig) -> Self {
        ParallelEvaluator {
            evaluator: OptimizedEvaluator::new(),
            config,
        }
    }

    /// Evaluate multiple positions in parallel
    pub fn bulk_evaluate(&mut self, positions: &[Position]) -> Vec<i32> {
        if !self.config.enable_parallel_eval || positions.len() < self.config.chunk_size {
            return positions
                .iter()
                .map(|pos| self.evaluator.evaluate(pos))
                .collect();
        }

        // For parallel evaluation, we need to clone the evaluator for each thread
        // to avoid borrowing issues
        let evaluator = self.evaluator.clone();
        positions
            .par_chunks(self.config.chunk_size)
            .flat_map(|chunk| {
                chunk
                    .par_iter()
                    .map(|pos| {
                        let mut eval = evaluator.clone();
                        eval.evaluate(pos)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// SIMD-optimized parallel evaluation for multiple positions
    pub fn simd_bulk_evaluate(&mut self, positions: &[Position]) -> Vec<i32> {
        let evaluator = self.evaluator.clone();
        positions
            .par_chunks(4)
            .flat_map(|chunk| {
                let mut eval = evaluator.clone();
                Self::simd_evaluate_chunk_static(&mut eval, chunk)
            })
            .collect()
    }

    fn simd_evaluate_chunk_static(
        evaluator: &mut OptimizedEvaluator,
        chunk: &[Position],
    ) -> Vec<i32> {
        // For simplicity in this case, just evaluate sequentially
        // The SIMD optimizations are better done inside the evaluator itself
        chunk.iter().map(|pos| evaluator.evaluate(pos)).collect()
    }

    #[allow(dead_code)]
    fn simd_evaluate_4_positions(
        &self,
        piece_counts: &[[i32; 12]],
        positional_features: &[[i32; 64]],
    ) -> Vec<i32> {
        // SIMD evaluation of 4 positions simultaneously
        let mut results = Vec::with_capacity(4);

        // Material evaluation using SIMD
        let material_weights = [
            100, 320, 330, 500, 900, 20000, -100, -320, -330, -500, -900, -20000,
        ]; // Piece values

        for i in 0..piece_counts.len() {
            let mut score = 0;

            // SIMD material calculation (would use actual SIMD instructions in production)
            for (j, &count) in piece_counts[i].iter().enumerate() {
                score += count * material_weights[j];
            }

            // Add positional evaluation
            score += positional_features[i].iter().sum::<i32>() / 10;

            results.push(score);
        }

        results
    }

    #[allow(dead_code)]
    fn extract_piece_counts(&self, _position: &Position) -> [i32; 12] {
        // Extract piece counts for material evaluation
        // This would need implementation based on Position structure
        [0; 12] // Placeholder
    }

    #[allow(dead_code)]
    fn extract_positional_features(&self, _position: &Position) -> [i32; 64] {
        // Extract positional features for each square
        // This would need implementation based on Position structure
        [0; 64] // Placeholder
    }
}

/// Shared data for parallel search
#[derive(Clone)]
struct SharedSearchData {
    transposition_table: Arc<Mutex<HashMap<u64, TranspositionEntry>>>,
    best_move: Arc<Mutex<Option<Move>>>,
    nodes_searched: Arc<Mutex<u64>>,
}

/// Parallel search engine using Lazy SMP
pub struct ParallelSearchEngine {
    config: ParallelConfig,
    move_generator: ParallelMoveGenerator,
    evaluator: ParallelEvaluator,
}

impl ParallelSearchEngine {
    pub fn new(config: ParallelConfig) -> Self {
        ParallelSearchEngine {
            move_generator: ParallelMoveGenerator::new(config.clone()),
            evaluator: ParallelEvaluator::new(config.clone()),
            config,
        }
    }

    /// Parallel root search using multiple threads
    pub fn parallel_root_search(&self, position: &Position, depth: u8) -> (Option<Move>, i32, u64) {
        if !self.config.enable_parallel_search {
            return self.sequential_search(position, depth);
        }

        let shared_data = SharedSearchData {
            transposition_table: Arc::new(Mutex::new(HashMap::new())),
            best_move: Arc::new(Mutex::new(None)),
            nodes_searched: Arc::new(Mutex::new(0)),
        };

        let legal_moves = self.move_generator.generator.generate_legal_moves(position);
        if legal_moves.is_empty() {
            return (None, 0, 1);
        }

        // Divide moves among threads
        let moves_per_thread = legal_moves.len().div_ceil(self.config.num_threads);
        let move_chunks: Vec<_> = legal_moves.chunks(moves_per_thread).collect();

        let results: Vec<_> = move_chunks
            .par_iter()
            .enumerate()
            .map(|(thread_id, moves)| {
                self.search_thread(position, moves, depth, thread_id, shared_data.clone())
            })
            .collect();

        // Find best result
        let mut best_move = None;
        let mut best_evaluation = i32::MIN;
        let total_nodes = results.iter().map(|(_, _, nodes)| nodes).sum();

        for (thread_move, evaluation, _) in results {
            if evaluation > best_evaluation {
                best_evaluation = evaluation;
                best_move = thread_move;
            }
        }

        (best_move, best_evaluation, total_nodes)
    }

    fn search_thread(
        &self,
        position: &Position,
        moves: &[Move],
        depth: u8,
        thread_id: usize,
        shared_data: SharedSearchData,
    ) -> (Option<Move>, i32, u64) {
        let mut local_nodes = 0;
        let mut best_move = None;
        let mut best_evaluation = i32::MIN;

        // Add thread-specific depth variation for Lazy SMP
        let thread_depth = if thread_id == 0 {
            depth // Main thread uses full depth
        } else {
            depth.saturating_sub((thread_id % 3) as u8) // Helper threads use slightly reduced depth
        };

        for &move_item in moves {
            let mut new_position = position.clone();
            if new_position.make_move(move_item).is_err() {
                continue;
            }

            let evaluation = self.alpha_beta_search(
                &new_position,
                thread_depth.saturating_sub(1),
                i32::MIN,
                i32::MAX,
                &shared_data,
            );

            local_nodes += 1;

            if evaluation > best_evaluation {
                best_evaluation = evaluation;
                best_move = Some(move_item);

                // Update shared best move if this is better
                if let Ok(mut shared_best) = shared_data.best_move.try_lock() {
                    *shared_best = Some(move_item);
                }
            }
        }

        // Update shared node count
        if let Ok(mut shared_nodes) = shared_data.nodes_searched.try_lock() {
            *shared_nodes += local_nodes;
        }

        (best_move, best_evaluation, local_nodes)
    }

    fn alpha_beta_search(
        &self,
        position: &Position,
        depth: u8,
        alpha: i32,
        beta: i32,
        shared_data: &SharedSearchData,
    ) -> i32 {
        if depth == 0 {
            let mut evaluator = self.evaluator.evaluator.clone();
            return evaluator.evaluate(position);
        }

        // Check transposition table (with lock)
        let zobrist = position.zobrist_hash();
        if let Ok(tt) = shared_data.transposition_table.try_lock() {
            if let Some(entry) = tt.get(&zobrist) {
                if entry.depth >= depth {
                    return entry.evaluation;
                }
            }
        }

        let legal_moves = self.move_generator.generator.generate_legal_moves(position);
        if legal_moves.is_empty() {
            return -10000; // Simplified checkmate/stalemate evaluation
        }

        let mut best_score = alpha;
        let mut best_move = None;

        for move_item in legal_moves {
            let mut new_position = position.clone();
            if new_position.make_move(move_item).is_err() {
                continue;
            }

            // Use saturating_neg to avoid overflow when negating i32::MIN
            let next_alpha = beta.saturating_neg();
            let next_beta = best_score.saturating_neg();
            let score = -self.alpha_beta_search(
                &new_position,
                depth - 1,
                next_alpha,
                next_beta,
                shared_data,
            );

            if score > best_score {
                best_score = score;
                best_move = Some(move_item);

                if best_score >= beta {
                    break; // Beta cutoff
                }
            }
        }

        // Store in transposition table (with lock)
        if let Ok(mut tt) = shared_data.transposition_table.try_lock() {
            tt.insert(
                zobrist,
                TranspositionEntry {
                    zobrist_hash: zobrist,
                    depth,
                    evaluation: best_score,
                    best_move,
                },
            );
        }

        best_score
    }

    fn sequential_search(&self, position: &Position, depth: u8) -> (Option<Move>, i32, u64) {
        // Fallback sequential search
        let legal_moves = self.move_generator.generator.generate_legal_moves(position);
        if legal_moves.is_empty() {
            return (None, 0, 1);
        }

        let mut best_move = None;
        let mut best_evaluation = i32::MIN;
        let mut nodes = 0;

        for move_item in legal_moves {
            let mut new_position = position.clone();
            if new_position.make_move(move_item).is_err() {
                continue;
            }

            let evaluation = self.minimax(&new_position, depth - 1, false);
            nodes += 1;

            if evaluation > best_evaluation {
                best_evaluation = evaluation;
                best_move = Some(move_item);
            }
        }

        (best_move, best_evaluation, nodes)
    }

    fn minimax(&self, position: &Position, depth: u8, maximizing: bool) -> i32 {
        if depth == 0 {
            let mut evaluator = self.evaluator.evaluator.clone();
            return evaluator.evaluate(position);
        }

        let legal_moves = self.move_generator.generator.generate_legal_moves(position);
        if legal_moves.is_empty() {
            return if maximizing { -10000 } else { 10000 };
        }

        let mut best_value = if maximizing { i32::MIN } else { i32::MAX };

        for move_item in legal_moves {
            let mut new_position = position.clone();
            if new_position.make_move(move_item).is_err() {
                continue;
            }

            let value = self.minimax(&new_position, depth - 1, !maximizing);

            if maximizing {
                best_value = best_value.max(value);
            } else {
                best_value = best_value.min(value);
            }
        }

        best_value
    }
}

/// Utility functions for parallel processing
pub struct ParallelUtils;

impl ParallelUtils {
    /// Initialize Rayon thread pool with custom configuration
    pub fn initialize_thread_pool(num_threads: usize) -> Result<(), rayon::ThreadPoolBuildError> {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .thread_name(|i| format!("chess-engine-{}", i))
            .build_global()
    }

    /// Parallel perft testing for move generation validation
    pub fn parallel_perft(position: &Position, depth: u8, move_generator: &MoveGenerator) -> u64 {
        if depth == 0 {
            return 1;
        }

        let legal_moves = move_generator.generate_legal_moves(position);

        if depth == 1 {
            return legal_moves.len() as u64;
        }

        legal_moves
            .par_iter()
            .map(|&move_item| {
                let mut new_position = position.clone();
                if new_position.make_move(move_item).is_ok() {
                    Self::sequential_perft(&new_position, depth - 1, move_generator)
                } else {
                    0
                }
            })
            .sum()
    }

    fn sequential_perft(position: &Position, depth: u8, move_generator: &MoveGenerator) -> u64 {
        if depth == 0 {
            return 1;
        }

        let legal_moves = move_generator.generate_legal_moves(position);

        if depth == 1 {
            return legal_moves.len() as u64;
        }

        let mut count = 0;
        for move_item in legal_moves {
            let mut new_position = position.clone();
            if new_position.make_move(move_item).is_ok() {
                count += Self::sequential_perft(&new_position, depth - 1, move_generator);
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_config_default() {
        let config = ParallelConfig::default();
        assert!(config.num_threads > 0);
        assert!(config.enable_parallel_moves);
        assert!(config.enable_parallel_eval);
        assert!(config.enable_parallel_search);
    }

    #[test]
    fn test_parallel_move_generator_creation() {
        let config = ParallelConfig::default();
        let generator = ParallelMoveGenerator::new(config);
        assert_eq!(generator.config.chunk_size, 64);
    }

    #[test]
    fn test_parallel_evaluator_creation() {
        let config = ParallelConfig::default();
        let evaluator = ParallelEvaluator::new(config);
        assert!(evaluator.config.enable_parallel_eval);
    }
}
