// Advanced search algorithms for chess engine
// Implements alpha-beta pruning, iterative deepening, and other modern search techniques

use crate::{Position, Move, Color, PieceType, OptimizedEvaluator, MoveGenerator};
use std::time::{Duration, Instant};
use std::collections::HashMap;

pub const MATE_VALUE: i32 = 32000;
pub const MAX_DEPTH: u8 = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchResult {
    pub best_move: Option<Move>,
    pub evaluation: i32,
    pub depth: u8,
    pub nodes_searched: u64,
    pub elapsed_time: Duration,
    pub principal_variation: [Option<Move>; 16],
}

#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub max_depth: u8,
    pub max_time: Option<Duration>,
    pub max_nodes: Option<u64>,
    pub use_null_move_pruning: bool,
    pub use_late_move_reductions: bool,
    pub use_futility_pruning: bool,
    pub aspiration_window: i32,
}

impl Default for SearchConfig {
    fn default() -> Self {
        SearchConfig {
            max_depth: 8,
            max_time: Some(Duration::from_secs(5)),
            max_nodes: None,
            use_null_move_pruning: true,
            use_late_move_reductions: true,
            use_futility_pruning: true,
            aspiration_window: 50,
        }
    }
}

/// Transposition table entry for storing search results
#[derive(Debug, Clone, Copy)]
struct TranspositionEntry {
    zobrist_hash: u64,
    depth: u8,
    evaluation: i32,
    #[allow(dead_code)]
    best_move: Option<Move>,
    node_type: NodeType,
    #[allow(dead_code)]
    age: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeType {
    Exact,      // PV node - exact evaluation
    LowerBound, // Cut node - beta cutoff (fail high)
    UpperBound, // All node - alpha didn't improve (fail low)
}

/// Advanced chess search engine with modern techniques
pub struct SearchEngine {
    evaluator: OptimizedEvaluator,
    move_generator: MoveGenerator,
    transposition_table: HashMap<u64, TranspositionEntry>,
    killer_moves: [[Option<Move>; 2]; MAX_DEPTH as usize],
    history_table: HashMap<(Move, Color), u32>,
    nodes_searched: u64,
    start_time: Instant,
    config: SearchConfig,
    age: u8,
}

impl SearchEngine {
    pub fn new(config: SearchConfig) -> Self {
        SearchEngine {
            evaluator: OptimizedEvaluator::new(),
            move_generator: MoveGenerator::new(),
            transposition_table: HashMap::with_capacity(1024 * 1024), // 1M entries
            killer_moves: [[None; 2]; MAX_DEPTH as usize],
            history_table: HashMap::new(),
            nodes_searched: 0,
            start_time: Instant::now(),
            config,
            age: 0,
        }
    }

    /// Main search function using iterative deepening
    pub fn search(&mut self, position: &Position) -> SearchResult {
        self.nodes_searched = 0;
        self.start_time = Instant::now();
        self.age = self.age.wrapping_add(1);

        let mut best_result = SearchResult {
            best_move: None,
            evaluation: -MATE_VALUE,
            depth: 0,
            nodes_searched: 0,
            elapsed_time: Duration::from_millis(0),
            principal_variation: [None; 16],
        };

        // Iterative deepening
        for depth in 1..=self.config.max_depth {
            if self.should_stop() {
                break;
            }

            let mut alpha = -MATE_VALUE;
            let mut beta = MATE_VALUE;

            // Aspiration windows for depths > 2
            if depth > 2 && best_result.best_move.is_some() {
                alpha = best_result.evaluation - self.config.aspiration_window;
                beta = best_result.evaluation + self.config.aspiration_window;
            }

            let mut search_result = self.aspiration_search(position, depth, alpha, beta);

            // Re-search with wider window if we fell outside aspiration window
            if search_result.evaluation <= alpha || search_result.evaluation >= beta {
                search_result = self.alpha_beta_root(position, depth, -MATE_VALUE, MATE_VALUE);
            }

            if !self.should_stop() {
                best_result = search_result;
                best_result.depth = depth;
            }

            // Stop if we found mate
            if best_result.evaluation.abs() > MATE_VALUE - 100 {
                break;
            }
        }

        best_result.nodes_searched = self.nodes_searched;
        best_result.elapsed_time = self.start_time.elapsed();
        best_result
    }

