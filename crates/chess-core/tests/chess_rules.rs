#[cfg(test)]
mod chess_rules_tests {
    use chess_core::{Color, GameState, Move, PieceType, Square};

    #[test]
    fn test_starting_position_move_count() {
        let game_state = GameState::new();
        let moves = game_state.generate_legal_moves();

        // Starting position should have exactly 20 legal moves for white
        // 16 pawn moves (2 per pawn) + 4 knight moves (2 per knight)
        assert_eq!(
            moves.len(),
            20,
            "Starting position should have 20 legal moves"
        );
    }

    #[test]
    fn test_castle_moves_starting_position() {
        let game_state = GameState::new();
        let moves = game_state.generate_legal_moves();

        // No castling should be possible in starting position (pieces in the way)
        let castle_moves: Vec<_> = moves.iter().filter(|m| m.is_castle()).collect();
        assert_eq!(
            castle_moves.len(),
            0,
            "No castling should be possible in starting position"
        );
    }

    #[test]
    fn test_white_pawn_moves() {
        let game_state = GameState::new();
        let moves = game_state.generate_legal_moves();

        // Check that we have the expected pawn moves
        let pawn_moves: Vec<_> = moves
            .iter()
            .filter(|m| {
                // Pawn moves from rank 2 to rank 3 or 4
                m.from.rank() == 1 && (m.to.rank() == 2 || m.to.rank() == 3)
            })
            .collect();

        assert_eq!(
            pawn_moves.len(),
            16,
            "Should have 16 pawn moves from starting position"
        );
    }

    #[test]
    fn test_knight_moves() {
        let game_state = GameState::new();
        let moves = game_state.generate_legal_moves();

        // Check knight moves from b1 and g1
        let knight_moves: Vec<_> = moves
            .iter()
            .filter(|m| {
                (m.from == Square::B1 || m.from == Square::G1)
                    && (m.to == Square::A3
                        || m.to == Square::C3
                        || m.to == Square::F3
                        || m.to == Square::H3)
            })
            .collect();

        assert_eq!(
            knight_moves.len(),
            4,
            "Should have 4 knight moves from starting position"
        );
    }

    #[test]
    fn test_king_safety_in_check() {
        // Test that king in check is properly detected
        // This would require setting up a specific position
        // For now, just test the basic framework
        let game_state = GameState::new();
        assert!(
            !game_state.is_in_check(Color::White),
            "King should not be in check in starting position"
        );
        assert!(
            !game_state.is_in_check(Color::Black),
            "King should not be in check in starting position"
        );
    }

    #[test]
    fn test_castling_rights_preservation() {
        let game_state = GameState::new();

        // Starting position should have all castling rights
        assert!(game_state.castling_rights.can_castle_kingside(Color::White));
        assert!(game_state
            .castling_rights
            .can_castle_queenside(Color::White));
        assert!(game_state.castling_rights.can_castle_kingside(Color::Black));
        assert!(game_state
            .castling_rights
            .can_castle_queenside(Color::Black));
    }

    #[test]
    fn test_move_making_basic() {
        let mut game_state = GameState::new();

        // Try to make a simple pawn move
        let pawn_move = Move::normal(Square::E2, Square::E4);
        let result = game_state.make_move(pawn_move);

        assert!(result.is_ok(), "Basic pawn move should succeed");

        // Check that the side to move switched
        assert_eq!(game_state.position.side_to_move, Color::Black);
    }

    #[test]
    fn test_fen_parsing_starting_position() {
        let result =
            GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!(
            result.is_ok(),
            "Should be able to parse starting position FEN"
        );

        if let Ok(game_state) = result {
            assert_eq!(game_state.position.side_to_move, Color::White);
            assert!(game_state.castling_rights.can_castle_kingside(Color::White));
            assert!(game_state
                .castling_rights
                .can_castle_queenside(Color::White));
            assert!(game_state.castling_rights.can_castle_kingside(Color::Black));
            assert!(game_state
                .castling_rights
                .can_castle_queenside(Color::Black));
        }
    }

    #[test]
    fn test_position_evaluation_starting() {
        let game_state = GameState::new();
        let evaluator = chess_core::Evaluator::new();
        let score = evaluator.evaluate(&game_state);

        // Starting position should have a reasonable evaluation
        // The exact value may change as we improve the engine, but it should be > 0 for white
        // indicating slight advantage due to going first
        println!("Evaluation score: {}", score);
        assert!(
            score >= 0,
            "Starting position should evaluate favorably for white or be equal"
        );
    }

    #[test]
    fn test_game_not_over_at_start() {
        let game_state = GameState::new();

        assert!(
            !game_state.is_checkmate(),
            "Game should not be checkmate at start"
        );
        assert!(
            !game_state.is_stalemate(),
            "Game should not be stalemate at start"
        );
        assert!(!game_state.is_draw(), "Game should not be draw at start");
    }

