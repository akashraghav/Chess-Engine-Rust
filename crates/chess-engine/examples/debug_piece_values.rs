use chess_core::{Evaluator, GameState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let evaluator = Evaluator::new();

    let queen_fen = "4k3/8/8/8/8/8/8/3QK3 w - - 0 1";
    let rook_fen = "4k3/8/8/8/8/8/8/3RK3 w - - 0 1";
    let bishop_fen = "4k3/8/8/8/8/8/8/3BK3 w - - 0 1";
    let knight_fen = "4k3/8/8/8/8/8/8/3NK3 w - - 0 1";
    let pawn_fen = "4k3/8/8/8/8/8/8/3PK3 w - - 0 1";

    let queen_eval = evaluator.evaluate(&GameState::from_fen(queen_fen)?);
    let rook_eval = evaluator.evaluate(&GameState::from_fen(rook_fen)?);
    let bishop_eval = evaluator.evaluate(&GameState::from_fen(bishop_fen)?);
    let knight_eval = evaluator.evaluate(&GameState::from_fen(knight_fen)?);
    let pawn_eval = evaluator.evaluate(&GameState::from_fen(pawn_fen)?);

    println!("Piece evaluations:");
    println!("Queen:  {}", queen_eval);
    println!("Rook:   {}", rook_eval);
    println!("Bishop: {}", bishop_eval);
    println!("Knight: {}", knight_eval);
    println!("Pawn:   {}", pawn_eval);

    println!("\nDifferences:");
    println!("Bishop - Knight: {}", bishop_eval - knight_eval);

    Ok(())
}
