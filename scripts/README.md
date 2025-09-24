# 🚀 Local CI Scripts

This directory contains scripts to run CI jobs locally before pushing to GitHub.

## 📝 Available Scripts

### `./scripts/run-ci-locally.sh` - Complete CI Simulation
Runs all CI jobs locally, matching the GitHub Actions workflow exactly.

```bash
./scripts/run-ci-locally.sh
```

**What it includes:**
- ✅ Code formatting checks (`cargo fmt`)
- ✅ Clippy lints (`cargo clippy`)
- ✅ Documentation build (`cargo doc`)
- ✅ Debug and release builds
- ✅ Unit tests, chess rules tests, integration tests
- ✅ Chess engine validation
- ✅ Performance benchmarks
- ✅ Cross-platform compilation checks
- ✅ Language bindings (FFI/JNI)
- ✅ Security vulnerability scanning

### `./scripts/quick-check.sh` - Essential Checks Only
Fast validation of the most critical issues.

```bash
./scripts/quick-check.sh
```

**Perfect for:**
- Before committing changes
- Quick validation during development
- Pre-push checks

### `./scripts/test-all.sh` - Comprehensive Testing
Runs all test suites with detailed reporting.

```bash
# Basic testing
./scripts/test-all.sh

# With coverage report
./scripts/test-all.sh --coverage
```

**Includes:**
- Unit tests (82 tests)
- Chess rules validation (27 tests)
- Integration tests
- Performance tests
- Documentation tests
- Multi-threaded testing
- Memory leak detection (if valgrind available)

### `./scripts/run-benchmarks.sh` - Performance Testing
Runs performance benchmarks matching CI benchmark job.

```bash
# Basic benchmarks
./scripts/run-benchmarks.sh

# Save results to files
./scripts/run-benchmarks.sh --save-results
```

**Benchmarks:**
- Move generation speed
- Position evaluation performance
- Search algorithm efficiency
- Memory usage analysis
- Multi-threading performance

## 🎯 Usage Recommendations

### Before Every Commit
```bash
./scripts/quick-check.sh
```

### Before Every Push
```bash
./scripts/run-ci-locally.sh
```

### During Performance Optimization
```bash
./scripts/run-benchmarks.sh --save-results
```

### For Release Testing
```bash
./scripts/test-all.sh --coverage
./scripts/run-benchmarks.sh --save-results
./scripts/run-ci-locally.sh
```

## 🔧 Requirements

### Essential (included with Rust)
- `cargo` (Rust package manager)
- `rustc` (Rust compiler)

### Optional Enhancements
```bash
# For security auditing
cargo install cargo-audit

# For dependency checking
cargo install cargo-outdated

# For test coverage
cargo install cargo-tarpaulin

# For cross-compilation
cargo install cross
```

## 🏃‍♂️ Quick Start

1. **Make scripts executable** (already done):
   ```bash
   chmod +x scripts/*.sh
   ```

2. **Run quick validation**:
   ```bash
   ./scripts/quick-check.sh
   ```

3. **If quick check passes, run full CI**:
   ```bash
   ./scripts/run-ci-locally.sh
   ```

## 📊 Expected Results

### ✅ Passing Results
```
🎉 All critical checks passed! (X/X)
✅ Safe to push to GitHub
```

### ❌ Failing Results
```
❌ Some checks failed: X failed, X passed out of X total
❌ Please fix failing checks before pushing
```

## 🐛 Troubleshooting

### Common Issues

**Formatting Failures:**
```bash
cargo fmt --all
```

**Clippy Warnings:**
```bash
cargo clippy --fix --all-targets --all-features
```

**Test Failures:**
```bash
cargo test --verbose -- --nocapture
```

**Build Errors:**
```bash
cargo build --verbose
cargo check
```

### Performance Issues

**Slow Benchmarks:**
- Ensure release build: `cargo build --release`
- Close other applications
- Run on consistent hardware

**Memory Issues:**
- Check with: `cargo test --release`
- Monitor with system tools during benchmarks

## 🎯 CI/CD Integration

These scripts match the GitHub Actions workflows:
- `.github/workflows/ci.yml` → `./scripts/run-ci-locally.sh`
- Individual jobs → Specific scripts

**Workflow Jobs Coverage:**
- `quality` → Code formatting & lints
- `test` → Comprehensive testing
- `chess_validation` → Engine validation
- `platform_builds` → Cross-compilation
- `benchmarks` → Performance testing

## 📈 Performance Targets

**Expected Benchmarks:**
- **Move Generation**: >1M moves/second
- **Position Evaluation**: >100K positions/second
- **Search Depth**: 6+ plies in <5 seconds
- **Memory Usage**: <100MB for typical games

**Test Coverage:**
- **Unit Tests**: 82 tests passing
- **Chess Rules**: 27 validation tests passing
- **Integration**: All scenarios covered

## 🔗 Integration with Development Workflow

### Pre-commit Hook Setup
```bash
# Add to .git/hooks/pre-commit
#!/bin/bash
./scripts/quick-check.sh
```

### IDE Integration
Most IDEs can run these scripts as build tasks:
- **VS Code**: Add to `.vscode/tasks.json`
- **IntelliJ**: Add as external tools
- **Vim/Neovim**: Add as makeprg

## 📚 Additional Resources

- [GitHub Actions Workflows](../.github/workflows/)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Cross-compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)

---

*Happy coding! 🦀♟️*