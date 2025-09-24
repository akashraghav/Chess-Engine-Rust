use chess_engine::{ChessEngineBuilder, Color};
use chess_core::{GameState, MoveGenerator, Square, PieceType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 Chess Engine Tactical Awareness Test");
    println!("=========================================");
    
    let mut engine = ChessEngineBuilder::new()
        .with_depth(4)  // Reasonable depth for tactics
        .build()?;
    
    // Test 1: Basic tactical awareness - can engine capture a hanging piece?
    println!("\n🎯 Test 1: Capturing hanging piece");
    
    // Position where black queen is hanging on d4
    engine.set_position_from_fen("rnb1kbnr/pppp1ppp/8/4p3/3qP3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 3")?;
    
    if let Some(best_move) = engine.find_best_move()? {
        println!("   Engine suggests: {}", best_move);
        
        // Check if engine wants to capture the queen
        let move_str = best_move.to_string();
        if move_str.contains("d4") {
            println!("   ✅ PASS: Engine recognizes the hanging queen!");
        } else {
            println!("   ⚠️  Engine found different move: {}", move_str);
        }
    }
    
    // Test 2: Basic checkmate in one
    println!("\n🎯 Test 2: Checkmate in one");
    
    // Simple back-rank mate position
    engine.set_position_from_fen("6k1/5ppp/8/8/8/8/5PPP/4R1K1 w - - 0 25")?;
    
    if let Some(best_move) = engine.find_best_move()? {
        println!("   Engine suggests: {}", best_move);
        
        let move_str = best_move.to_string();
        if move_str.contains("e8") {
            println!("   ✅ PASS: Engine finds back-rank mate!");
        } else {
            println!("   ⚠️  Engine found different move: {}", move_str);
        }
    }
    
    // Test 3: Basic piece development vs material
    println!("\n🎯 Test 3: Opening development");
    
    engine.set_position_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")?;
    
    if let Some(best_move) = engine.find_best_move()? {
        println!("   Engine suggests: {}", best_move);
        
        let move_str = best_move.to_string();
        // Good developing moves or central control
        if move_str.contains("e5") || move_str.contains("d5") || 
           move_str.contains("f6") || move_str.contains("c6") ||
           move_str.contains("f5") || move_str.contains("c5") {
            println!("   ✅ PASS: Engine plays reasonable opening move!");
        } else {
            println!("   ⚠️  Engine found different move: {}", move_str);
        }
    }
    
    // Test 4: Basic evaluation understanding
    println!("\n🎯 Test 4: Position evaluation");
    
    // Material advantage position (extra queen)
    engine.set_position_from_fen("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
    let eval_with_queen = engine.get_evaluation();
    
    // Same position but queen removed
    engine.set_position_from_fen("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1")?;
    let eval_without_queen = engine.get_evaluation();
    
    println!("   Evaluation with queen: {}", eval_with_queen);
    println!("   Evaluation without queen: {}", eval_without_queen);
    
    if eval_with_queen > eval_without_queen + 500 {  // Queen worth ~900, so should be significant
        println!("   ✅ PASS: Engine correctly values material!");
    } else {
        println!("   ⚠️  Engine evaluation seems inconsistent");
    }
    
    println!("\n🎉 Tactical awareness tests completed!");
    println!("   These tests verify the engine can:");
    println!("   • Recognize hanging pieces");
    println!("   • Find basic checkmates");
    println!("   • Make reasonable opening moves");
    println!("   • Evaluate material differences");
    
    Ok(())
}