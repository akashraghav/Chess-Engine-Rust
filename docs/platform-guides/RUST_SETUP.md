# 🦀 Rust Development Setup Guide

*Complete guide to setting up and using Chess Engine Rust in your Rust projects.*

---

## 📋 Quick Setup

```bash
# Add to your existing Rust project
cargo add chess-engine-rust

# Or add to Cargo.toml manually
[dependencies]
chess-engine-rust = "0.1.0"
```

## 🚀 Basic Usage

### Simple Game Setup

```rust
use chess_engine::{ChessEngineBuilder, Color, Move, Square};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create engine with default settings
    let mut engine = ChessEngineBuilder::new().build()?;

    // Make some moves
    engine.make_move_from_uci("e2e4")?;  // 1. e4
    engine.make_move_from_uci("e7e5")?;  // 1... e5

    // Get game information
    let info = engine.get_game_info();
    println!("Position: {}", info.fen);
    println!("Side to move: {:?}", info.side_to_move);
    println!("Legal moves: {}", info.legal_moves.len());

    // Find best move
    if let Some(best_move) = engine.find_best_move()? {
        println!("Engine recommends: {}", best_move);
    }

    Ok(())
}
```

### Advanced Configuration

```rust
use chess_engine::{ChessEngineBuilder, SearchConfig, EvaluationConfig};
use std::time::Duration;

fn create_tournament_engine() -> Result<ChessEngine, Box<dyn std::error::Error>> {
    let engine = ChessEngineBuilder::new()
        // Search configuration
        .with_depth(10)                          // Deep search
        .with_time_limit(Duration::from_secs(5)) // 5 second limit
        .with_aspiration_windows(true)           // Advanced search

        // Evaluation configuration
        .with_advanced_evaluation(true)          // Better position assessment
        .with_endgame_tables(true)              // Endgame knowledge

        // Performance configuration
        .with_threads(4)                        // Multi-threading
        .with_transposition_table_size(1_000_000) // 1M entries
        .with_hash_size_mb(256)                 // 256MB hash table

        // Features
        .with_opening_book(true)                // Opening knowledge
        .with_pondering(true)                   // Think on opponent's time
        .with_debug_mode(false)                 // Production mode

        .build()?;

    Ok(engine)
}
```

---

## 🎯 Core Features

### Move Generation and Validation

```rust
use chess_engine::{Position, MoveGenerator, Move, Square};

// Generate all legal moves
let position = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
let move_gen = MoveGenerator::new();
let legal_moves = move_gen.generate_legal_moves(&position);

println!("Legal moves from starting position: {}", legal_moves.len()); // 20

// Check if a specific move is legal
let move_e4 = Move::normal(Square::E2, Square::E4);
let is_legal = move_gen.is_legal_move(&position, move_e4);
println!("e2-e4 is legal: {}", is_legal); // true

// Generate moves for specific piece types
let pawn_moves = move_gen.generate_pawn_moves(&position);
let knight_moves = move_gen.generate_knight_moves(&position);
let castle_moves = move_gen.generate_castle_moves(&position);
```

### Position Management

```rust
use chess_engine::{Position, GameState, Color};

// Create positions from FEN
let position = Position::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4")?;

// Access position properties
println!("Side to move: {:?}", position.side_to_move);
println!("Castling rights: {:?}", position.castling_rights);
println!("En passant target: {:?}", position.en_passant_target);

// Get piece at specific square
if let Some(piece) = position.piece_at(Square::E4) {
    println!("Piece at e4: {:?} {:?}", piece.color, piece.piece_type);
}

// Check game state
let game_state = GameState::from_position(position);
println!("Is in check: {}", game_state.is_in_check(Color::White));
println!("Is checkmate: {}", game_state.is_checkmate());
println!("Is stalemate: {}", game_state.is_stalemate());
```

### Search and Evaluation

```rust
use chess_engine::{SearchEngine, SearchConfig, Evaluator};

// Configure search engine
let search_config = SearchConfig {
    max_depth: 8,
    max_time: Some(Duration::from_secs(3)),
    use_iterative_deepening: true,
    use_transposition_table: true,
    use_null_move_pruning: true,
    aspiration_window: 50,
    ..Default::default()
};

let mut search_engine = SearchEngine::new(search_config);

// Search for best move
let result = search_engine.search(&position);
if let Some(best_move) = result.best_move {
    println!("Best move: {} (score: {})", best_move, result.score);
    println!("Searched {} nodes in {:?}", result.nodes_searched, result.time_taken);
}

// Evaluate position
let evaluator = Evaluator::new();
let score = evaluator.evaluate(&position);
println!("Position evaluation: {} centipawns", score);
```

---

## 🔧 Advanced Features

### Custom Evaluation Functions

