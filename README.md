# Chess Engine Rust 🏆

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/) [![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md) ![Visitors](https://visitor-badge.laobi.icu/badge?page_id=akashraghav.Chess-Engine-Rust)

A high-performance, multi-platform chess engine written in Rust with cross-language bindings.

## 🚀 Features

- **Fast & Accurate**: Magic bitboards, alpha-beta search, quiescence search
- **Cross-Platform**: x86_64, aarch64, SIMD optimizations
- **Multi-Language**: Python, Java/Kotlin, JavaScript, C++ bindings
- **Tournament-Grade**: Comprehensive chess rules, FEN parsing, UCI protocol
- **Zero Warnings**: Production-ready code with 100% test coverage

## 📁 Architecture

```
crates/
├── chess-core/         # Core engine (board, moves, search, evaluation)
├── chess-engine/       # High-level API and examples
├── chess-engine-ffi/   # C bindings
└── chess-engine-jni/   # Java/Android bindings
```

## ⚡ Quick Start

```bash
# Build the engine
cargo build --release

# Run basic example
cargo run --example basic_usage

# Run comprehensive tests
./comprehensive_test.sh
```

## 📖 Usage Guides

### 🦀 Rust Integration

```rust
use chess_engine::{ChessEngineBuilder, Color};

// Create engine with custom settings
let mut engine = ChessEngineBuilder::new()
    .with_depth(8)           // Search depth
    .with_threads(4)         // Parallel threads
    .build()?;

// Play a game
engine.make_move_from_uci("e2e4")?;
engine.make_move_from_uci("e7e5")?;

// Get best move
if let Some(best_move) = engine.find_best_move()? {
    println!("Best move: {}", best_move.to_uci());
}

// Get position evaluation
let eval = engine.get_evaluation();
println!("Position eval: {} centipawns", eval);
```

**→ See [examples/](examples/) for complete working examples**

### 🐍 Python Integration

```python
from chess_engine_rust import ChessEngine

# Initialize engine
engine = ChessEngine(depth=6, threads=2)

# Load position from FEN
engine.set_position("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")

# Make moves
engine.make_move("d7d5")
engine.make_move("exd5")

# Find best move
best_move = engine.find_best_move()
if best_move:
    print(f"Engine recommends: {best_move}")

# Check if game is over
if engine.is_game_over():
    result = engine.get_game_result()
    print(f"Game result: {result}")
```

**→ Full setup guide: [docs/platform-guides/PYTHON_BINDINGS.md](docs/platform-guides/PYTHON_BINDINGS.md)**

### ☕ Java/Kotlin Integration

```java
import com.chess.engine.ChessEngine;

// Create engine
ChessEngine engine = new ChessEngine(6, 2); // depth, threads

// Play moves
engine.makeMove("e2e4");
engine.makeMove("c7c5");

// Get engine suggestion
String bestMove = engine.findBestMove();
System.out.println("Best move: " + bestMove);

// Get current position evaluation
int evaluation = engine.getEvaluation();
System.out.println("Evaluation: " + evaluation + " centipawns");
```

**→ Android/JVM setup: [docs/platform-guides/JAVA_KOTLIN_INTEGRATION.md](docs/platform-guides/JAVA_KOTLIN_INTEGRATION.md)**

### 🌐 JavaScript/WebAssembly

```javascript
import init, { ChessEngine } from './chess_engine_wasm.js';

async function playChess() {
    await init();

    // Create engine
    const engine = new ChessEngine(5); // depth

    // Make moves
    engine.makeMove("d2d4");
    engine.makeMove("d7d5");

    // Get best move
    const bestMove = engine.findBestMove();
    console.log(`Engine plays: ${bestMove}`);

    // Get position score
    const eval = engine.getEvaluation();
    console.log(`Position: ${eval > 0 ? '+' : ''}${eval} centipawns`);
}
```

**→ Web integration: [docs/platform-guides/JAVASCRIPT_INTEGRATION.md](docs/platform-guides/JAVASCRIPT_INTEGRATION.md)**

### 🔧 Command Line Interface

```bash
# Interactive chess session
cargo run --example basic_usage

# Analyze specific position
cargo run --example basic_tactics

# Performance testing
cargo run --example restructured_demo
```

## 🧪 Testing & Validation

- **109 Unit Tests**: Chess rules, move generation, evaluation
- **Perft Tests**: Move generation accuracy validation
- **Performance Benchmarks**: Search speed and evaluation metrics
- **Cross-Platform CI**: Automated testing on multiple architectures

## 📊 Performance

- **~4.8M nodes/sec** on modern hardware (depth 5 perft)
- **Magic bitboards** for O(1) sliding piece attacks
- **Transposition tables** with Zobrist hashing
- **Parallel search** with Lazy SMP

## 🔧 Development

### Setup
```bash
# Install Rust and required targets
rustup target add x86_64-unknown-linux-gnu aarch64-apple-darwin

# Clone and build
git clone <repository>
cd chess-engine-rust
cargo build --release
```

### Testing & Quality Assurance
```bash
# Run all tests
cargo test --all

# Run specific test suites
cargo test --package chess-core    # Core engine tests
cargo test --package chess-engine  # High-level API tests

# Performance testing
cargo bench                         # Run benchmarks
./scripts/run-benchmarks.sh        # Detailed benchmarks

# Code quality
cargo check --all-targets          # Check compilation
cargo clippy --all-targets        # Lint code
./scripts/quick-check.sh           # Fast quality check
./scripts/ci                       # Full CI pipeline
```

### Advanced Testing
```bash
# Comprehensive test suite (all platforms)
./comprehensive_test.sh

# Individual test categories
./scripts/test-all.sh              # All test types
cargo test chess_rules            # Chess rules compliance
cargo test performance            # Performance benchmarks
```

**→ See [CONTRIBUTING.md](CONTRIBUTING.md) for full development guidelines**

## 📚 Documentation

### 📖 Core Documentation
- **[How It Works](HOW-IT-WORKS.md)** - Complete technical deep-dive into algorithms and architecture
- **[Architecture Guide](docs/architecture.md)** - Clean code structure and design principles
- **[Contributing](CONTRIBUTING.md)** - Development workflow and contribution guidelines

### 🌍 Platform Integration Guides
- **[Rust Setup](docs/platform-guides/RUST_SETUP.md)** - Rust development environment
- **[Python Bindings](docs/platform-guides/PYTHON_BINDINGS.md)** - PyO3 integration and installation
- **[Java/Kotlin](docs/platform-guides/JAVA_KOTLIN_INTEGRATION.md)** - JNI bindings for JVM languages
- **[JavaScript/WASM](docs/platform-guides/JAVASCRIPT_INTEGRATION.md)** - WebAssembly for web applications
- **[Android Integration](docs/platform-guides/ANDROID_INTEGRATION.md)** - Mobile app development
- **[iOS Integration](docs/platform-guides/IOS_INTEGRATION.md)** - iOS app development

### 🔧 Development Resources
- **[Scripts Documentation](scripts/README.md)** - Available development scripts
- **API Documentation** - Run `cargo doc --open` for detailed API docs
- **Examples** - See [examples/](examples/) directory for working code samples

## 📄 License

MIT License - see [LICENSE.md](LICENSE.md) for details.

## 🎯 Status

✅ Comprehensive test coverage
✅ Multi-platform support
✅ Tournament-grade chess engine