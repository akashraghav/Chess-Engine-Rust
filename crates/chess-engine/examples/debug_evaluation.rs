use chess_core::{evaluation::Evaluator, GameState};
use chess_engine::ChessEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Debug Chess Engine Evaluation");
    println!("=================================");

    // Test 1: Direct evaluator on starting position
    let game_state = GameState::new();
    let evaluator = Evaluator::new();
    let direct_eval = evaluator.evaluate(&game_state);
    println!("Direct evaluator on starting position: {}", direct_eval);

    // Test 2: Engine evaluator on starting position
    let mut engine = ChessEngine::new();
    engine.initialize()?;
    let engine_eval = engine.evaluate();
    println!("Engine evaluator on starting position: {}", engine_eval);

    // Test 3: Position with material advantage
    println!("\n🎯 Testing material difference:");

    // Test simple position changes
    println!("   Starting FEN: {}", engine.get_fen());

    // Try loading a very simple position - just kings
    engine.load_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 1")?;
    println!("   After loading kings only: {}", engine.get_fen());
    let eval_kings_only = engine.evaluate();
    println!("   Evaluation with kings only: {}", eval_kings_only);

    // Load position with extra queen for white
    engine.load_fen("4k3/8/8/8/8/8/8/3QK3 w - - 0 1")?;
    println!("   After loading king+queen vs king: {}", engine.get_fen());
    let eval_white_advantage = engine.evaluate();
    println!(
        "   Evaluation with white queen advantage: {}",
        eval_white_advantage
    );

    let difference = eval_white_advantage - eval_kings_only;
    println!(
        "   Difference: {} (should be ~900 for queen value)",
        difference
    );

    // Test 4: Check piece counting in unequal position
    let game_state_unequal =
        GameState::from_fen("rnb1kb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
    let white_queens = game_state_unequal
        .position
        .pieces_of_type(chess_core::PieceType::Queen, chess_core::Color::White)
        .count_bits();
    let black_queens = game_state_unequal
        .position
        .pieces_of_type(chess_core::PieceType::Queen, chess_core::Color::Black)
        .count_bits();

    println!("\n🔍 Piece counting:");
    println!("   FEN loaded: {}", game_state_unequal.to_fen());
    println!("   White queens: {}", white_queens);
    println!("   Black queens: {}", black_queens);

    // Test 5: Check current engine state
    println!("\n🔍 Current engine state:");
    println!("   Engine FEN: {}", engine.get_fen());
    let current_state = engine.get_position();
    let current_white_queens = current_state
        .pieces_of_type(chess_core::PieceType::Queen, chess_core::Color::White)
        .count_bits();
    let current_black_queens = current_state
        .pieces_of_type(chess_core::PieceType::Queen, chess_core::Color::Black)
        .count_bits();
    println!("   Current white queens: {}", current_white_queens);
    println!("   Current black queens: {}", current_black_queens);

    Ok(())
}
