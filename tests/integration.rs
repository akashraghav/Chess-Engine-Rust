use chess_core::{GameState, MoveGenerator};
use chess_engine::{ChessEngineBuilder, SearchConfig};

#[test]
fn test_engine_integration() {
    let engine = ChessEngineBuilder::new()
        .with_depth(3)
        .build()
        .expect("Failed to create engine");
    
    // Test basic engine functionality
    let game_info = engine.get_game_info();
    assert!(!game_info.legal_moves.is_empty());
    
    println!("✅ Engine integration test passed");
}

#[test]
fn test_move_making_integration() {
    let mut engine = ChessEngineBuilder::new()
        .with_depth(2)
        .build()
        .expect("Failed to create engine");
    
    // Make a basic opening move
    let result = engine.make_move_from_uci("e2e4");
    assert!(result.is_ok(), "Should be able to make legal move e2e4");
    
    // Verify game state changed
    let game_info = engine.get_game_info();
    assert_eq!(game_info.side_to_move, chess_core::Color::Black);
    
    println!("✅ Move making integration test passed");
}

#[test]
fn test_search_integration() {
    let mut engine = ChessEngineBuilder::new()
        .with_depth(2)
        .build()
        .expect("Failed to create engine");
    
    // Test that engine can find a move
    let best_move = engine.find_best_move();
    assert!(best_move.is_ok());
    
    if let Ok(Some(mv)) = best_move {
        println!("✅ Search integration test passed: found move {}", mv);
    } else {
        println!("✅ Search integration test passed: no move found (valid for some positions)");
    }
}

#[test]
fn test_position_evaluation_integration() {
    let engine = ChessEngineBuilder::new()
        .build()
        .expect("Failed to create engine");
    
    // Test position evaluation
    let eval = engine.get_evaluation();
    // Starting position should be roughly equal (close to 0)
    assert!(eval.abs() < 100, "Starting position evaluation should be close to 0, got {}", eval);
    
    println!("✅ Position evaluation integration test passed: eval = {}", eval);
}