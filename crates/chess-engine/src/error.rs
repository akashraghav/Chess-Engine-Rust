use chess_core::ChessError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    ChessError(ChessError),
    ConfigurationError(String),
    InvalidState(String),
    NotInitialized,
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::ChessError(err) => write!(f, "Chess error: {}", err),
            EngineError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            EngineError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            EngineError::NotInitialized => write!(f, "Engine not initialized"),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<ChessError> for EngineError {
    fn from(err: ChessError) -> Self {
        EngineError::ChessError(err)
    }
}

pub type Result<T> = std::result::Result<T, EngineError>;