    fn aspiration_search(&mut self, position: &Position, depth: u8, mut alpha: i32, mut beta: i32) -> SearchResult {
        let mut attempts = 0;
        loop {
            let result = self.alpha_beta_root(position, depth, alpha, beta);
            
            if result.evaluation > alpha && result.evaluation < beta {
                return result;
            }

            attempts += 1;
            if attempts > 3 {
                // Give up on aspiration windows and search with full window
                return self.alpha_beta_root(position, depth, -MATE_VALUE, MATE_VALUE);
            }

            // Widen the window
            if result.evaluation <= alpha {
                alpha -= self.config.aspiration_window * (1 << attempts);
            }
            if result.evaluation >= beta {
                beta += self.config.aspiration_window * (1 << attempts);
            }
        }
    }

    fn alpha_beta_root(&mut self, position: &Position, depth: u8, mut alpha: i32, beta: i32) -> SearchResult {
        let mut best_move = None;
        let mut pv = [None; 16];
        
        let legal_moves = self.generate_and_sort_moves(position, depth, None);
        
        for (i, move_data) in legal_moves.iter().enumerate() {
            if self.should_stop() {
                break;
            }

            let mut new_position = position.clone();
            if new_position.make_move(move_data.0).is_err() {
                continue;
            }

            let evaluation = if i == 0 {
                // Full search for first move
                -self.alpha_beta(&new_position, depth - 1, -beta, -alpha, false)
            } else {
                // Late move reductions
                let reduction = if self.config.use_late_move_reductions 
                    && depth >= 3 
                    && i >= 4 
                    && !move_data.0.is_capture() 
                    && !self.is_check(&new_position) {
                    1
                } else {
                    0
                };

                let reduced_depth = (depth - 1).saturating_sub(reduction);
                let mut score = -self.alpha_beta(&new_position, reduced_depth, -alpha - 1, -alpha, false);

                // Re-search if reduced search failed high
                if reduction > 0 && score > alpha {
                    score = -self.alpha_beta(&new_position, depth - 1, -alpha - 1, -alpha, false);
                }

                // Re-search with full window if necessary
                if score > alpha && score < beta {
                    score = -self.alpha_beta(&new_position, depth - 1, -beta, -alpha, false);
                }

                score
            };

            if evaluation > alpha {
                alpha = evaluation;
                best_move = Some(move_data.0);
                pv[0] = Some(move_data.0);
            }

            if alpha >= beta {
                break; // Beta cutoff
            }
        }

        SearchResult {
            best_move,
            evaluation: alpha,
            depth,
            nodes_searched: self.nodes_searched,
            elapsed_time: self.start_time.elapsed(),
            principal_variation: pv,
        }
    }

