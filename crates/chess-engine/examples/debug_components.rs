use chess_engine::{ChessEngine, Result};
use chess_core::{GameState, Evaluator, PieceType, Color};

fn main() -> Result<()> {
    println!("=== Debug Individual Evaluation Components ===");
    
    let mut engine = ChessEngine::new();
    engine.initialize()?;
    
    // Get direct access to game state and evaluator for debugging
    let game_state = GameState::new(); // Starting position
    let evaluator = Evaluator::new();
    
    println!("Starting position FEN: {}", game_state.to_fen());
    
    // Test individual components
    println!("\n=== Testing material_balance directly ===");
    // Since material_balance is private, let's use a different approach
    // Let's count pieces manually and compute expected material balance
    
    let mut white_material = 0;
    let mut black_material = 0;
    
    for piece_type in PieceType::ALL {
        let white_count = game_state.position.pieces_of_type(piece_type, Color::White).count_bits() as i32;
        let black_count = game_state.position.pieces_of_type(piece_type, Color::Black).count_bits() as i32;
        let piece_value = piece_type.value();
        
        println!("  {:?}: White={}, Black={}, Value={}", piece_type, white_count, black_count, piece_value);
        
        white_material += white_count * piece_value;
        black_material += black_count * piece_value;
    }
    
    let expected_material_balance = white_material - black_material;
    println!("  Expected material balance: {} - {} = {}", white_material, black_material, expected_material_balance);
    
    // Test the full evaluator
    println!("\n=== Testing full evaluation ===");
    let full_eval = evaluator.evaluate(&game_state);
    println!("  Full evaluation result: {}", full_eval);
    
    // Test with an unbalanced position
    println!("\n=== Testing unbalanced position (just kings) ===");
    // Position with just kings
    let unbalanced_fen = "4k3/8/8/8/8/8/8/4K3 w - - 0 1";
    let unbalanced_state = GameState::from_fen(unbalanced_fen)?;
    
    println!("Unbalanced FEN: {}", unbalanced_fen);
    
    // Count pieces in unbalanced position
    let mut white_material_unbal = 0;
    let mut black_material_unbal = 0;
    
    for piece_type in PieceType::ALL {
        let white_count = unbalanced_state.position.pieces_of_type(piece_type, Color::White).count_bits() as i32;
        let black_count = unbalanced_state.position.pieces_of_type(piece_type, Color::Black).count_bits() as i32;
        let piece_value = piece_type.value();
        
        println!("  {:?}: White={}, Black={}, Value={}", piece_type, white_count, black_count, piece_value);
        
        white_material_unbal += white_count * piece_value;
        black_material_unbal += black_count * piece_value;
    }
    
    let expected_material_balance_unbal = white_material_unbal - black_material_unbal;
    println!("  Expected material balance: {} - {} = {}", white_material_unbal, black_material_unbal, expected_material_balance_unbal);
    
    let unbalanced_eval = evaluator.evaluate(&unbalanced_state);
    println!("  Full evaluation result: {}", unbalanced_eval);
    
    Ok(())
}