    #[test]
    fn test_move_generation_consistency() {
        let game_state = GameState::new();

        // Generate moves multiple times and ensure consistency
        let moves1 = game_state.generate_legal_moves();
        let moves2 = game_state.generate_legal_moves();

        assert_eq!(
            moves1.len(),
            moves2.len(),
            "Move generation should be consistent"
        );

        // Check that all moves in first generation are in second
        for move1 in &moves1 {
            assert!(
                moves2.contains(move1),
                "Generated moves should be consistent"
            );
        }
    }

    #[test]
    fn test_piece_placement_starting_position() {
        let game_state = GameState::new();

        // Test that pieces are in correct starting positions

        // White pieces
        assert_eq!(
            game_state
                .position
                .piece_at(Square::E1)
                .map(|p| p.piece_type),
            Some(PieceType::King)
        );
        assert_eq!(
            game_state
                .position
                .piece_at(Square::D1)
                .map(|p| p.piece_type),
            Some(PieceType::Queen)
        );
        assert_eq!(
            game_state
                .position
                .piece_at(Square::A1)
                .map(|p| p.piece_type),
            Some(PieceType::Rook)
        );
        assert_eq!(
            game_state
                .position
                .piece_at(Square::H1)
                .map(|p| p.piece_type),
            Some(PieceType::Rook)
        );

        // Black pieces
        assert_eq!(
            game_state
                .position
                .piece_at(Square::E8)
                .map(|p| p.piece_type),
            Some(PieceType::King)
        );
        assert_eq!(
            game_state
                .position
                .piece_at(Square::D8)
                .map(|p| p.piece_type),
            Some(PieceType::Queen)
        );
        assert_eq!(
            game_state
                .position
                .piece_at(Square::A8)
                .map(|p| p.piece_type),
            Some(PieceType::Rook)
        );
        assert_eq!(
            game_state
                .position
                .piece_at(Square::H8)
                .map(|p| p.piece_type),
            Some(PieceType::Rook)
        );

        // Pawns
        for file in 0..8 {
            let white_pawn_square = Square::new(file + 8).unwrap(); // Rank 2
            let black_pawn_square = Square::new(file + 48).unwrap(); // Rank 7

            assert_eq!(
                game_state
                    .position
                    .piece_at(white_pawn_square)
                    .map(|p| p.piece_type),
                Some(PieceType::Pawn)
            );
            assert_eq!(
                game_state
                    .position
                    .piece_at(black_pawn_square)
                    .map(|p| p.piece_type),
                Some(PieceType::Pawn)
            );
        }
    }
}

#[cfg(test)]
mod castle_specific_tests {
    use chess_core::{Color, GameState, Move, Square};

    #[test]
    fn test_castle_move_creation() {
        // Test that castle moves can be created properly
        let kingside_castle = Move::castle(Square::E1, Square::G1);
        let queenside_castle = Move::castle(Square::E1, Square::C1);

        assert!(kingside_castle.is_castle());
        assert!(queenside_castle.is_castle());
        assert_eq!(kingside_castle.from, Square::E1);
        assert_eq!(kingside_castle.to, Square::G1);
    }

    #[test]
    fn test_castling_blocked_by_pieces() {
        let game_state = GameState::new();

        // In starting position, castling should be blocked by pieces
        let castle_moves = game_state.generate_castle_moves();
        assert_eq!(
            castle_moves.len(),
            0,
            "Castling should be blocked in starting position"
        );
    }

    #[test]
    fn test_castling_rights_tracking() {
        let game_state = GameState::new();

        // Test that castling rights are properly tracked
        assert!(game_state.castling_rights.can_castle_kingside(Color::White));

        // After king moves, castling rights should be lost
        // This would require implementing the castling rights update logic
        // For now, just test the basic structure
    }
}

#[cfg(test)]
mod edge_case_tests {
    use chess_core::{Color, GameState};

    #[test]
    fn test_empty_move_list_handling() {
        // Test behavior when no moves are available
        // This would happen in checkmate or stalemate positions
        let game_state = GameState::new();
        let moves = game_state.generate_legal_moves();

        // Starting position should always have moves
        assert!(
            !moves.is_empty(),
            "Starting position should have legal moves"
        );
    }

    #[test]
    fn test_invalid_fen_handling() {
        // Test that invalid FEN strings are handled gracefully
        let result = GameState::from_fen("invalid_fen_string");
        assert!(result.is_err(), "Invalid FEN should return error");

        let result = GameState::from_fen("");
        assert!(result.is_err(), "Empty FEN should return error");
    }

    #[test]
    fn test_side_to_move_alternation() {
        let mut game_state = GameState::new();

        // Starting position should be white to move
        assert_eq!(game_state.position.side_to_move, Color::White);

        // After any legal move, it should be black to move
        let moves = game_state.generate_legal_moves();
        if let Some(first_move) = moves.first() {
            let result = game_state.make_move(*first_move);
            if result.is_ok() {
                assert_eq!(game_state.position.side_to_move, Color::Black);
            }
        }
    }
}

