// Chess Engine Core Library
// Modular, well-organized chess engine implementation

pub mod board;
pub mod error;
pub mod evaluation;
pub mod game;
pub mod moves;
pub mod pieces;
pub mod search;
pub mod utils;

// Re-export commonly used types
pub use board::{Bitboard, Position, Square, UndoInfo};
pub use error::{ChessError, Result};
pub use evaluation::{EvaluationCache, Evaluator, GamePhase, OptimizedEvaluator};
pub use game::{CastlingRights, GameResult, GameState};
pub use moves::{Move, MoveGenerator, MoveType};
pub use pieces::{Color, Piece, PieceType};
pub use search::{ParallelConfig, ParallelSearchEngine, SearchConfig, SearchEngine, SearchResult};
pub use utils::{
    MemoryConfig, MemoryManager, MemoryStats, MovePool, OptimizedBitboard, OptimizedMoveList,
    SimdBitboard, TranspositionTable,
};

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
