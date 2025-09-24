use crate::{Result, EngineError, GameEvent, EventHandler, MoveResult, GameInfo, event::DefaultEventHandler};
use chess_core::{
    GameState, Move, Color, GameResult, Evaluator, MoveGenerator, Position, SearchEngine, SearchConfig, Square, Piece
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub depth: u8,
    pub time_limit_ms: Option<u64>,
    pub enable_transposition_table: bool,
    pub transposition_table_size: usize,
    pub enable_book: bool,
    pub thread_count: usize,
    pub debug_mode: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        EngineConfig {
            depth: 6,
            time_limit_ms: None,
            enable_transposition_table: true,
            transposition_table_size: 1_000_000,
            enable_book: false,
            thread_count: 1,
            debug_mode: false,
        }
    }
}

pub struct ChessEngine {
    game_state: GameState,
    config: EngineConfig,
    evaluator: Evaluator,
    #[allow(dead_code)]
    move_generator: MoveGenerator,
    search_engine: SearchEngine,
    event_handler: Arc<Mutex<dyn EventHandler>>,
    initialized: bool,
}

impl ChessEngine {
    pub fn new() -> Self {
        ChessEngine {
            game_state: GameState::new(),
            config: EngineConfig::default(),
            evaluator: Evaluator::new(),
            move_generator: MoveGenerator::new(),
            search_engine: SearchEngine::new(SearchConfig::default()),
            event_handler: Arc::new(Mutex::new(DefaultEventHandler::new())),
            initialized: false,
        }
    }

    pub fn with_config(config: EngineConfig) -> Self {
        ChessEngine {
            game_state: GameState::new(),
            config,
            evaluator: Evaluator::new(),
            move_generator: MoveGenerator::new(),
            search_engine: SearchEngine::new(SearchConfig::default()),
            event_handler: Arc::new(Mutex::new(DefaultEventHandler::new())),
            initialized: false,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self> {
        let game_state = GameState::from_fen(fen)?;
        Ok(ChessEngine {
            game_state,
            config: EngineConfig::default(),
            evaluator: Evaluator::new(),
            move_generator: MoveGenerator::new(),
            search_engine: SearchEngine::new(SearchConfig::default()),
            event_handler: Arc::new(Mutex::new(DefaultEventHandler::new())),
            initialized: false,
        })
    }

    pub fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Err(EngineError::InvalidState("Engine already initialized".to_string()));
        }

        self.emit_event(GameEvent::GameStarted);
        self.initialized = true;
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn set_event_handler(&mut self, handler: Arc<Mutex<dyn EventHandler>>) {
        self.event_handler = handler;
    }

    pub fn get_config(&self) -> &EngineConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: EngineConfig) -> Result<()> {
        if self.initialized {
            return Err(EngineError::InvalidState("Cannot change config after initialization".to_string()));
        }
        self.config = config;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.game_state = GameState::new();
        self.emit_event(GameEvent::GameStarted);
        Ok(())
    }

    pub fn load_fen(&mut self, fen: &str) -> Result<()> {
        self.game_state = GameState::from_fen(fen)?;
        Ok(())
    }

    pub fn get_fen(&self) -> String {
        self.game_state.to_fen()
    }

    pub fn get_position(&self) -> &Position {
        &self.game_state.position
    }

    pub fn get_side_to_move(&self) -> Color {
        self.game_state.position.side_to_move
    }

    pub fn make_move(&mut self, mv: Move) -> Result<MoveResult> {
        if !self.initialized {
            return Err(EngineError::NotInitialized);
        }

        let mut events = Vec::new();

        if !self.is_legal_move(mv) {
            return Ok(MoveResult {
                success: false,
                game_result: None,
                events,
            });
        }

        let captured_piece = self.game_state.position.piece_at(mv.to);
        let _is_check_before = self.game_state.is_in_check(self.game_state.position.side_to_move);

        self.game_state.make_move(mv)?;

        let san = self.move_to_san(mv);
        events.push(GameEvent::MoveMade {
            mv,
            san,
            fen: self.get_fen(),
        });

        if let Some(piece) = captured_piece {
            events.push(GameEvent::PieceCaptured {
                piece,
                square: mv.to,
            });
        }

        if mv.is_promotion() {
            if let Some(promotion_piece) = mv.promotion_piece() {
                let promoted_piece = Piece::new(
                    promotion_piece,
                    self.game_state.position.side_to_move.opposite()
                );
                events.push(GameEvent::Promotion {
                    piece: promoted_piece,
                    square: mv.to,
                });
            }
        }

        if mv.is_castle() {
            let side = match (mv.from, mv.to) {
                (Square::E1, Square::G1) |
                (Square::E8, Square::G8) =>
                    crate::event::CastleSide::Kingside,
                _ => crate::event::CastleSide::Queenside,
            };
            events.push(GameEvent::Castle {
                color: self.game_state.position.side_to_move.opposite(),
                side,
            });
        }

        if mv.is_en_passant() {
            let captured_square = match self.game_state.position.side_to_move {
                Color::White => Square::from_file_rank(mv.to.file(), mv.to.rank() + 1),
                Color::Black => Square::from_file_rank(mv.to.file(), mv.to.rank() - 1),
            }.unwrap();
            events.push(GameEvent::EnPassant { captured_square });
        }

        let is_check_after = self.game_state.is_in_check(self.game_state.position.side_to_move);
        if is_check_after {
            events.push(GameEvent::Check {
                color: self.game_state.position.side_to_move,
            });
        }

        let game_result = self.game_state.game_result();
        let mut final_game_result = None;

        match game_result {
            GameResult::WhiteWins => {
                events.push(GameEvent::Checkmate { winner: Color::White });
                events.push(GameEvent::GameEnded { result: game_result });
                final_game_result = Some(game_result);
            }
            GameResult::BlackWins => {
                events.push(GameEvent::Checkmate { winner: Color::Black });
                events.push(GameEvent::GameEnded { result: game_result });
                final_game_result = Some(game_result);
            }
            GameResult::Draw => {
                let draw_reason = if self.game_state.is_stalemate() {
                    crate::event::DrawReason::Stalemate
                } else if self.game_state.is_fifty_move_rule() {
                    crate::event::DrawReason::FiftyMoveRule
                } else if self.game_state.is_threefold_repetition() {
                    crate::event::DrawReason::ThreefoldRepetition
                } else if self.game_state.is_insufficient_material() {
                    crate::event::DrawReason::InsufficientMaterial
                } else {
                    crate::event::DrawReason::Agreement
                };

                if draw_reason == crate::event::DrawReason::Stalemate {
                    events.push(GameEvent::Stalemate);
                }

                events.push(GameEvent::Draw { reason: draw_reason });
                events.push(GameEvent::GameEnded { result: game_result });
                final_game_result = Some(game_result);
            }
            GameResult::Ongoing => {}
        }

        for event in &events {
            self.emit_event(event.clone());
        }

        Ok(MoveResult {
            success: true,
            game_result: final_game_result,
            events,
        })
    }

