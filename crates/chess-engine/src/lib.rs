pub mod builder;
pub mod engine;
pub mod error;
pub mod event;

pub use chess_core::{
    Bitboard, CastlingRights, Color, Evaluator, GameResult, Move, MoveType, Piece, PieceType,
    Position, Square,
};

pub use builder::ChessEngineBuilder;
pub use engine::{ChessEngine, EngineConfig};
pub use error::{EngineError, Result};
pub use event::{EventHandler, GameEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveResult {
    pub success: bool,
    pub game_result: Option<GameResult>,
    pub events: Vec<GameEvent>,
}

#[derive(Debug, Clone)]
pub struct GameInfo {
    pub fen: String,
    pub side_to_move: Color,
    pub legal_moves: Vec<Move>,
    pub is_check: bool,
    pub is_checkmate: bool,
    pub is_stalemate: bool,
    pub is_draw: bool,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub position_count: usize,
}
