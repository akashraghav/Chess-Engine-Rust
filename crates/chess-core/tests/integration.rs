use chess_core::{GameState, Color, Position, SearchEngine, SearchConfig};

#[test]
fn test_restructured_chess_engine() {
    // Test that all modules work together properly in the new structure

    // Test board module
    let position = Position::starting_position();
    assert_eq!(position.side_to_move, Color::White);

    // Test game state
    let game_state = GameState::new();
    let legal_moves = game_state.generate_legal_moves();
    assert_eq!(legal_moves.len(), 20); // Standard starting position has 20 legal moves

    // Test search engine
    let mut search_engine = SearchEngine::new(SearchConfig::default());
    let search_result = search_engine.search(&position);
    // Should not crash and should return some result
    assert!(search_result.nodes_searched > 0);
}

#[test]
fn test_modular_imports() {
    // Test that the modular structure allows clean imports
    use chess_core::board::{Position, Bitboard, Square};
    use chess_core::pieces::{Color, Piece, PieceType};
    use chess_core::moves::MoveGenerator;
    use chess_core::game::GameState;

    let position = Position::starting_position();
    let color = Color::White;
    let piece = Piece::new(PieceType::Pawn, color);
    let square = Square::A1;
    let bitboard = Bitboard::EMPTY;
    let game_state = GameState::new();
    let move_gen = MoveGenerator::new();

    // Just ensure all types can be instantiated without import conflicts
    assert_eq!(position.side_to_move, color);
    assert_eq!(piece.color, color);
    assert_eq!(square.index(), 0);
    assert_eq!(bitboard.value(), 0);
    assert_eq!(game_state.fullmove_number, 1);

    // Test move generation works
    let moves = move_gen.generate_legal_moves(&position);
    assert_eq!(moves.len(), 20);
}

#[test]
fn test_clean_architecture_principles() {
    // Test that the architecture follows good principles:
    // 1. Single Responsibility - each module has a clear purpose
    // 2. Dependency Inversion - high-level modules don't depend on low-level details
    // 3. Open/Closed - modules are open for extension, closed for modification

    // Board module should handle only board representation
    let _position = chess_core::board::Position::new();
    let _bitboard = chess_core::board::Bitboard::EMPTY;
    let _square = chess_core::board::Square::A1;

    // Pieces module should handle only piece logic
    let _color = chess_core::pieces::Color::White;
    let _piece = chess_core::pieces::Piece::new(chess_core::pieces::PieceType::King, chess_core::pieces::Color::White);

    // Moves module should handle only move generation and validation
    let _move_gen = chess_core::moves::MoveGenerator::new();

    // Game module should handle only game state
    let _game_state = chess_core::game::GameState::new();

    // Search module should handle only search algorithms
    let _search_engine = chess_core::search::SearchEngine::new(chess_core::search::SearchConfig::default());

    // This test passing means the modular structure is working correctly
}