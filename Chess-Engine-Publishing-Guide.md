# Chess Engine Rust - Multi-Platform Publishing Guide

A comprehensive guide for publishing and integrating the Chess Engine Rust across different platforms and package managers.

## 📦 Overview

The Chess Engine Rust is designed to be integrated into applications across multiple platforms and languages. This guide covers publishing strategies, integration methods, and platform-specific considerations.

## 🦀 Rust Ecosystem

### Crates.io Publishing

**Primary Package**: `chess-engine-rust`
```toml
[dependencies]
chess-engine-rust = "0.1.0"
```

**Publishing Steps**:
1. Update version in all `Cargo.toml` files
2. Run comprehensive tests: `./comprehensive_test.sh`
3. Generate documentation: `cargo doc --no-deps`
4. Publish core crate: `cargo publish -p chess-core`
5. Publish engine crate: `cargo publish -p chess-engine`
6. Publish bindings: `cargo publish -p chess-ffi`

**Integration Example**:
```rust
use chess_engine_rust::{ChessEngine, Color};

let mut engine = ChessEngine::new();
engine.initialize()?;
engine.make_move_from_uci("e2e4")?;
let best_move = engine.find_best_move()?;
```

## 🐍 Python Ecosystem

### PyPI Publishing

**Package Name**: `chess-engine-rust`
```bash
pip install chess-engine-rust
```

**Build & Publish**:
```bash
# Install maturin for Python bindings
pip install maturin

# Build Python wheels
maturin build --release --features python

# Publish to PyPI
maturin publish --features python
```

**Integration Example**:
```python
from chess_engine_rust import ChessEngine

engine = ChessEngine(depth=6, threads=2)
engine.make_move("e2e4")
best_move = engine.find_best_move()
print(f"Engine suggests: {best_move}")
```

**Platform Wheels**:
- `chess_engine_rust-0.1.0-cp38-abi3-linux_x86_64.whl`
- `chess_engine_rust-0.1.0-cp38-abi3-macosx_10_12_x86_64.whl`
- `chess_engine_rust-0.1.0-cp38-abi3-win_amd64.whl`

## ☕ Java/Kotlin Ecosystem

### Maven Central Publishing

**Group ID**: `io.github.chess-engine`
**Artifact ID**: `chess-engine-rust`

```xml
<dependency>
    <groupId>io.github.chess-engine</groupId>
    <artifactId>chess-engine-rust</artifactId>
    <version>0.1.0</version>
</dependency>
```

**Gradle**:
```kotlin
implementation("io.github.chess-engine:chess-engine-rust:0.1.0")
```

**Build & Publish**:
```bash
# Build JNI library
cargo build --release -p chess-jni

# Create JAR with native libraries
./scripts/build-java-jar.sh

# Publish to Maven Central
./gradlew publishToMavenCentral
```

**Integration Example**:
```java
import io.github.chessengine.ChessEngine;

ChessEngine engine = new ChessEngine(6, 2); // depth, threads
engine.makeMove("d2d4");
String bestMove = engine.findBestMove();
System.out.println("Best move: " + bestMove);
```

**Platform Libraries**:
- `libchess_jni.so` (Linux)
- `chess_jni.dll` (Windows)
- `libchess_jni.dylib` (macOS)

## 🌐 JavaScript/Node.js Ecosystem

### NPM Publishing

**Package Name**: `@chess-engine/chess-engine-rust`
```bash
npm install @chess-engine/chess-engine-rust
```

**WebAssembly Build**:
```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build WebAssembly package
wasm-pack build --target web --out-dir pkg --features wasm

# Publish to NPM
cd pkg && npm publish --access public
```

**Integration Example**:
```javascript
import init, { ChessEngine } from '@chess-engine/chess-engine-rust';

async function playChess() {
    await init();

    const engine = new ChessEngine(5); // depth
    engine.makeMove("e2e4");
    const bestMove = engine.findBestMove();
    console.log(`Engine plays: ${bestMove}`);
}
```

**Package Structure**:
```
pkg/
├── chess_engine_rust.js
├── chess_engine_rust_bg.wasm
├── chess_engine_rust.d.ts
└── package.json
```

## 📱 Mobile Platforms

### Android (AAR)

**Publishing**: JitPack or Maven Central
```gradle
implementation 'io.github.chess-engine:chess-engine-android:0.1.0'
```

**Build Steps**:
```bash
# Build for Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi
cargo build --target aarch64-linux-android --release -p chess-jni
cargo build --target armv7-linux-androideabi --release -p chess-jni

# Create AAR
./scripts/build-android-aar.sh
```

**Kotlin Integration**:
```kotlin
import io.github.chessengine.ChessEngine

val engine = ChessEngine(6, 1)
engine.makeMove("e2e4")
val bestMove = engine.findBestMove()
println("Best move: $bestMove")
```

### iOS (XCFramework)

**Distribution**: Swift Package Manager or CocoaPods
```swift
.package(url: "https://github.com/chess-engine/chess-engine-rust", from: "0.1.0")
```

**Build Steps**:
```bash
# Build for iOS targets
rustup target add aarch64-apple-ios x86_64-apple-ios
cargo build --target aarch64-apple-ios --release -p chess-ffi
cargo build --target x86_64-apple-ios --release -p chess-ffi

# Create XCFramework
./scripts/build-ios-xcframework.sh
```

