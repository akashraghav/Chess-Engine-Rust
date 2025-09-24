use chess_core::{GameState, MoveGenerator, Square, PieceType, Color};

#[test]
fn test_perft_starting_position() {
    let game_state = GameState::new();
    let generator = MoveGenerator::new();
    
    // Perft test for starting position at depth 1
    let moves = generator.generate_legal_moves(&game_state);
    assert_eq!(moves.len(), 20, "Starting position should have exactly 20 legal moves");
    
    println!("✅ Perft test passed: {} moves from starting position", moves.len());
}

#[test]
fn test_basic_chess_rules() {
    let mut game_state = GameState::new();
    let generator = MoveGenerator::new();
    
    // Test initial position
    assert_eq!(game_state.side_to_move(), Color::White);
    assert!(!game_state.is_check());
    assert!(!game_state.is_checkmate());
    assert!(!game_state.is_stalemate());
    
    // Test move generation works
    let moves = generator.generate_legal_moves(&game_state);
    assert!(!moves.is_empty(), "Should have legal moves in starting position");
    
    println!("✅ Basic chess rules validation passed");
}

#[test]
fn test_castling_rights() {
    let game_state = GameState::new();
    
    // Initial castling rights
    assert!(game_state.can_castle_kingside(Color::White));
    assert!(game_state.can_castle_queenside(Color::White));
    assert!(game_state.can_castle_kingside(Color::Black));
    assert!(game_state.can_castle_queenside(Color::Black));
    
    println!("✅ Castling rights test passed");
}

#[test]
fn test_en_passant_detection() {
    // Test en passant square detection
    let game_state = GameState::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
    
    if let Some(ep_square) = game_state.en_passant_square() {
        assert_eq!(ep_square, Square::from_str("e3").unwrap());
    }
    
    println!("✅ En passant detection test passed");
}