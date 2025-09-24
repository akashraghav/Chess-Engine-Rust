#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChessError {
    InvalidMove(String),
    InvalidPosition(String),
    ParseError(String),
    GameOver(String),
}

impl std::fmt::Display for ChessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessError::InvalidMove(msg) => write!(f, "Invalid move: {}", msg),
            ChessError::InvalidPosition(msg) => write!(f, "Invalid position: {}", msg),
            ChessError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ChessError::GameOver(msg) => write!(f, "Game over: {}", msg),
        }
    }
}

impl std::error::Error for ChessError {}

pub type Result<T> = std::result::Result<T, ChessError>;