```rust
use chess_engine::{Evaluator, Position, EvaluationFeature};

// Create custom evaluator with specific features
let mut evaluator = Evaluator::builder()
    .with_material_evaluation(true)
    .with_piece_square_tables(true)
    .with_king_safety(true)
    .with_pawn_structure(true)
    .with_mobility(true)
    .with_endgame_knowledge(true)
    .build();

// Add custom evaluation feature
evaluator.add_feature(EvaluationFeature::Custom {
    name: "center_control",
    weight: 10,
    evaluator: Box::new(|position: &Position| {
        // Custom evaluation logic
        let center_squares = [Square::D4, Square::D5, Square::E4, Square::E5];
        let mut score = 0;

        for square in center_squares {
            if let Some(piece) = position.piece_at(square) {
                score += match piece.color {
                    Color::White => 10,
                    Color::Black => -10,
                };
            }
        }

        score
    }),
});

let score = evaluator.evaluate(&position);
```

### Performance Optimization

```rust
use chess_engine::{OptimizedEngine, ParallelConfig, MemoryConfig};

// Configure for maximum performance
let parallel_config = ParallelConfig {
    num_threads: 8,                    // Use all CPU cores
    enable_parallel_search: true,
    enable_parallel_evaluation: true,
    enable_lazy_smp: true,            // Advanced parallelization
    thread_affinity: true,            // Pin threads to cores
};

let memory_config = MemoryConfig {
    transposition_table_mb: 512,     // 512MB hash table
    evaluation_cache_mb: 128,        // 128MB eval cache
    move_pool_size: 10000,          // Pre-allocated moves
    use_memory_mapping: true,        // Memory-mapped files
};

let engine = OptimizedEngine::new()
    .with_parallel_config(parallel_config)
    .with_memory_config(memory_config)
    .with_simd_optimizations(true)   // Use SIMD instructions
    .build()?;

// Run performance benchmarks
let benchmark_results = engine.benchmark(Duration::from_secs(10));
println!("Move generation: {} moves/second", benchmark_results.moves_per_second);
println!("Position evaluation: {} positions/second", benchmark_results.evaluations_per_second);
```

### Event Handling and Callbacks

```rust
use chess_engine::{ChessEngine, EngineEvent, EventHandler};

struct MyEventHandler;

impl EventHandler for MyEventHandler {
    fn on_move_made(&mut self, move_item: Move, position: &Position) {
        println!("Move made: {} -> new position: {}", move_item, position.to_fen());
    }

    fn on_search_started(&mut self, depth: u8) {
        println!("Starting search at depth {}", depth);
    }

    fn on_search_progress(&mut self, depth: u8, nodes: u64, time: Duration) {
        println!("Depth {} progress: {} nodes in {:?}", depth, nodes, time);
    }

    fn on_best_move_found(&mut self, move_item: Move, score: i32) {
        println!("New best move: {} (score: {})", move_item, score);
    }
}

// Use custom event handler
let mut engine = ChessEngineBuilder::new()
    .with_event_handler(Box::new(MyEventHandler))
    .build()?;
```

---

## 📊 Performance Tuning

### Compilation Optimization

```toml
# Cargo.toml - optimizations for production
[profile.release]
lto = "fat"              # Link-time optimization
codegen-units = 1        # Better optimization
panic = "abort"          # Smaller binary
opt-level = 3           # Maximum optimization

# Target-specific optimizations
[target.'cfg(target_arch = "x86_64")']
rustflags = ["-C", "target-cpu=native", "-C", "target-feature=+avx2,+bmi2"]

[target.'cfg(target_arch = "aarch64")']
rustflags = ["-C", "target-cpu=native"]
```

### Runtime Configuration

```rust
use chess_engine::{PerformanceConfig, CpuFeatures};

// Detect and use CPU features
let cpu_features = CpuFeatures::detect();
println!("Available features: {:?}", cpu_features);

let perf_config = PerformanceConfig::builder()
    .with_simd_level(cpu_features.max_simd_level())
    .with_prefetch_enabled(true)
    .with_branch_prediction_hints(true)
    .with_cache_optimization(true)
    .build();

let engine = ChessEngineBuilder::new()
    .with_performance_config(perf_config)
    .build()?;
```

### Memory Management

```rust
use chess_engine::{MemoryPool, AllocatorConfig};

// Custom memory allocator for better performance
let allocator = AllocatorConfig::builder()
    .with_pool_size_mb(1024)        // 1GB memory pool
    .with_alignment(64)             // Cache-line alignment
    .with_huge_pages(true)          // Use huge pages if available
    .build();

let engine = ChessEngineBuilder::new()
    .with_allocator(allocator)
    .build()?;
```

---

## 🧪 Testing and Debugging

### Unit Testing with Chess Engine

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chess_engine::test_utils::*;

    #[test]
    fn test_basic_chess_logic() {
        let mut engine = ChessEngineBuilder::new().build().unwrap();

        // Test legal moves
        let moves = engine.get_legal_moves();
        assert_eq!(moves.len(), 20, "Starting position should have 20 legal moves");

        // Test move making
        assert!(engine.make_move_from_uci("e2e4").is_ok());
        assert!(engine.make_move_from_uci("invalid_move").is_err());

        // Test position consistency
        let fen = engine.get_fen();
        assert!(fen.contains("w"), "Should be white to move after black's move");
    }

    #[test]
    fn test_engine_performance() {
        let mut engine = ChessEngineBuilder::new()
            .with_depth(6)
            .build().unwrap();

        let start = std::time::Instant::now();
        let best_move = engine.find_best_move().unwrap();
        let duration = start.elapsed();

        assert!(best_move.is_some(), "Engine should find a move");
        assert!(duration < Duration::from_secs(5), "Search should complete within 5 seconds");
    }
}
```

### Debug Mode and Logging

```rust
use chess_engine::{ChessEngineBuilder, LogLevel};