    /// Main alpha-beta search with pruning techniques
    fn alpha_beta(&mut self, position: &Position, depth: u8, mut alpha: i32, mut beta: i32, null_move: bool) -> i32 {
        self.nodes_searched += 1;

        if self.should_stop() {
            return alpha;
        }

        // Check for immediate draws
        if self.is_draw(position) {
            return 0;
        }

        // Mate distance pruning
        alpha = alpha.max(-MATE_VALUE + position.halfmove_clock() as i32);
        beta = beta.min(MATE_VALUE - position.halfmove_clock() as i32);
        if alpha >= beta {
            return alpha;
        }

        // Transposition table lookup
        let zobrist = position.zobrist_hash();
        if let Some(entry) = self.transposition_table.get(&zobrist) {
            if entry.zobrist_hash == zobrist && entry.depth >= depth {
                match entry.node_type {
                    NodeType::Exact => return entry.evaluation,
                    NodeType::LowerBound if entry.evaluation >= beta => return entry.evaluation,
                    NodeType::UpperBound if entry.evaluation <= alpha => return entry.evaluation,
                    _ => {}
                }
            }
        }

        // Terminal node evaluation
        if depth == 0 {
            return self.quiescence_search(position, alpha, beta, 0);
        }

        let in_check = self.is_check(position);
        let static_eval = if in_check {
            -MATE_VALUE + position.halfmove_clock() as i32
        } else {
            self.evaluator.evaluate(position)
        };

        // Null move pruning
        if self.config.use_null_move_pruning
            && !null_move 
            && !in_check
            && depth >= 3
            && static_eval >= beta
            && self.has_non_pawn_pieces(position) {
            
            let mut null_position = position.clone();
            null_position.make_null_move();
            
            let null_score = -self.alpha_beta(&null_position, depth - 3, -beta, -beta + 1, true);
            if null_score >= beta {
                return beta; // Fail high
            }
        }

        // Futility pruning
        if self.config.use_futility_pruning
            && !in_check
            && depth <= 3
            && static_eval + 200 * depth as i32 <= alpha {
            return static_eval;
        }

        let legal_moves = self.generate_and_sort_moves(position, depth, 
            self.transposition_table.get(&zobrist).and_then(|e| e.best_move));
        
        if legal_moves.is_empty() {
            return if in_check {
                -MATE_VALUE + position.halfmove_clock() as i32 // Checkmate
            } else {
                0 // Stalemate
            };
        }

        let mut best_move = None;
        let mut node_type = NodeType::UpperBound;
        let mut moves_searched = 0;

        for (move_item, _score) in legal_moves {
            let mut new_position = position.clone();
            if new_position.make_move(move_item).is_err() {
                continue;
            }

            moves_searched += 1;
            let evaluation = -self.alpha_beta(&new_position, depth - 1, -beta, -alpha, false);

            if evaluation > alpha {
                alpha = evaluation;
                best_move = Some(move_item);
                node_type = NodeType::Exact;

                if alpha >= beta {
                    // Store killer move
                    if !move_item.is_capture() {
                        self.store_killer_move(move_item, depth);
                    }
                    
                    // Update history heuristic
                    self.update_history(move_item, position.side_to_move(), depth);
                    
                    node_type = NodeType::LowerBound;
                    break; // Beta cutoff
                }
            }
        }

        // Store in transposition table
        self.store_transposition(zobrist, depth, alpha, best_move, node_type);

        // Update nodes searched based on moves examined
        if moves_searched > 0 {
            self.nodes_searched += moves_searched as u64;
        }

        alpha
    }

    /// Quiescence search for tactical positions
    fn quiescence_search(&mut self, position: &Position, mut alpha: i32, beta: i32, ply: u8) -> i32 {
        self.nodes_searched += 1;

        if ply > 16 || self.should_stop() {
            return self.evaluator.evaluate(position);
        }

        let static_eval = self.evaluator.evaluate(position);
        
        if static_eval >= beta {
            return beta;
        }
        
        if static_eval > alpha {
            alpha = static_eval;
        }

        // Generate only captures and checks
        let captures = self.generate_tactical_moves(position);
        
        for move_item in captures {
            let mut new_position = position.clone();
            if new_position.make_move(move_item).is_err() {
                continue;
            }

            let evaluation = -self.quiescence_search(&new_position, -beta, -alpha, ply + 1);

            if evaluation > alpha {
                alpha = evaluation;
                if alpha >= beta {
                    return beta; // Beta cutoff
                }
            }
        }

        alpha
    }

    fn generate_and_sort_moves(&self, position: &Position, depth: u8, tt_move: Option<Move>) -> Vec<(Move, i32)> {
        let moves = self.move_generator.generate_legal_moves(position);
        let mut scored_moves = Vec::with_capacity(moves.len());

        for move_item in moves {
            let score = self.score_move(move_item, position, depth, tt_move);
            scored_moves.push((move_item, score));
        }

        // Sort moves by score (highest first)
        scored_moves.sort_by(|a, b| b.1.cmp(&a.1));
        scored_moves
    }

