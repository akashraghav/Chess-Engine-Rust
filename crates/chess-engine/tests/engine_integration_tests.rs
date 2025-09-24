#[cfg(test)]
mod engine_integration_tests {
    use chess_engine::{ChessEngine, Result};

    #[test]
    fn test_engine_initialization() -> Result<()> {
        let mut engine = ChessEngine::new();
        engine.initialize()?;

        // Engine should start with starting position
        let fen = engine.get_fen();
        assert!(fen.contains("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));

        Ok(())
    }

    #[test]
    fn test_fen_loading_and_evaluation_integration() -> Result<()> {
        let mut engine = ChessEngine::new();
        engine.initialize()?;

        // Test starting position
        let starting_eval = engine.evaluate();
        assert!(
            starting_eval.abs() <= 50,
            "Starting position should be near 0"
        );

        // Load position with white queen advantage
        engine.load_fen("4k3/8/8/8/8/8/8/3QK3 w - - 0 1")?;
        let queen_eval = engine.evaluate();
        assert!(
            queen_eval > 800,
            "White queen advantage should be significant"
        );

        // Load position with black queen advantage
        engine.load_fen("3qk3/8/8/8/8/8/8/4K3 w - - 0 1")?;
        let black_queen_eval = engine.evaluate();
        assert!(
            black_queen_eval < -800,
            "Black queen advantage should be negative"
        );

        Ok(())
    }

    #[test]
    fn test_multiple_fen_loads() -> Result<()> {
        let mut engine = ChessEngine::new();
        engine.initialize()?;

        // Load multiple different positions and verify evaluations change
        let positions_and_expected = vec![
            ("4k3/8/8/8/8/8/8/4K3 w - - 0 1", -50..50), // Kings only - near 0
            ("4k3/8/8/8/8/8/8/3QK3 w - - 0 1", 800..1200), // White queen advantage
            ("3qk3/8/8/8/8/8/8/4K3 w - - 0 1", -1200..-800), // Black queen advantage
            ("4k3/8/8/8/8/8/8/R3K3 w - - 0 1", 400..700), // White rook advantage
        ];

        for (fen, expected_range) in positions_and_expected {
            engine.load_fen(fen)?;
            let eval = engine.evaluate();
            assert!(
                expected_range.contains(&eval),
                "FEN '{}' should evaluate in range {:?}, got {}",
                fen,
                expected_range,
                eval
            );
        }

        Ok(())
    }

    #[test]
    fn test_engine_from_fen_constructor() -> Result<()> {
        // Test creating engine directly from FEN
        let engine = ChessEngine::from_fen("4k3/8/8/8/8/8/8/3QK3 w - - 0 1")?;
        let eval = engine.evaluate();

        assert!(
            eval > 800,
            "Engine created from FEN should evaluate queen advantage correctly"
        );

        Ok(())
    }

    #[test]
    fn test_tactical_awareness_integration() -> Result<()> {
        let mut engine = ChessEngine::new();
        engine.initialize()?;

        // Test the same positions used in basic_tactics example
        // Position with material imbalance should be detected correctly
        engine.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKB1R w KQkq - 0 1")?; // Missing white knight
        let eval_missing_knight = engine.evaluate();

        // Load balanced position
        engine.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
        let eval_balanced = engine.evaluate();

        // Missing knight should result in worse evaluation for white
        assert!(
            eval_missing_knight < eval_balanced,
            "Missing piece should worsen evaluation: {} vs {}",
            eval_missing_knight,
            eval_balanced
        );

        // The difference should be roughly a knight's value (320)
        let diff = eval_balanced - eval_missing_knight;
        assert!(
            diff > 250 && diff < 400,
            "Missing knight should cost ~320 points, actual diff: {}",
            diff
        );

        Ok(())
    }

    #[test]
    fn test_evaluation_sign_convention() -> Result<()> {
        let mut engine = ChessEngine::new();
        engine.initialize()?;

        // Test that evaluation sign is consistent with side to move
        let white_advantage_fen = "4k3/8/8/8/8/8/8/3QK3 w - - 0 1";
        let black_advantage_fen = "3qk3/8/8/8/8/8/8/4K3 w - - 0 1";

        engine.load_fen(white_advantage_fen)?;
        let white_eval = engine.evaluate();

        engine.load_fen(black_advantage_fen)?;
        let black_eval = engine.evaluate();

        // White advantage should be positive, black advantage should be negative
        assert!(
            white_eval > 0,
            "White advantage should be positive: {}",
            white_eval
        );
        assert!(
            black_eval < 0,
            "Black advantage should be negative: {}",
            black_eval
        );

        // Should be roughly symmetric
        let symmetry_diff = (white_eval + black_eval).abs();
        assert!(
            symmetry_diff < 100,
            "Evaluations should be roughly symmetric, diff: {}",
            symmetry_diff
        );

        Ok(())
    }

    #[test]
    fn test_engine_error_handling() {
        let mut engine = ChessEngine::new();

        // Should fail to load invalid FEN
        let result = engine.load_fen("invalid_fen_string");
        assert!(result.is_err(), "Invalid FEN should return error");

        // Should fail to load FEN with wrong number of ranks
        let result = engine.load_fen("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!(
            result.is_err(),
            "FEN with wrong rank count should return error"
        );
    }

    #[test]
    fn test_engine_state_isolation() -> Result<()> {
        // Test that multiple engines don't interfere with each other
        let mut engine1 = ChessEngine::new();
        let mut engine2 = ChessEngine::new();

        engine1.initialize()?;
        engine2.initialize()?;

        // Load different positions
        engine1.load_fen("4k3/8/8/8/8/8/8/3QK3 w - - 0 1")?;
        engine2.load_fen("3qk3/8/8/8/8/8/8/4K3 w - - 0 1")?;

        let eval1 = engine1.evaluate();
        let eval2 = engine2.evaluate();

        // Should have different evaluations
        assert!(eval1 > 0, "Engine 1 should have positive evaluation");
        assert!(eval2 < 0, "Engine 2 should have negative evaluation");
        assert!(
            (eval1 - eval2).abs() > 1000,
            "Engines should have significantly different evaluations"
        );

        Ok(())
    }

    #[test]
    fn test_evaluation_after_multiple_loads() -> Result<()> {
        let mut engine = ChessEngine::new();
        engine.initialize()?;

        // Load multiple positions in sequence and verify each evaluation is correct
        let test_positions = vec![
            ("4k3/8/8/8/8/8/8/4K3 w - - 0 1", "kings_only"),
            ("4k3/8/8/8/8/8/8/3QK3 w - - 0 1", "white_queen"),
            ("4k3/8/8/8/8/8/8/4K3 w - - 0 1", "kings_only_again"),
            ("3qk3/8/8/8/8/8/8/4K3 w - - 0 1", "black_queen"),
        ];

        let mut evaluations = Vec::new();

        for (fen, description) in &test_positions {
            engine.load_fen(fen)?;
            let eval = engine.evaluate();
            evaluations.push((eval, description));
        }

        // Verify expected patterns
        assert!(
            evaluations[0].0.abs() <= 50,
            "Kings only should be near 0: {}",
            evaluations[0].0
        );
        assert!(
            evaluations[1].0 > 800,
            "White queen should be positive: {}",
            evaluations[1].0
        );
        assert!(
            evaluations[2].0.abs() <= 50,
            "Kings only again should be near 0: {}",
            evaluations[2].0
        );
        assert!(
            evaluations[3].0 < -800,
            "Black queen should be negative: {}",
            evaluations[3].0
        );

        // First and third should be the same (same position)
        let kings_diff = (evaluations[0].0 - evaluations[2].0).abs();
        assert!(
            kings_diff <= 5,
            "Same position should give same evaluation: diff {}",
            kings_diff
        );

        Ok(())
    }
}
