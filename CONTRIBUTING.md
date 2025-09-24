# 🤝 Contributing to Chess Engine Rust

*Thank you for your interest in contributing to our open-source chess engine! This guide will help you get started with making meaningful contributions.*

---

## 📋 Table of Contents

1. [🚀 Quick Start](#-quick-start)
2. [🛠️ Development Setup](#️-development-setup)
3. [🎯 Types of Contributions](#-types-of-contributions)
4. [📝 Contribution Process](#-contribution-process)
5. [🧪 Testing Guidelines](#-testing-guidelines)
6. [🎨 Code Style Guide](#-code-style-guide)
7. [📖 Documentation Standards](#-documentation-standards)
8. [🔍 Review Process](#-review-process)
9. [🏆 Recognition](#-recognition)

---

## 🚀 Quick Start

**Ready to contribute?**

```bash
# 1. Fork the repository on GitHub
# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/chess-engine-rust.git
cd chess-engine-rust

# 3. Set up development environment
cargo build --release
cargo test

# 4. Create a feature branch
git checkout -b feature/your-amazing-contribution

# 5. Make your changes and test
cargo test
cargo bench  # For performance-related changes

# 6. Submit a pull request
git push origin feature/your-amazing-contribution
```

---

## 🛠️ Development Setup

### Prerequisites

```bash
# Required
rustup install stable        # Rust toolchain
rustup component add clippy  # Linting
rustup component add rustfmt # Formatting

# Optional but recommended
cargo install cargo-tarpaulin # Code coverage
cargo install flamegraph      # Performance profiling
cargo install cargo-watch    # Auto-rebuild on changes
```

### Platform-Specific Setup

**🦀 Rust Development:**
```bash
# Core development
cargo build
cargo test
cargo doc --open

# Watch mode for continuous development
cargo watch -x check -x test
```

**🐍 Python Bindings:**
```bash
# Install maturin for Python binding development
pip install maturin
cd python-bindings/
maturin develop
python -m pytest tests/
```

**☕ Java/Android Bindings:**
```bash
# Install additional targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi

# Build Java bindings
cd java/
./gradlew build
./gradlew test
```

**🌐 WebAssembly Bindings:**
```bash
# Install wasm-pack
cargo install wasm-pack

# Build WASM bindings
wasm-pack build --target web
cd web-demo/
npm install
npm run serve
```

---

## 🎯 Types of Contributions

We welcome various types of contributions, from bug fixes to major features:

### 🐛 **Bug Reports**
- 🎯 **Clear reproduction steps**
- 🧪 **Minimal test case**
- 🖥️ **Environment details** (OS, Rust version, etc.)
- 📊 **Expected vs actual behavior**

**Template:**
```markdown
**Bug Description:**
Brief description of the issue.

**Steps to Reproduce:**
1. Create engine with depth 6
2. Make moves: e2e4, e7e5
3. Call find_best_move()
4. Observe incorrect result

**Expected:** Engine should suggest Nf3
**Actual:** Engine suggests Nc3

**Environment:**
- OS: macOS 14.0
- Rust: 1.75.0
- Chess Engine: 0.1.0
```

### 🚀 **Feature Requests**
- 💡 **Clear use case** - Why is this feature needed?
- 🎯 **Specific requirements** - What exactly should it do?
- 🏗️ **Implementation ideas** - Any thoughts on how to build it?

### 🔧 **Code Contributions**

**🏅 High-Impact Areas:**
- **🧠 Chess AI Improvements**: Better evaluation functions, search algorithms
- **⚡ Performance Optimizations**: SIMD, parallel processing, memory management
- **🌐 Platform Support**: New language bindings, mobile optimizations
- **🎮 User Experience**: Better APIs, examples, tutorials

**🥇 Intermediate Areas:**
- **🧪 Test Coverage**: New test cases, edge case handling
- **📚 Documentation**: Code comments, tutorials, examples
- **🐛 Bug Fixes**: Chess rule edge cases, performance issues

**🥉 Beginner-Friendly:**
- **📝 Documentation**: README improvements, code comments
- **🧹 Code Cleanup**: Formatting, naming consistency
- **📊 Examples**: New usage examples, tutorials

---

## 📝 Contribution Process

### 1. 🍴 **Fork & Clone**

```bash
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/chess-engine-rust.git
cd chess-engine-rust
git remote add upstream https://github.com/akashraghav/Chess-Engine-Rust.git
```

### 2. 🌿 **Create Feature Branch**

```bash
# Use descriptive branch names
git checkout -b feature/add-endgame-tablebase-support
git checkout -b fix/castling-rights-bug
git checkout -b docs/improve-python-examples
git checkout -b perf/optimize-move-generation
```

### 3. 🔨 **Make Changes**

**🧪 Test-Driven Development:**
```bash
# 1. Write test first
cargo test test_new_feature -- --show-output

# 2. Watch it fail
# 3. Implement feature
# 4. Watch it pass
# 5. Refactor if needed
```

**⚡ Performance Changes:**
```bash
# Benchmark before your changes
cargo bench --bench move_generation > before.txt

# Make your optimizations

# Benchmark after
cargo bench --bench move_generation > after.txt

# Compare results
diff before.txt after.txt
```

### 4. ✅ **Quality Checks**

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy -- -D warnings

# Run all tests
cargo test

# Run benchmarks (for performance changes)
cargo bench

# Check test coverage
cargo tarpaulin --out Html
```

### 5. 📝 **Write Good Commit Messages**

```bash
# Good commit messages
git commit -m "Add null move pruning to alpha-beta search

Implement null move pruning optimization that skips the current
player's turn to identify positions where even doing nothing
maintains a good position. Results in 20% search speedup.

Fixes #123"

# Follow this format:
# - Line 1: Summary (50 chars max)
# - Line 2: Empty
# - Line 3+: Detailed explanation if needed
```

### 6. 🚀 **Submit Pull Request**

**📋 Pull Request Checklist:**
- [ ] ✅ All tests pass (`cargo test`)
- [ ] 🎨 Code is formatted (`cargo fmt`)
- [ ] 🔍 No clippy warnings (`cargo clippy`)
- [ ] 📚 Documentation updated (if needed)
- [ ] 🧪 Tests added for new functionality
- [ ] 📊 Benchmarks run (for performance changes)
- [ ] 📝 CHANGELOG.md updated (for notable changes)

---

## 🧪 Testing Guidelines

We maintain **100% test coverage** and follow strict testing standards.

### 🏗️ **Test Structure**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_function_name_should_describe_behavior() {
        // Arrange: Set up test data
        let position = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let move_generator = MoveGenerator::new();

        // Act: Execute the functionality
        let moves = move_generator.generate_legal_moves(&position);

        // Assert: Verify expected behavior
        assert_eq!(moves.len(), 20, \"Starting position should have 20 legal moves\");
        assert!(moves.contains(&Move::normal(Square::E2, Square::E4)), \"Should include e2-e4\");
    }
}
```

### 📊 **Test Categories**

**🔬 Unit Tests:**
```rust
// Test individual functions/methods
#[test]
fn test_bitboard_operations() {
    let bb1 = Bitboard::from_square(Square::E4);
    let bb2 = Bitboard::from_square(Square::E5);
    let combined = bb1 | bb2;

    assert!(combined.contains(Square::E4));
    assert!(combined.contains(Square::E5));
    assert_eq!(combined.count_ones(), 2);
}
```

**🎯 Integration Tests:**
```rust
// Test module interactions
#[test]
fn test_complete_game_flow() {
    let mut engine = ChessEngineBuilder::new().build().unwrap();

    // Play a complete game sequence
    engine.make_move_from_uci("e2e4").unwrap();
    engine.make_move_from_uci("e7e5").unwrap();

    let best_move = engine.find_best_move().unwrap();
    assert!(best_move.is_some(), \"Engine should find a move\");
}
```

**🏆 Chess Rules Tests:**
```rust
// Test chess-specific rules
#[test]
fn test_en_passant_capture() {
    let mut position = setup_en_passant_position();
    let moves = generate_legal_moves(&position);

    let en_passant_move = Move::en_passant(Square::E5, Square::F6);
    assert!(moves.contains(&en_passant_move), \"En passant should be legal\");
}
```

**⚡ Performance Tests:**
```rust
// Benchmark critical paths
#[bench]
fn bench_move_generation(b: &mut Bencher) {
    let position = Position::starting_position();
    let generator = MoveGenerator::new();

    b.iter(|| {
        black_box(generator.generate_legal_moves(&position))
    });
}
```

### 🎛️ **Test Utilities**

Create helper functions for common test scenarios:

```rust
// test_utils.rs
pub fn setup_position_from_fen(fen: &str) -> Position {
    Position::from_fen(fen).expect("Valid FEN string")
}

pub fn assert_move_exists(moves: &[Move], expected: Move) {
    assert!(moves.contains(&expected),
           \"Expected move {} not found in move list: {:?}\",
           expected, moves);
}

pub fn assert_performance_threshold<F>(operation: F, threshold_ns: u64)
where
    F: Fn() -> ()
{
    let start = std::time::Instant::now();
    operation();
    let duration = start.elapsed();

    assert!(duration.as_nanos() < threshold_ns as u128,
           \"Operation took {}ns, expected < {}ns\",
           duration.as_nanos(), threshold_ns);
}
```

---

## 🎨 Code Style Guide

### 🦀 **Rust Style Guidelines**

**📝 Formatting:**
```bash
# Use cargo fmt for automatic formatting
cargo fmt

# Settings in .rustfmt.toml
max_width = 100
hard_tabs = false
tab_spaces = 4
```

**🏷️ Naming Conventions:**
```rust
// Functions and variables: snake_case
fn generate_legal_moves() -> Vec<Move> { ... }
let best_move = engine.find_best_move();

// Types: PascalCase
struct ChessEngine { ... }
enum PieceType { ... }

// Constants: SCREAMING_SNAKE_CASE
const MAX_SEARCH_DEPTH: u8 = 20;
const PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 20000];

// Modules: snake_case
mod move_generation;
mod position_evaluation;
```

**📖 Documentation:**
```rust
/// Generates all legal moves for the given position.
///
/// This function uses bitboards and magic bitboards for efficient
/// move generation, supporting all chess moves including castling,
/// en passant, and pawn promotion.
///
/// # Arguments
/// * `position` - The current board position
///
/// # Returns
/// * `Vec<Move>` - List of all legal moves
///
/// # Examples
/// ```rust
/// let position = Position::starting_position();
/// let moves = generate_legal_moves(&position);
/// assert_eq!(moves.len(), 20); // 20 legal opening moves
/// ```
pub fn generate_legal_moves(position: &Position) -> Vec<Move> {
    // Implementation...
}
```

**⚠️ Error Handling:**
```rust
// Use Result for fallible operations
pub fn make_move(&mut self, move_item: Move) -> Result<UndoInfo, ChessError> {
    if !self.is_legal_move(move_item) {
        return Err(ChessError::IllegalMove(move_item));
    }

    // Safe to make move
    Ok(self.execute_move(move_item))
}

// Use custom error types
#[derive(Debug, thiserror::Error)]
pub enum ChessError {
    #[error("Illegal move: {0}")]
    IllegalMove(Move),

    #[error("Invalid FEN string: {0}")]
    InvalidFen(String),

    #[error("Game is already over")]
    GameOver,
}
```

### 🏗️ **Architecture Guidelines**

**🎯 Single Responsibility:**
```rust
// Good: Each struct has one clear purpose
pub struct MoveGenerator {
    magic_bitboards: MagicBitboards,
}

impl MoveGenerator {
    pub fn generate_legal_moves(&self, position: &Position) -> Vec<Move> { ... }
    pub fn is_legal_move(&self, position: &Position, move_item: Move) -> bool { ... }
}

// Bad: Mixed responsibilities
pub struct ChessEngine {
    pub fn generate_moves(&self) -> Vec<Move> { ... }    // Move generation
    pub fn evaluate_position(&self) -> i32 { ... }       // Evaluation
    pub fn save_to_file(&self, path: &str) { ... }      // I/O
}
```

**🔗 Clean Dependencies:**
```rust
// Good: Clear dependency hierarchy
// Low-level: bitboard operations
// Mid-level: move generation (uses bitboards)
// High-level: search (uses move generation)

use crate::bitboard::Bitboard;  // Low-level dependency
use crate::moves::MoveGenerator; // Same-level dependency

// Bad: Circular dependencies
// search.rs imports evaluation.rs
// evaluation.rs imports search.rs
```

---

## 📖 Documentation Standards

### 📚 **Code Documentation**

**🎯 Function Documentation:**
```rust
/// Brief one-line description.
///
/// Longer description explaining the purpose, algorithm, or important
/// details about this function's behavior.
///
/// # Arguments
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
///
/// # Returns
/// * `ReturnType` - Description of return value
///
/// # Errors
/// * `ErrorType::Variant` - When this error occurs
///
/// # Examples
/// ```rust
/// let result = function_name(arg1, arg2);
/// assert_eq!(result, expected_value);
/// ```
///
/// # Performance
/// * Time complexity: O(n log n)
/// * Space complexity: O(n)
pub fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType, ErrorType> {
    // Implementation
}
```

**📋 Module Documentation:**
```rust
//! Move generation module.
//!
//! This module contains all the logic for generating legal chess moves,
//! including special moves like castling, en passant, and pawn promotion.
//!
//! # Key Components
//! * `MoveGenerator` - Main interface for move generation
//! * `MagicBitboards` - Efficient sliding piece move generation
//! * `Move` - Represents a single chess move
//!
//! # Examples
//! ```rust
//! use chess_core::moves::MoveGenerator;
//!
//! let generator = MoveGenerator::new();
//! let moves = generator.generate_legal_moves(&position);
//! ```

use crate::bitboard::Bitboard;
```

### 📝 **External Documentation**

**📖 README Updates:**
When adding significant features, update the README with:
- Feature description in the main features list
- Code example showing usage
- Performance impact (if applicable)

**📚 Tutorial Content:**
For major features, consider adding:
- Step-by-step tutorial in `docs/tutorials/`
- Working example in `examples/`
- Platform-specific guide if relevant

---

## 🔍 Review Process

### 👥 **What We Review**

**🎯 Functionality:**
- ✅ Does it work correctly?
- ✅ Handles edge cases?
- ✅ Follows chess rules precisely?
- ✅ Performance is acceptable?

**🏗️ Code Quality:**
- ✅ Clean, readable code?
- ✅ Good naming conventions?
- ✅ Proper error handling?
- ✅ Follows architecture guidelines?

**🧪 Testing:**
- ✅ Comprehensive test coverage?
- ✅ Tests are clear and maintainable?
- ✅ Performance tests for optimizations?

**📚 Documentation:**
- ✅ Code is well-documented?
- ✅ Public APIs have examples?
- ✅ README updated if needed?

### 📋 **Review Checklist**

**For Reviewers:**
```markdown
## Code Review Checklist

### Functionality
- [ ] ✅ Feature works as described
- [ ] ✅ Edge cases handled
- [ ] ✅ No regressions in existing functionality
- [ ] ✅ Performance impact is acceptable

### Code Quality
- [ ] 🎨 Code follows style guidelines
- [ ] 🏗️ Architecture is clean and maintainable
- [ ] ⚠️ Error handling is appropriate
- [ ] 🔍 No obvious bugs or issues

### Testing
- [ ] 🧪 New functionality is tested
- [ ] 📊 Tests cover edge cases
- [ ] ⚡ Performance tests added (if applicable)
- [ ] ✅ All tests pass

### Documentation
- [ ] 📝 Code is well-documented
- [ ] 📚 Public APIs have examples
- [ ] 📖 User-facing documentation updated
```

### 🔄 **Review Process Steps**

1. **📥 Automatic Checks**: CI runs tests, linting, formatting checks
2. **👁️ Manual Review**: Team member reviews code for quality and correctness
3. **💬 Discussion**: Questions, suggestions, and improvements discussed
4. **🔨 Revisions**: Author makes requested changes
5. **✅ Approval**: Reviewer approves the changes
6. **🚀 Merge**: Changes are merged into main branch

---

## 🏆 Recognition

### 🌟 **Contributor Recognition**

**📊 Contribution Types:**
- 🐛 **Bug Reports**: Reported critical bugs
- 🔧 **Code Contributions**: Added features or fixes
- 📚 **Documentation**: Improved docs, tutorials, examples
- ⚡ **Performance**: Optimization contributions
- 🧪 **Testing**: Added test coverage
- 🎯 **Review**: Helped review other contributions

**🏅 Recognition Methods:**
- **Contributors list** in README.md
- **Release notes** mention for significant contributions
- **Social media** shout-outs for major features
- **Maintainer status** for consistent, high-quality contributors

### 📈 **Contribution Ladder**

```
🥉 **First-time Contributor**
   ↓ (1-2 contributions)
🥈 **Regular Contributor**
   ↓ (5+ contributions, consistent quality)
🥇 **Core Contributor**
   ↓ (10+ contributions, mentors others)
👑 **Maintainer**
```

---

## ❓ **Getting Help**

**🤔 Questions?**
- 💬 **Discussions**: Use GitHub Discussions for general questions
- 🐛 **Issues**: Create an issue for bugs or feature requests
- 📧 **Direct Contact**: Reach out to maintainers for sensitive issues

**📚 Resources:**
- 🔗 [Chess Programming Wiki](https://www.chessprogramming.org/)
- 📖 [Rust Book](https://doc.rust-lang.org/book/)
- 🎥 Chess Engine Video Tutorials - Search YouTube for "chess engine programming tutorials"

---

## 🎯 **Final Notes**

**We Value:**
- 🤝 **Respectful communication** and constructive feedback
- 🧠 **Learning mindset** - everyone is here to improve
- 🏆 **Quality over quantity** - well-tested, thoughtful contributions
- 🌟 **Community spirit** - helping others succeed

**Remember:**
- 🎯 **Start small** - even fixing typos helps!
- 📚 **Learn continuously** - chess programming is complex and rewarding
- 🤝 **Ask questions** - we're here to help you succeed
- 🎉 **Celebrate progress** - every contribution makes the engine better

---

*Thank you for contributing to Chess Engine Rust! Together, we're building something amazing.* 🚀

**Happy coding!** ♟️👨‍💻👩‍💻