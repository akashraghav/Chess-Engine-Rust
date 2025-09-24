#[cfg(test)]
mod evaluation_regression_tests {
    use chess_core::{Evaluator, GameState, PieceType};

    /// Test known evaluation values to prevent regressions
    #[test]
    fn test_starting_position_evaluation() {
        let game_state = GameState::new();
        let evaluator = Evaluator::new();
        let eval = evaluator.evaluate(&game_state);

        // Starting position should be roughly equal (small positional differences allowed)
        assert!(
            eval.abs() <= 50,
            "Starting position should be near 0, got {}",
            eval
        );
    }

    #[test]
    fn test_material_advantage_white_queen() {
        let fen = "4k3/8/8/8/8/8/8/3QK3 w - - 0 1"; // White has queen vs just kings
        let game_state = GameState::from_fen(fen).expect("Valid FEN");
        let evaluator = Evaluator::new();
        let eval = evaluator.evaluate(&game_state);

        // Should be positive (good for white) and approximately queen value (900) + positional
        assert!(
            eval > 800,
            "White queen advantage should be > 800, got {}",
            eval
        );
        assert!(
            eval < 1200,
            "White queen advantage should be < 1200, got {}",
            eval
        );
    }

    #[test]
    fn test_material_advantage_black_queen() {
        let fen = "3qk3/8/8/8/8/8/8/4K3 w - - 0 1"; // Black has queen vs just kings
        let game_state = GameState::from_fen(fen).expect("Valid FEN");
        let evaluator = Evaluator::new();
        let eval = evaluator.evaluate(&game_state);

        // Should be negative (bad for white) and approximately -queen value
        assert!(
            eval < -800,
            "Black queen advantage should be < -800, got {}",
            eval
        );
        assert!(
            eval > -1200,
            "Black queen advantage should be > -1200, got {}",
            eval
        );
    }

    #[test]
    fn test_material_advantage_white_rook() {
        let fen = "4k3/8/8/8/8/8/8/R3K3 w - - 0 1"; // White has rook vs just kings
        let game_state = GameState::from_fen(fen).expect("Valid FEN");
        let evaluator = Evaluator::new();
        let eval = evaluator.evaluate(&game_state);

        // Should be positive and approximately rook value (500) + positional
        assert!(
            eval > 400,
            "White rook advantage should be > 400, got {}",
            eval
        );
        assert!(
            eval < 700,
            "White rook advantage should be < 700, got {}",
            eval
        );
    }

    #[test]
    fn test_material_advantage_symmetry() {
        let white_queen_fen = "4k3/8/8/8/8/8/8/3QK3 w - - 0 1";
        let black_queen_fen = "3qk3/8/8/8/8/8/8/4K3 w - - 0 1";

        let white_state = GameState::from_fen(white_queen_fen).expect("Valid FEN");
        let black_state = GameState::from_fen(black_queen_fen).expect("Valid FEN");

        let evaluator = Evaluator::new();
        let white_eval = evaluator.evaluate(&white_state);
        let black_eval = evaluator.evaluate(&black_state);

        // Should be roughly symmetric (opposite signs, similar magnitude)
        let diff = (white_eval + black_eval).abs();
        assert!(
            diff <= 50,
            "Evaluations should be symmetric within 50 points: white={}, black={}, diff={}",
            white_eval,
            black_eval,
            diff
        );
    }

    #[test]
    fn test_piece_value_ordering() {
        let evaluator = Evaluator::new();

        // Test individual piece advantages
        let queen_fen = "4k3/8/8/8/8/8/8/3QK3 w - - 0 1";
        let rook_fen = "4k3/8/8/8/8/8/8/3RK3 w - - 0 1";
        let bishop_fen = "4k3/8/8/8/8/8/8/3BK3 w - - 0 1";
        let knight_fen = "4k3/8/8/8/8/8/8/3NK3 w - - 0 1";
        let pawn_fen = "4k3/8/8/8/8/8/8/3PK3 w - - 0 1";

        let queen_eval = evaluator.evaluate(&GameState::from_fen(queen_fen).unwrap());
        let rook_eval = evaluator.evaluate(&GameState::from_fen(rook_fen).unwrap());
        let bishop_eval = evaluator.evaluate(&GameState::from_fen(bishop_fen).unwrap());
        let knight_eval = evaluator.evaluate(&GameState::from_fen(knight_fen).unwrap());
        let pawn_eval = evaluator.evaluate(&GameState::from_fen(pawn_fen).unwrap());

        // Verify piece value ordering: Queen > Rook > Bishop ≈ Knight > Pawn
        assert!(
            queen_eval > rook_eval,
            "Queen should be worth more than rook"
        );
        assert!(
            rook_eval > bishop_eval,
            "Rook should be worth more than bishop"
        );
        assert!(
            rook_eval > knight_eval,
            "Rook should be worth more than knight"
        );
        assert!(
            bishop_eval > pawn_eval,
            "Bishop should be worth more than pawn"
        );
        assert!(
            knight_eval > pawn_eval,
            "Knight should be worth more than pawn"
        );

        // Bishop and knight should have similar base values, but positional differences are expected
        // The raw piece values are Bishop=330, Knight=320 (diff=10)
        // But positional scoring can create larger differences depending on square
        let bishop_knight_diff = (bishop_eval - knight_eval).abs();
        assert!(
            bishop_knight_diff >= 10,
            "Bishop-knight difference should be at least base value diff (10)"
        );
        assert!(
            bishop_knight_diff <= 150,
            "Bishop-knight positional difference should be reasonable, got: {}",
            bishop_knight_diff
        );

        // More importantly, verify the base piece values are correct
        assert_eq!(
            PieceType::Bishop.value(),
            330,
            "Bishop base value should be 330"
        );
        assert_eq!(
            PieceType::Knight.value(),
            320,
            "Knight base value should be 320"
        );
    }

