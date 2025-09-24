// Chess Engine Core Library
// Modular, well-organized chess engine implementation

pub mod board;
pub mod pieces;
pub mod moves;
pub mod game;
pub mod search;
pub mod evaluation;
pub mod utils;
pub mod error;

// Re-export commonly used types
pub use board::{Bitboard, Position, UndoInfo, Square};
pub use pieces::{Color, Piece, PieceType};
pub use moves::{Move, MoveType, MoveGenerator};
pub use game::{GameState, CastlingRights, GameResult};
pub use search::{SearchEngine, SearchConfig, SearchResult, ParallelConfig, ParallelSearchEngine};
pub use evaluation::{Evaluator, OptimizedEvaluator, EvaluationCache, GamePhase};
pub use utils::{OptimizedBitboard, SimdBitboard, MemoryManager, TranspositionTable, MovePool, OptimizedMoveList, MemoryConfig, MemoryStats};
pub use error::{ChessError, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_library_exports() {
        // Basic smoke test to ensure all exports work
        let _position = Position::new();
        let _game_state = GameState::new();
        let _evaluator = Evaluator::new();
    }
}