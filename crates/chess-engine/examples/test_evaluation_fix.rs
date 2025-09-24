use chess_engine::{ChessEngine, Result};

fn main() -> Result<()> {
    println!("=== Testing Fixed FEN Parser ===");
    
    let mut engine = ChessEngine::new();
    engine.initialize()?;
    
    println!("Starting position:");
    println!("  FEN: {}", engine.get_fen());
    println!("  Evaluation: {}", engine.evaluate());
    
    // Test 1: Kings only
    println!("\n=== Test 1: Kings only ===");
    engine.load_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 1")?;
    println!("  FEN: {}", engine.get_fen());
    println!("  Evaluation: {}", engine.evaluate());
    
    // Test 2: White has extra queen
    println!("\n=== Test 2: White has extra queen ===");
    engine.load_fen("4k3/8/8/8/8/8/8/3QK3 w - - 0 1")?;
    println!("  FEN: {}", engine.get_fen());
    println!("  Evaluation: {}", engine.evaluate());
    
    // Test 3: Black has extra queen
    println!("\n=== Test 3: Black has extra queen ===");
    engine.load_fen("3qk3/8/8/8/8/8/8/4K3 w - - 0 1")?;
    println!("  FEN: {}", engine.get_fen());
    println!("  Evaluation: {}", engine.evaluate());
    
    // Test 4: Material imbalance (white has extra rook)
    println!("\n=== Test 4: White has extra rook ===");
    engine.load_fen("4k3/8/8/8/8/8/8/R3K3 w - - 0 1")?;
    println!("  FEN: {}", engine.get_fen());
    println!("  Evaluation: {}", engine.evaluate());
    
    Ok(())
}