use chess_engine::{ChessEngine, Result};

fn main() -> Result<()> {
    println!("Creating engine...");
    let mut engine = ChessEngine::new();
    
    println!("Initializing engine...");
    engine.initialize()?;
    
    println!("Engine created and initialized successfully");
    
    println!("Getting starting FEN...");
    let starting_fen = engine.get_fen();
    println!("Starting FEN: {}", starting_fen);
    
    println!("Testing evaluation...");
    let eval = engine.evaluate();
    println!("Starting position evaluation: {}", eval);
    
    Ok(())
}