    pub fn make_move_from_uci(&mut self, uci: &str) -> Result<MoveResult> {
        let mv: Move = uci.parse()?;
        self.make_move(mv)
    }

    pub fn make_move_from_san(&mut self, san: &str) -> Result<MoveResult> {
        let mv = self.parse_san_move(san)?;
        self.make_move(mv)
    }

    pub fn is_legal_move(&self, mv: Move) -> bool {
        self.game_state.is_legal_move(mv)
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        self.game_state.generate_legal_moves()
    }

    pub fn get_game_info(&self) -> GameInfo {
        let legal_moves = self.get_legal_moves();
        GameInfo {
            fen: self.get_fen(),
            side_to_move: self.get_side_to_move(),
            legal_moves: legal_moves.clone(),
            is_check: self.game_state.is_in_check(self.get_side_to_move()),
            is_checkmate: self.game_state.is_checkmate(),
            is_stalemate: self.game_state.is_stalemate(),
            is_draw: self.game_state.is_draw(),
            halfmove_clock: self.game_state.halfmove_clock,
            fullmove_number: self.game_state.fullmove_number,
            position_count: self.game_state.position_history.len(),
        }
    }

    pub fn evaluate(&self) -> i32 {
        self.evaluator.evaluate(&self.game_state)
    }

    pub fn find_best_move(&mut self) -> Result<Option<Move>> {
        if !self.initialized {
            return Err(EngineError::NotInitialized);
        }

        let legal_moves = self.get_legal_moves();
        if legal_moves.is_empty() {
            return Ok(None);
        }

        // Use the search engine to find the best move
        let search_result = self.search_engine.search(&self.game_state.position);
        Ok(search_result.best_move)
    }

    pub fn get_game_result(&self) -> GameResult {
        self.game_state.game_result()
    }

    pub fn is_game_over(&self) -> bool {
        self.get_game_result().is_game_over()
    }

    fn move_to_san(&self, mv: Move) -> String {
        format!("{}", mv)
    }

    fn parse_san_move(&self, _san: &str) -> Result<Move> {
        Err(EngineError::InvalidState("SAN parsing not yet implemented".to_string()))
    }

    fn emit_event(&self, event: GameEvent) {
        if let Ok(mut handler) = self.event_handler.try_lock() {
            handler.handle_event(&event);
        }
    }
}

impl Default for ChessEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = ChessEngine::new();
        assert!(!engine.is_initialized());
        assert_eq!(engine.get_side_to_move(), Color::White);
    }

    #[test]
    fn test_engine_initialization() {
        let mut engine = ChessEngine::new();
        assert!(engine.initialize().is_ok());
        assert!(engine.is_initialized());
    }

    #[test]
    fn test_engine_from_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let engine = ChessEngine::from_fen(fen).unwrap();
        assert_eq!(engine.get_fen(), fen);
    }

    #[test]
    fn test_make_move() {
        let mut engine = ChessEngine::new();
        engine.initialize().unwrap();

        let mv = Move::normal(Square::E2, Square::E4);
        let result = engine.make_move(mv).unwrap();

        assert!(result.success);
        assert_eq!(engine.get_side_to_move(), Color::Black);
    }

    #[test]
    fn test_legal_moves() {
        let engine = ChessEngine::new();
        let legal_moves = engine.get_legal_moves();
        assert_eq!(legal_moves.len(), 20);
    }

    #[test]
    fn test_game_info() {
        let engine = ChessEngine::new();
        let info = engine.get_game_info();

        assert_eq!(info.side_to_move, Color::White);
        assert_eq!(info.legal_moves.len(), 20);
        assert!(!info.is_check);
        assert!(!info.is_checkmate);
        assert!(!info.is_stalemate);
        assert!(!info.is_draw);
    }

    #[test]
    fn test_evaluation() {
        let engine = ChessEngine::new();
        let score = engine.evaluate();
        // The evaluation may be 0 or some positive value depending on the implementation
        // As long as it's consistent and not negative, it's acceptable
        assert!(score >= 0, "Starting position evaluation should be non-negative: {}", score);
    }
}