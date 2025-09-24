use chess_engine::{ChessEngine, Result};

fn main() -> Result<()> {
    println!("=== Testing ChessEngine load_fen method ===");
    
    let mut engine = ChessEngine::new();
    engine.initialize()?;
    
    println!("Starting FEN: {}", engine.get_fen());
    println!("Starting evaluation: {}", engine.evaluate());
    
    // Count pieces in starting position
    println!("\nStarting position piece count:");
    count_pieces_via_engine(&engine);
    
    // Load a FEN with just kings
    println!("\n=== Loading kings-only FEN ===");
    engine.load_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 1")?;
    
    println!("After load_fen: {}", engine.get_fen());
    println!("Kings-only evaluation: {}", engine.evaluate());
    
    // Count pieces after FEN load
    println!("\nAfter FEN load piece count:");
    count_pieces_via_engine(&engine);
    
    // Load a FEN with material imbalance
    println!("\n=== Loading queen vs king+king FEN ===");
    engine.load_fen("4k3/8/8/8/8/8/8/3QK3 w - - 0 1")?;
    
    println!("After queen load_fen: {}", engine.get_fen());
    println!("Queen advantage evaluation: {}", engine.evaluate());
    
    // Count pieces after queen FEN load
    println!("\nAfter queen FEN load piece count:");
    count_pieces_via_engine(&engine);
    
    Ok(())
}

fn count_pieces_via_engine(engine: &ChessEngine) {
    // We can't directly access the game state from ChessEngine, but we can check via FEN
    let fen = engine.get_fen();
    println!("  FEN: {}", fen);
    
    // Parse the FEN to count pieces manually
    let board_part = fen.split_whitespace().next().unwrap();
    let mut piece_counts = std::collections::HashMap::new();
    
    for ch in board_part.chars() {
        if ch.is_alphabetic() {
            *piece_counts.entry(ch).or_insert(0) += 1;
        }
    }
    
    println!("  Piece counts from FEN: {:?}", piece_counts);
}