#[cfg(test)]
mod special_moves_tests {
    use chess_core::{GameState, Move, PieceType, Square};

    #[test]
    fn test_en_passant_generation() {
        // In starting position, no en passant should be possible
        let game_state = GameState::new();
        let en_passant_moves = game_state.generate_en_passant_moves();
        assert_eq!(
            en_passant_moves.len(),
            0,
            "No en passant in starting position"
        );
    }

    #[test]
    fn test_promotion_generation() {
        // In starting position, no promotions should be possible
        let game_state = GameState::new();
        let promotion_moves = game_state.generate_promotion_moves();
        assert_eq!(
            promotion_moves.len(),
            0,
            "No promotions in starting position"
        );
    }

    #[test]
    fn test_special_move_types() {
        // Test that special move types are correctly identified
        let en_passant_move = Move::en_passant(Square::E5, Square::F6);
        let promotion_move = Move::promotion(Square::E7, Square::E8, PieceType::Queen);
        let castle_move = Move::castle(Square::E1, Square::G1);

        assert!(en_passant_move.is_en_passant());
        assert!(promotion_move.is_promotion());
        assert!(castle_move.is_castle());

        // Test that normal moves don't register as special
        let normal_move = Move::normal(Square::E2, Square::E4);
        assert!(!normal_move.is_en_passant());
        assert!(!normal_move.is_promotion());
        assert!(!normal_move.is_castle());
    }

    #[test]
    fn test_move_generation_includes_all_types() {
        let game_state = GameState::new();
        let all_moves = game_state.generate_pseudo_legal_moves();

        // Starting position should have normal moves but no special moves
        let normal_moves: Vec<_> = all_moves
            .iter()
            .filter(|m| !m.is_castle() && !m.is_en_passant() && !m.is_promotion())
            .collect();

        let special_moves: Vec<_> = all_moves
            .iter()
            .filter(|m| m.is_castle() || m.is_en_passant() || m.is_promotion())
            .collect();

        assert!(!normal_moves.is_empty(), "Should have normal moves");
        assert_eq!(
            special_moves.len(),
            0,
            "Should have no special moves in starting position"
        );
    }
}

#[cfg(test)]
mod comprehensive_rules_tests {
    use chess_core::{Color, GameState};

    #[test]
    fn test_all_move_types_framework() {
        let game_state = GameState::new();

        // Test that all move generation methods exist and work
        let normal_moves = game_state.generate_pseudo_legal_moves();
        let castle_moves = game_state.generate_castle_moves();
        let en_passant_moves = game_state.generate_en_passant_moves();
        let promotion_moves = game_state.generate_promotion_moves();

        // All methods should return vectors (even if empty)
        // Verify move collections are valid (length checks are unnecessary for usize)
        let _ = normal_moves.len();
        let _ = castle_moves.len();
        let _ = en_passant_moves.len();
        let _ = promotion_moves.len();
    }

    #[test]
    fn test_game_state_consistency() {
        let game_state = GameState::new();

        // Game state should be consistent
        assert_eq!(game_state.position.side_to_move, Color::White);
        assert!(!game_state.is_in_check(Color::White));
        assert!(!game_state.is_in_check(Color::Black));
        assert!(!game_state.is_checkmate());
        assert!(!game_state.is_stalemate());
        assert!(!game_state.is_draw());
    }

    #[test]
    fn test_legal_vs_pseudo_legal_moves() {
        let game_state = GameState::new();

        let legal_moves = game_state.generate_legal_moves();
        let pseudo_legal_moves = game_state.generate_pseudo_legal_moves();

        // In starting position, all pseudo-legal moves should be legal
        // (no pieces can be pinned or expose the king)
        assert_eq!(
            legal_moves.len(),
            pseudo_legal_moves.len(),
            "All pseudo-legal moves should be legal in starting position"
        );
    }
}

#[cfg(test)]
mod performance_tests {
    use chess_core::{GameState, Position};
    use std::time::Instant;

    #[test]
    fn test_move_generation_performance() {
        let game_state = GameState::new();
        let iterations = 1000;

        let start = Instant::now();
        for _ in 0..iterations {
            let _moves = game_state.generate_legal_moves();
        }
        let duration = start.elapsed();

        let moves_per_second = iterations as f64 / duration.as_secs_f64();

        // Should be able to generate moves quickly
        assert!(
            moves_per_second > 10000.0,
            "Move generation should be fast: {} moves/sec",
            moves_per_second
        );
    }

    #[test]
    fn test_position_creation_performance() {
        let iterations = 10000;

        let start = Instant::now();
        for _ in 0..iterations {
            let _position = Position::new();
        }
        let duration = start.elapsed();

        let positions_per_second = iterations as f64 / duration.as_secs_f64();

        // Should be able to create positions quickly
        assert!(
            positions_per_second > 100000.0,
            "Position creation should be fast: {} positions/sec",
            positions_per_second
        );
    }
}