let engine = ChessEngineBuilder::new()
    .with_debug_mode(true)
    .with_log_level(LogLevel::Debug)
    .with_log_destination(LogDestination::File("chess_engine.log".into()))
    .build()?;

// Enable specific debug features
engine.debug_config()
    .enable_search_tree_visualization()
    .enable_move_generation_tracing()
    .enable_evaluation_breakdown()
    .enable_performance_profiling();
```

---

## 🔗 Integration Examples

### Command Line Interface

```rust
use chess_engine::{ChessEngineBuilder, UciProtocol};
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = ChessEngineBuilder::new().build()?;
    let mut uci = UciProtocol::new(&mut engine);

    println!("Chess Engine Console - Type 'help' for commands");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line?;
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.get(0) {
            Some(&"quit") => break,
            Some(&"move") => {
                if let Some(move_str) = parts.get(1) {
                    match engine.make_move_from_uci(move_str) {
                        Ok(_) => println!("Move made: {}", move_str),
                        Err(e) => println!("Invalid move: {}", e),
                    }
                }
            }
            Some(&"bestmove") => {
                if let Some(best_move) = engine.find_best_move()? {
                    println!("Best move: {}", best_move);
                } else {
                    println!("No legal moves available");
                }
            }
            Some(&"position") => {
                println!("Current position: {}", engine.get_fen());
                println!("Legal moves: {:?}", engine.get_legal_moves());
            }
            Some(&"help") => {
                println!("Commands: move <move>, bestmove, position, quit");
            }
            _ => println!("Unknown command. Type 'help' for available commands."),
        }
    }

    Ok(())
}
```

### Game Analysis Tool

```rust
use chess_engine::{ChessEngineBuilder, GameAnalyzer, AnalysisConfig};

fn analyze_pgn_game(pgn_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let game_moves = parse_pgn_file(pgn_file)?;

    let analyzer = GameAnalyzer::new(
        ChessEngineBuilder::new()
            .with_depth(12)  // Deep analysis
            .build()?
    );

    let analysis_config = AnalysisConfig {
        depth_per_move: 12,
        time_per_move: Duration::from_secs(10),
        annotate_blunders: true,
        annotate_missed_tactics: true,
        include_alternative_lines: true,
    };

    let analysis = analyzer.analyze_game(&game_moves, analysis_config)?;

    // Print analysis results
    for (move_num, move_analysis) in analysis.moves.iter().enumerate() {
        println!("Move {}: {}", move_num + 1, move_analysis.move_made);
        println!("  Evaluation: {} cp", move_analysis.evaluation);

        if let Some(best_move) = &move_analysis.best_alternative {
            println!("  Best move: {} ({} cp)", best_move.move_str, best_move.evaluation);
        }

        if move_analysis.is_blunder {
            println!("  ❌ BLUNDER! Lost {} cp", move_analysis.centipawn_loss);
        }

        if !move_analysis.tactical_opportunities.is_empty() {
            println!("  🎯 Missed tactics: {:?}", move_analysis.tactical_opportunities);
        }
    }

    println!("\n📊 Game Summary:");
    println!("  Average centipawn loss: {}", analysis.average_centipawn_loss);
    println!("  Number of blunders: {}", analysis.blunder_count);
    println!("  Accuracy: {:.1}%", analysis.accuracy_percentage);

    Ok(())
}
```

---

## 🚀 Getting Started Checklist

### For New Projects

- [ ] ✅ Add `chess-engine-rust` to Cargo.toml
- [ ] 🔧 Choose appropriate configuration (depth, threads, time limits)
- [ ] 🧪 Write basic tests to verify integration
- [ ] 📚 Read the [HOW-IT-WORKS.md](../../HOW-IT-WORKS.md) guide
- [ ] ⚡ Run performance benchmarks on your target hardware

### For Existing Projects

- [ ] 📦 Update dependencies to latest version
- [ ] 🔄 Migrate from other chess libraries (see migration guide)
- [ ] 🧪 Update tests to use new APIs
- [ ] ⚡ Benchmark performance improvements
- [ ] 📝 Update documentation

---

## 📚 Additional Resources

- **📖 API Documentation**: Available in the source code and `docs/` directory
- **🏗️ Architecture Guide**: [HOW-IT-WORKS.md](../../HOW-IT-WORKS.md)
- **🤝 Contributing**: [CONTRIBUTING.md](../../CONTRIBUTING.md)
- **💬 Community**: [GitHub Discussions](https://github.com/akashraghav/Chess-Engine-Rust/discussions)
- **🐛 Issues**: [GitHub Issues](https://github.com/akashraghav/Chess-Engine-Rust/issues)

---

*Happy chess programming! 🦀♟️*