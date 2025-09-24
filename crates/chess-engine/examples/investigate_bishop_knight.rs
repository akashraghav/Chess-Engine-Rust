use chess_core::{GameState, Evaluator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Investigating Bishop vs Knight Evaluation ===");
    
    let evaluator = Evaluator::new();
    
    // Test same positions but with detailed analysis
    let bishop_fen = "4k3/8/8/8/8/8/8/3BK3 w - - 0 1"; // Bishop on d1
    let knight_fen = "4k3/8/8/8/8/8/8/3NK3 w - - 0 1"; // Knight on d1
    let empty_fen =  "4k3/8/8/8/8/8/8/4K3 w - - 0 1";  // Just kings
    
    let bishop_state = GameState::from_fen(bishop_fen)?;
    let knight_state = GameState::from_fen(knight_fen)?;
    let empty_state = GameState::from_fen(empty_fen)?;
    
    let bishop_eval = evaluator.evaluate(&bishop_state);
    let knight_eval = evaluator.evaluate(&knight_state);
    let empty_eval = evaluator.evaluate(&empty_state);
    
    println!("Raw evaluations:");
    println!("Bishop position: {}", bishop_eval);
    println!("Knight position: {}", knight_eval);
    println!("Empty position:  {}", empty_eval);
    
    println!("\nPiece contributions (subtracting empty position):");
    println!("Bishop contribution: {}", bishop_eval - empty_eval);
    println!("Knight contribution: {}", knight_eval - empty_eval);
    println!("Difference: {}", (bishop_eval - empty_eval) - (knight_eval - empty_eval));
    
    // Let's also check raw piece values
    use chess_core::PieceType;
    println!("\nRaw piece values from PieceType:");
    println!("Bishop value: {}", PieceType::Bishop.value());
    println!("Knight value: {}", PieceType::Knight.value());
    println!("Raw difference: {}", PieceType::Bishop.value() - PieceType::Knight.value());
    
    // Test different square positions to see if it's positional
    println!("\n=== Testing different squares ===");
    let positions = vec![
        ("4k3/8/8/8/8/8/8/B3K3 w - - 0 1", "a1"), // Bishop on a1
        ("4k3/8/8/8/8/8/8/N3K3 w - - 0 1", "a1"), // Knight on a1
        ("4k3/8/8/8/8/8/8/4KB2 w - - 0 1", "f1"), // Bishop on f1
        ("4k3/8/8/8/8/8/8/4KN2 w - - 0 1", "f1"), // Knight on f1
        ("4k3/8/8/8/3B4/8/8/4K3 w - - 0 1", "d4"), // Bishop on d4
        ("4k3/8/8/8/3N4/8/8/4K3 w - - 0 1", "d4"), // Knight on d4
    ];
    
    for i in (0..positions.len()).step_by(2) {
        let (bishop_pos, square) = &positions[i];
        let (knight_pos, _) = &positions[i + 1];
        
        let b_eval = evaluator.evaluate(&GameState::from_fen(bishop_pos)?);
        let n_eval = evaluator.evaluate(&GameState::from_fen(knight_pos)?);
        
        println!("Square {}: Bishop={}, Knight={}, Diff={}", 
                square, b_eval - empty_eval, n_eval - empty_eval, 
                (b_eval - empty_eval) - (n_eval - empty_eval));
    }
    
    Ok(())
}