    fn generate_tactical_moves(&self, position: &Position) -> Vec<Move> {
        // For now, return all legal moves - could be optimized to only return captures/checks
        self.move_generator.generate_legal_moves(position)
    }

    fn score_move(&self, move_item: Move, position: &Position, depth: u8, tt_move: Option<Move>) -> i32 {
        // Transposition table move gets highest priority
        if Some(move_item) == tt_move {
            return 10000;
        }

        let mut score = 0;

        // Captures (MVV-LVA: Most Valuable Victim - Least Valuable Attacker)
        if move_item.is_capture() {
            if let Some(captured_piece) = position.piece_at(move_item.to) {
                if let Some(attacker_piece) = position.piece_at(move_item.from) {
                    score += Self::piece_value(captured_piece.piece_type) * 10 - Self::piece_value(attacker_piece.piece_type);
                }
            }
        }

        // Promotions
        if let Some(promotion_piece) = move_item.promotion_piece() {
            score += Self::piece_value(promotion_piece) - 100; // Pawn value
        }

        // Killer moves
        if depth < MAX_DEPTH as u8 {
            for killer in &self.killer_moves[depth as usize] {
                if *killer == Some(move_item) {
                    score += 9000;
                    break;
                }
            }
        }

        // History heuristic
        if let Some(&history_score) = self.history_table.get(&(move_item, position.side_to_move())) {
            score += (history_score / 100) as i32;
        }

        score
    }

    fn piece_value(piece_type: PieceType) -> i32 {
        match piece_type {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 20000,
        }
    }

    fn store_killer_move(&mut self, move_item: Move, depth: u8) {
        if depth < MAX_DEPTH as u8 {
            let killers = &mut self.killer_moves[depth as usize];
            if killers[0] != Some(move_item) {
                killers[1] = killers[0];
                killers[0] = Some(move_item);
            }
        }
    }

    fn update_history(&mut self, move_item: Move, color: Color, depth: u8) {
        let entry = self.history_table.entry((move_item, color)).or_insert(0);
        *entry += (depth as u32).pow(2);
    }

    fn store_transposition(&mut self, zobrist: u64, depth: u8, evaluation: i32, best_move: Option<Move>, node_type: NodeType) {
        let entry = TranspositionEntry {
            zobrist_hash: zobrist,
            depth,
            evaluation,
            best_move,
            node_type,
            age: self.age,
        };
        
        // Simple replacement scheme - could be improved
        self.transposition_table.insert(zobrist, entry);
    }

    fn is_check(&self, _position: &Position) -> bool {
        // This would need to be implemented based on your position structure
        false // Placeholder
    }

    fn is_draw(&self, position: &Position) -> bool {
        // Check for 50-move rule, threefold repetition, insufficient material, etc.
        position.halfmove_clock() >= 100 // 50-move rule
    }

    fn has_non_pawn_pieces(&self, _position: &Position) -> bool {
        // Check if the side to move has pieces other than pawns and king
        true // Placeholder - would need actual implementation
    }

    fn should_stop(&self) -> bool {
        if let Some(max_time) = self.config.max_time {
            if self.start_time.elapsed() >= max_time {
                return true;
            }
        }

        if let Some(max_nodes) = self.config.max_nodes {
            if self.nodes_searched >= max_nodes {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_config_default() {
        let config = SearchConfig::default();
        assert_eq!(config.max_depth, 8);
        assert!(config.use_null_move_pruning);
        assert!(config.use_late_move_reductions);
    }

    #[test]
    fn test_piece_values() {
        assert_eq!(SearchEngine::piece_value(PieceType::Pawn), 100);
        assert_eq!(SearchEngine::piece_value(PieceType::Queen), 900);
        assert_eq!(SearchEngine::piece_value(PieceType::King), 20000);
    }
}