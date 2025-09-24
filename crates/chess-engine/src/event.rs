use crate::{Move, GameResult, Color, Square, Piece};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameEvent {
    GameStarted,
    MoveMade {
        mv: Move,
        san: String,
        fen: String,
    },
    PieceCaptured {
        piece: Piece,
        square: Square,
    },
    Check {
        color: Color,
    },
    Checkmate {
        winner: Color,
    },
    Stalemate,
    Draw {
        reason: DrawReason,
    },
    Promotion {
        piece: Piece,
        square: Square,
    },
    Castle {
        color: Color,
        side: CastleSide,
    },
    EnPassant {
        captured_square: Square,
    },
    GameEnded {
        result: GameResult,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawReason {
    Stalemate,
    FiftyMoveRule,
    ThreefoldRepetition,
    InsufficientMaterial,
    Agreement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleSide {
    Kingside,
    Queenside,
}

pub trait EventHandler: Send + Sync {
    fn handle_event(&mut self, event: &GameEvent);
}

pub struct DefaultEventHandler {
    events: Vec<GameEvent>,
}

impl DefaultEventHandler {
    pub fn new() -> Self {
        DefaultEventHandler {
            events: Vec::new(),
        }
    }

    pub fn get_events(&self) -> &[GameEvent] {
        &self.events
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}

impl EventHandler for DefaultEventHandler {
    fn handle_event(&mut self, event: &GameEvent) {
        self.events.push(event.clone());
    }
}

impl Default for DefaultEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LoggingEventHandler;

impl EventHandler for LoggingEventHandler {
    fn handle_event(&mut self, event: &GameEvent) {
        println!("Chess Event: {:?}", event);
    }
}