    #[test]
    fn test_evaluation_consistency() {
        // Same position should always give same evaluation
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let evaluator = Evaluator::new();

        let eval1 = evaluator.evaluate(&GameState::from_fen(fen).unwrap());
        let eval2 = evaluator.evaluate(&GameState::from_fen(fen).unwrap());
        let eval3 = evaluator.evaluate(&GameState::from_fen(fen).unwrap());

        assert_eq!(eval1, eval2, "Evaluation should be consistent");
        assert_eq!(eval2, eval3, "Evaluation should be consistent");
    }

    #[test]
    fn test_known_tactical_positions() {
        // Position where white has a clear material advantage (extra piece)
        let material_advantage_fen = "rnbqk2r/pppppppp/5n2/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let balanced_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let evaluator = Evaluator::new();
        let advantage_eval =
            evaluator.evaluate(&GameState::from_fen(material_advantage_fen).unwrap());
        let balanced_eval = evaluator.evaluate(&GameState::from_fen(balanced_fen).unwrap());

        // The position with material advantage should evaluate better for white
        assert!(
            advantage_eval > balanced_eval,
            "Material advantage position should evaluate better: {} vs {}",
            advantage_eval,
            balanced_eval
        );
    }

    #[test]
    fn test_evaluation_bounds() {
        // Extreme positions should have reasonable evaluation bounds
        let many_queens_fen = "QQQQQQQQ/8/8/8/8/8/8/4k2K w - - 0 1"; // Unrealistic but tests bounds
        let evaluator = Evaluator::new();

        let eval = evaluator.evaluate(&GameState::from_fen(many_queens_fen).unwrap());

        // Should be very positive but not infinite
        assert!(eval > 5000, "Many queens should have very high evaluation");
        assert!(eval < 100000, "Evaluation should not be unreasonably high");
    }

    #[test]
    fn test_positional_evaluation_makes_sense() {
        let evaluator = Evaluator::new();

        // Test that central squares are generally better for pieces
        let bishop_center =
            evaluator.evaluate(&GameState::from_fen("4k3/8/8/8/3B4/8/8/4K3 w - - 0 1").unwrap());
        let bishop_corner =
            evaluator.evaluate(&GameState::from_fen("4k3/8/8/8/8/8/8/B3K3 w - - 0 1").unwrap());
        let empty_pos =
            evaluator.evaluate(&GameState::from_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap());

        let bishop_center_bonus = bishop_center - empty_pos;
        let bishop_corner_bonus = bishop_corner - empty_pos;

        // Bishop should prefer center over corner (though both should be positive due to material)
        assert!(
            bishop_center_bonus > bishop_corner_bonus,
            "Bishop should prefer center (d4) over corner (a1): center={}, corner={}",
            bishop_center_bonus,
            bishop_corner_bonus
        );

        // Both should be positive (material advantage)
        assert!(
            bishop_center_bonus > 300,
            "Bishop in center should have significant value"
        );
        assert!(
            bishop_corner_bonus > 300,
            "Bishop in corner should still have material value"
        );
    }

    #[test]
    fn test_endgame_positions() {
        // King and pawn vs king - should favor the side with pawn
        let kp_vs_k_fen = "8/8/8/8/8/8/4P3/4K1k1 w - - 0 1";
        let k_vs_k_fen = "8/8/8/8/8/8/8/4K1k1 w - - 0 1";

        let evaluator = Evaluator::new();
        let kp_eval = evaluator.evaluate(&GameState::from_fen(kp_vs_k_fen).unwrap());
        let kk_eval = evaluator.evaluate(&GameState::from_fen(k_vs_k_fen).unwrap());

        assert!(
            kp_eval > kk_eval,
            "King+pawn should be better than just kings"
        );
        assert!(kp_eval > 50, "King+pawn advantage should be significant");
    }
}
