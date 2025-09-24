use crate::{ChessEngine, EngineConfig, EngineError, EventHandler, Result};
use std::sync::{Arc, Mutex};

pub struct ChessEngineBuilder {
    config: EngineConfig,
    fen: Option<String>,
    event_handler: Option<Arc<Mutex<dyn EventHandler>>>,
    auto_initialize: bool,
}

impl ChessEngineBuilder {
    pub fn new() -> Self {
        ChessEngineBuilder {
            config: EngineConfig::default(),
            fen: None,
            event_handler: None,
            auto_initialize: true,
        }
    }

    pub fn with_depth(mut self, mut depth: u8) -> Self {
        if depth == 0 {
            depth = 1;
        }
        self.config.depth = depth.min(20);
        self
    }

    pub fn with_time_limit(mut self, time_limit_ms: u64) -> Self {
        self.config.time_limit_ms = Some(time_limit_ms);
        self
    }

    pub fn with_transposition_table(mut self, enable: bool, size: Option<usize>) -> Self {
        self.config.enable_transposition_table = enable;
        if let Some(size) = size {
            self.config.transposition_table_size = size;
        }
        self
    }

    pub fn with_opening_book(mut self, enable: bool) -> Self {
        self.config.enable_book = enable;
        self
    }

    pub fn with_threads(mut self, thread_count: usize) -> Self {
        self.config.thread_count = thread_count.clamp(1, 16);
        self
    }

    pub fn with_debug_mode(mut self, enable: bool) -> Self {
        self.config.debug_mode = enable;
        self
    }

    pub fn from_fen(mut self, fen: &str) -> Self {
        self.fen = Some(fen.to_string());
        self
    }

    pub fn with_event_handler(mut self, handler: Arc<Mutex<dyn EventHandler>>) -> Self {
        self.event_handler = Some(handler);
        self
    }

    pub fn auto_initialize(mut self, enable: bool) -> Self {
        self.auto_initialize = enable;
        self
    }

    pub fn build(self) -> Result<ChessEngine> {
        let mut engine = if let Some(fen) = &self.fen {
            let mut engine = ChessEngine::from_fen(fen)?;
            engine.set_config(self.config)?;
            engine
        } else {
            ChessEngine::with_config(self.config)
        };

        if let Some(handler) = self.event_handler {
            engine.set_event_handler(handler);
        }

        if self.auto_initialize {
            engine.initialize()?;
        }

        Ok(engine)
    }

    pub fn build_and_validate(self) -> Result<ChessEngine> {
        let engine = self.build()?;

        if engine.get_config().depth > 15 && engine.get_config().time_limit_ms.is_none() {
            return Err(EngineError::ConfigurationError(
                "Deep search requires time limit for safety".to_string(),
            ));
        }

        if engine.get_config().thread_count > 1 && !engine.get_config().enable_transposition_table {
            return Err(EngineError::ConfigurationError(
                "Multi-threading requires transposition table".to_string(),
            ));
        }

        Ok(engine)
    }
}

impl Default for ChessEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{event::DefaultEventHandler, Color};

    #[test]
    fn test_builder_default() {
        let engine = ChessEngineBuilder::new().build().unwrap();
        assert!(engine.is_initialized());
        assert_eq!(engine.get_side_to_move(), Color::White);
    }

    #[test]
    fn test_builder_with_depth() {
        let engine = ChessEngineBuilder::new().with_depth(10).build().unwrap();
        assert_eq!(engine.get_config().depth, 10);
    }

    #[test]
    fn test_builder_from_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let engine = ChessEngineBuilder::new().from_fen(fen).build().unwrap();
        assert_eq!(engine.get_side_to_move(), Color::Black);
    }

    #[test]
    fn test_builder_with_time_limit() {
        let engine = ChessEngineBuilder::new()
            .with_time_limit(5000)
            .build()
            .unwrap();
        assert_eq!(engine.get_config().time_limit_ms, Some(5000));
    }

    #[test]
    fn test_builder_with_transposition_table() {
        let engine = ChessEngineBuilder::new()
            .with_transposition_table(true, Some(2_000_000))
            .build()
            .unwrap();
        assert!(engine.get_config().enable_transposition_table);
        assert_eq!(engine.get_config().transposition_table_size, 2_000_000);
    }

    #[test]
    fn test_builder_with_threads() {
        let engine = ChessEngineBuilder::new().with_threads(4).build().unwrap();
        assert_eq!(engine.get_config().thread_count, 4);
    }

    #[test]
    fn test_builder_no_auto_initialize() {
        let engine = ChessEngineBuilder::new()
            .auto_initialize(false)
            .build()
            .unwrap();
        assert!(!engine.is_initialized());
    }

    #[test]
    fn test_builder_with_event_handler() {
        let handler = Arc::new(Mutex::new(DefaultEventHandler::new()));
        let engine = ChessEngineBuilder::new()
            .with_event_handler(handler)
            .build()
            .unwrap();
        assert!(engine.is_initialized());
    }

    #[test]
    fn test_builder_validation_deep_search() {
        let result = ChessEngineBuilder::new()
            .with_depth(20)
            .build_and_validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_validation_multithreading() {
        let result = ChessEngineBuilder::new()
            .with_threads(4)
            .with_transposition_table(false, None)
            .build_and_validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_valid_configuration() {
        let engine = ChessEngineBuilder::new()
            .with_depth(8)
            .with_time_limit(10000)
            .with_threads(2)
            .with_transposition_table(true, Some(1_000_000))
            .with_debug_mode(true)
            .build_and_validate()
            .unwrap();

        assert_eq!(engine.get_config().depth, 8);
        assert_eq!(engine.get_config().time_limit_ms, Some(10000));
        assert_eq!(engine.get_config().thread_count, 2);
        assert!(engine.get_config().enable_transposition_table);
        assert!(engine.get_config().debug_mode);
    }
}
