use chess_engine::{ChessEngineBuilder, Color, GameResult};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Chess Engine Basic Usage Example");
    println!("=================================\n");

    // Create and initialize the chess engine
    let mut engine = ChessEngineBuilder::new()
        .with_depth(6)
        .with_time_limit(5000) // 5 seconds
        .build()?;

    println!("Engine initialized successfully!");
    println!("Starting position: {}", engine.get_fen());

    // Main game loop
    loop {
        // Display current game state
        display_game_info(&engine);

        // Check if game is over
        let game_result = engine.get_game_result();
        if game_result != GameResult::Ongoing {
            println!("\nGame Over! Result: {:?}", game_result);
            break;
        }

        // Get user input or computer move
        if engine.get_side_to_move() == Color::White {
            // Human player (white)
            print!("\nEnter your move (UCI format, e.g., e2e4): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let move_str = input.trim();

            if move_str == "quit" {
                break;
            }

            // Try to make the move
            match engine.make_move_from_uci(move_str) {
                Ok(result) => {
                    if result.success {
                        println!("Move {} played successfully!", move_str);
                    } else {
                        println!("Invalid move: {}", move_str);
                        continue;
                    }
                }
                Err(e) => {
                    println!("Error making move: {}", e);
                    continue;
                }
            }
        } else {
            // Computer player (black)
            println!("\nComputer is thinking...");

            match engine.find_best_move() {
                Ok(Some(best_move)) => {
                    println!("Computer plays: {}", best_move);
                    engine.make_move(best_move)?;
                }
                Ok(None) => {
                    println!("No legal moves available");
                    break;
                }
                Err(e) => {
                    println!("Error finding best move: {}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}

fn display_game_info(engine: &chess_engine::ChessEngine) {
    let info = engine.get_game_info();

    println!("\n{}", "=".repeat(50));
    println!("Side to move: {:?}", info.side_to_move);
    println!("Legal moves: {} available", info.legal_moves.len());

    if info.is_check {
        println!("⚠️  IN CHECK!");
    }

    if info.is_checkmate {
        println!("🏁 CHECKMATE!");
    } else if info.is_stalemate {
        println!("🤝 STALEMATE!");
    } else if info.is_draw {
        println!("🤝 DRAW!");
    }

    println!("Halfmove clock: {}", info.halfmove_clock);
    println!("Fullmove number: {}", info.fullmove_number);

    let evaluation = engine.evaluate();
    if evaluation > 0 {
        println!("Position evaluation: +{} (White advantage)", evaluation);
    } else if evaluation < 0 {
        println!("Position evaluation: {} (Black advantage)", evaluation);
    } else {
        println!("Position evaluation: 0 (Equal position)");
    }

    println!("FEN: {}", info.fen);
    println!("{}", "=".repeat(50));

    // Show some legal moves
    if !info.legal_moves.is_empty() {
        println!(
            "Some legal moves: {}",
            info.legal_moves
                .iter()
                .take(8)
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        if info.legal_moves.len() > 8 {
            println!("... and {} more", info.legal_moves.len() - 8);
        }
    }
}
