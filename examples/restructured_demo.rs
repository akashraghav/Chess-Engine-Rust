use chess_engine::{ChessEngineBuilder, Color};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  Chess Engine Restructured Demo");
    println!("=====================================");

    // Create engine with the new modular structure
    let mut engine = ChessEngineBuilder::new()
        .with_depth(4)
        .build()?;

    println!("✅ Engine created successfully with modular architecture!");
    println!("📊 Starting position: {}", engine.get_fen());
    println!("👤 Side to move: {:?}", engine.get_side_to_move());

    let legal_moves = engine.get_legal_moves();
    println!("🎯 Legal moves available: {}", legal_moves.len());

    if !legal_moves.is_empty() {
        println!("🤖 Engine finding best move...");
        match engine.find_best_move()? {
            Some(best_move) => {
                println!("✨ Best move found: {}", best_move);
                println!("🧠 Using advanced search algorithms from modular structure!");
            }
            None => println!("❌ No moves available"),
        }
    }

    println!("\n🎉 Restructured chess engine working perfectly!");
    println!("📚 Clean modular architecture with proper separation of concerns");
    println!("⚡ All {} legal moves generated efficiently", legal_moves.len());

    Ok(())
}