**Swift Integration**:
```swift
import ChessEngineRust

let engine = ChessEngine(depth: 6)
engine.makeMove("e2e4")
let bestMove = engine.findBestMove()
print("Best move: \(bestMove)")
```

## 🔧 C/C++ Integration

### Package Managers

**vcpkg**:
```bash
vcpkg install chess-engine-rust
```

**Conan**:
```python
[requires]
chess-engine-rust/0.1.0
```

**Build & Install**:
```bash
# Build static/dynamic libraries
cargo build --release -p chess-ffi

# Install system-wide (Linux/macOS)
sudo cp target/release/libchess_ffi.so /usr/local/lib/
sudo cp chess-ffi.h /usr/local/include/
```

**Integration Example**:
```cpp
#include "chess-ffi.h"

int main() {
    long engine_id = chess_engine_create();
    chess_engine_initialize(engine_id);

    chess_engine_make_move(engine_id, "e2e4");
    char* best_move = chess_engine_find_best_move(engine_id);

    printf("Best move: %s\n", best_move);

    chess_engine_free_string(best_move);
    chess_engine_destroy(engine_id);
    return 0;
}
```

## 🎮 Game Engine Integrations

### Unity (C# Bindings)

**Package**: Unity Package Manager
```json
{
    "name": "io.github.chess-engine.unity",
    "version": "0.1.0",
    "displayName": "Chess Engine Rust"
}
```

**Integration**:
```csharp
using ChessEngine;

public class ChessGame : MonoBehaviour {
    private ChessEngineWrapper engine;

    void Start() {
        engine = new ChessEngineWrapper(6); // depth
        engine.MakeMove("e2e4");
        string bestMove = engine.FindBestMove();
        Debug.Log($"Best move: {bestMove}");
    }
}
```

### Unreal Engine (C++ Plugin)

**Plugin Structure**:
```
ChessEngineRust/
├── ChessEngineRust.uplugin
├── Source/
│   ├── ChessEngineRust/
│   │   ├── Public/ChessEngineRust.h
│   │   └── Private/ChessEngineRust.cpp
└── ThirdParty/
    └── chess-ffi/
        ├── lib/libchess_ffi.so
        └── include/chess-ffi.h
```

## 📋 Publishing Checklist

### Pre-Release
- [ ] Update version numbers in all `Cargo.toml` files
- [ ] Run comprehensive test suite: `./comprehensive_test.sh`
- [ ] Update `CHANGELOG.md` with new features/fixes
- [ ] Generate and review documentation
- [ ] Test examples across platforms
- [ ] Benchmark performance regression tests

### Release Process
- [ ] **Rust**: Publish to crates.io
- [ ] **Python**: Build wheels and publish to PyPI
- [ ] **Java**: Publish JAR to Maven Central
- [ ] **JavaScript**: Build WASM and publish to NPM
- [ ] **C/C++**: Update package manager recipes
- [ ] **Mobile**: Update platform-specific packages

### Post-Release
- [ ] Create GitHub release with binaries
- [ ] Update documentation website
- [ ] Announce on relevant forums/communities
- [ ] Update integration examples
- [ ] Monitor for issues and feedback

## 🔗 Distribution Channels

### Primary Repositories
- **Rust**: [crates.io](https://crates.io/crates/chess-engine-rust)
- **Python**: [PyPI](https://pypi.org/project/chess-engine-rust/)
- **Java**: [Maven Central](https://search.maven.org/artifact/io.github.chess-engine/chess-engine-rust)
- **JavaScript**: [NPM](https://www.npmjs.com/package/@chess-engine/chess-engine-rust)

### Secondary Channels
- **GitHub Releases**: Pre-built binaries for direct download
- **Docker Hub**: Containerized chess engine for microservices
- **Homebrew**: macOS package manager formula
- **Chocolatey**: Windows package manager
- **Snap**: Linux universal package format

## 📖 Integration Examples Repository

Create a separate repository with integration examples:
```
chess-engine-integrations/
├── rust-example/
├── python-example/
├── java-example/
├── javascript-example/
├── android-example/
├── ios-example/
├── cpp-example/
├── unity-example/
└── unreal-example/
```

Each example should be a complete, runnable project demonstrating:
- Installation/setup process
- Basic engine usage
- Move making and validation
- Position evaluation
- Game state management
- Performance optimization tips

## 🎯 Target Audiences

1. **Chess Application Developers** - GUI chess games
2. **Game Developers** - Integrating chess into larger games
3. **Chess Analysis Tools** - Position analysis and training
4. **Educational Platforms** - Teaching chess programming
5. **Research/AI** - Chess engine experiments
6. **Tournament Software** - Competition management

## 📊 Success Metrics

- **Download counts** across all package managers
- **GitHub stars/forks** as community indicators
- **Integration examples** submitted by users
- **Performance benchmarks** vs other engines
- **Issue resolution time** for user problems
- **Documentation coverage** and clarity

This publishing strategy ensures the chess engine is accessible across all major platforms while maintaining high quality and ease of integration for developers.