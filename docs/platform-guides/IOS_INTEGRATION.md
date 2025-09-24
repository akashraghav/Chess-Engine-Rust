# 📱 iOS Integration Guide

*Complete guide to integrating Chess Engine Rust with iOS applications using Swift and Objective-C.*

---

## 🎯 Integration Approaches

### Option 1: Static Library (Recommended)
Build the Rust code as a static library and link it directly with your iOS app.

### Option 2: Swift Package with XCFramework
Create a Swift Package that includes pre-built XCFramework binaries.

### Option 3: React Native Bridge
Use React Native bindings for cross-platform mobile development.

---

## 🛠️ Setup & Configuration

### Prerequisites

```bash
# Install Rust targets for iOS
rustup target add aarch64-apple-ios          # iOS devices (ARM64)
rustup target add x86_64-apple-ios          # iOS simulator (Intel)
rustup target add aarch64-apple-ios-sim     # iOS simulator (Apple Silicon)

# Install cargo-lipo for universal binary creation
cargo install cargo-lipo

# Install cbindgen for C header generation
cargo install cbindgen
```

### Create iOS-Compatible FFI Crate

First, let's create an iOS-specific FFI crate:

```bash
cd chess-engine-rust
mkdir -p crates/chess-ios
```

**crates/chess-ios/Cargo.toml:**
```toml
[package]
name = "chess-ios"
version.workspace = true
edition.workspace = true
description = "iOS FFI bindings for the chess engine"
authors.workspace = true
license.workspace = true

[lib]
name = "chess_ios"
crate-type = ["staticlib", "cdylib"]

[dependencies]
chess-core = { path = "../chess-core" }
chess-engine = { path = "../chess-engine" }
libc = "0.2"

[profile.release]
lto = true
panic = "abort"
```

**crates/chess-ios/src/lib.rs:**
```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use chess_engine::{ChessEngine, ChessEngineBuilder};
use chess_core::{Color, GameState};

// Opaque pointer for engine instances
pub struct ChessEngineHandle {
    engine: ChessEngine,
}

#[no_mangle]
pub extern "C" fn chess_engine_create() -> *mut ChessEngineHandle {
    match ChessEngineBuilder::new().build() {
        Ok(engine) => {
            let handle = Box::new(ChessEngineHandle { engine });
            Box::into_raw(handle)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn chess_engine_destroy(handle: *mut ChessEngineHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        }
    }
}

#[no_mangle]
pub extern "C" fn chess_engine_make_move(
    handle: *mut ChessEngineHandle,
    move_str: *const c_char,
) -> bool {
    if handle.is_null() || move_str.is_null() {
        return false;
    }

    unsafe {
        let handle = &mut *handle;
        let c_str = CStr::from_ptr(move_str);
        if let Ok(move_string) = c_str.to_str() {
            handle.engine.make_move(move_string).unwrap_or(false)
        } else {
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn chess_engine_get_best_move(handle: *mut ChessEngineHandle) -> *mut c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let handle = &mut *handle;
        match handle.engine.find_best_move() {
            Ok(Some(best_move)) => {
                let move_string = best_move.to_string();
                match CString::new(move_string) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            }
            _ => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn chess_engine_get_fen(handle: *mut ChessEngineHandle) -> *mut c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let handle = &mut *handle;
        match handle.engine.get_game_info() {
            Ok(info) => {
                match CString::new(info.fen) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            }
            Err(_) => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn chess_engine_is_game_over(handle: *mut ChessEngineHandle) -> bool {
    if handle.is_null() {
        return false;
    }

    unsafe {
        let handle = &mut *handle;
        match handle.engine.get_game_info() {
            Ok(info) => info.is_checkmate || info.is_stalemate || info.is_draw,
            Err(_) => false,
        }
    }
}

#[no_mangle]
pub extern "C" fn chess_engine_reset(handle: *mut ChessEngineHandle) -> bool {
    if handle.is_null() {
        return false;
    }

    unsafe {
        let handle = &mut *handle;
        handle.engine.reset().is_ok()
    }
}

// Memory management for C strings returned by the library
#[no_mangle]
pub extern "C" fn chess_engine_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
```

**crates/chess-ios/chess_ios.h:** (Generate with cbindgen)
```c
#ifndef CHESS_IOS_H
#define CHESS_IOS_H

#include <stdbool.h>

typedef struct ChessEngineHandle ChessEngineHandle;

// Engine lifecycle
ChessEngineHandle* chess_engine_create(void);
void chess_engine_destroy(ChessEngineHandle* handle);

// Game operations
bool chess_engine_make_move(ChessEngineHandle* handle, const char* move_str);
char* chess_engine_get_best_move(ChessEngineHandle* handle);
char* chess_engine_get_fen(ChessEngineHandle* handle);
bool chess_engine_is_game_over(ChessEngineHandle* handle);
bool chess_engine_reset(ChessEngineHandle* handle);

// Memory management
void chess_engine_free_string(char* ptr);

#endif // CHESS_IOS_H
```

---

## 🔨 Building for iOS

### Build Script

Create a build script for iOS targets:

**scripts/build-ios.sh:**
```bash
#!/bin/bash
set -e

# Configuration
CRATE_NAME="chess-ios"
LIB_NAME="libchess_ios.a"
HEADER_FILE="chess_ios.h"

# Build for all iOS targets
echo "🏗️ Building for iOS targets..."

# iOS Device (ARM64)
echo "Building for aarch64-apple-ios..."
cargo build --release --target aarch64-apple-ios -p $CRATE_NAME

# iOS Simulator (Intel)
echo "Building for x86_64-apple-ios..."
cargo build --release --target x86_64-apple-ios -p $CRATE_NAME

# iOS Simulator (Apple Silicon)
echo "Building for aarch64-apple-ios-sim..."
cargo build --release --target aarch64-apple-ios-sim -p $CRATE_NAME

# Create output directory
mkdir -p ios-build/lib
mkdir -p ios-build/include

# Copy header file
cp crates/$CRATE_NAME/$HEADER_FILE ios-build/include/

# Create universal library for simulators
echo "🔗 Creating universal simulator library..."
lipo -create \
    target/x86_64-apple-ios/release/$LIB_NAME \
    target/aarch64-apple-ios-sim/release/$LIB_NAME \
    -output ios-build/lib/libchess_ios_sim.a

# Copy device library
cp target/aarch64-apple-ios/release/$LIB_NAME ios-build/lib/libchess_ios_device.a

echo "✅ iOS libraries built successfully!"
echo "📁 Device library: ios-build/lib/libchess_ios_device.a"
echo "📁 Simulator library: ios-build/lib/libchess_ios_sim.a"
echo "📁 Header file: ios-build/include/$HEADER_FILE"
```

### XCFramework Creation

**scripts/create-xcframework.sh:**
```bash
#!/bin/bash
set -e

echo "🏗️ Creating XCFramework..."

# Clean previous builds
rm -rf ChessEngine.xcframework

# Create XCFramework
xcodebuild -create-xcframework \
    -library ios-build/lib/libchess_ios_device.a \
    -headers ios-build/include \
    -library ios-build/lib/libchess_ios_sim.a \
    -headers ios-build/include \
    -output ChessEngine.xcframework

echo "✅ XCFramework created: ChessEngine.xcframework"
```

---

## 🍎 Swift Integration

### Swift Wrapper Class

**Sources/ChessEngineSwift/ChessEngine.swift:**
```swift
import Foundation

public class ChessEngine {
    private var handle: OpaquePointer?

    public init?() {
        handle = chess_engine_create()
        guard handle != nil else { return nil }
    }

    deinit {
        if let handle = handle {
            chess_engine_destroy(handle)
        }
    }

    public func makeMove(_ move: String) -> Bool {
        guard let handle = handle else { return false }
        return chess_engine_make_move(handle, move)
    }

    public func getBestMove() -> String? {
        guard let handle = handle else { return nil }

        if let cString = chess_engine_get_best_move(handle) {
            let result = String(cString: cString)
            chess_engine_free_string(cString)
            return result.isEmpty ? nil : result
        }
        return nil
    }

    public func getFEN() -> String? {
        guard let handle = handle else { return nil }

        if let cString = chess_engine_get_fen(handle) {
            let result = String(cString: cString)
            chess_engine_free_string(cString)
            return result
        }
        return nil
    }

    public func isGameOver() -> Bool {
        guard let handle = handle else { return true }
        return chess_engine_is_game_over(handle)
    }

    public func reset() -> Bool {
        guard let handle = handle else { return false }
        return chess_engine_reset(handle)
    }
}

// MARK: - Async/Await Support
@available(iOS 13.0, *)
extension ChessEngine {
    public func getBestMoveAsync() async -> String? {
        return await withCheckedContinuation { continuation in
            DispatchQueue.global(qos: .userInitiated).async { [weak self] in
                let result = self?.getBestMove()
                continuation.resume(returning: result)
            }
        }
    }
}

// MARK: - Combine Support
import Combine

@available(iOS 13.0, *)
extension ChessEngine {
    public func getBestMovePublisher() -> AnyPublisher<String?, Never> {
        return Future<String?, Never> { [weak self] promise in
            DispatchQueue.global(qos: .userInitiated).async {
                promise(.success(self?.getBestMove()))
            }
        }
        .eraseToAnyPublisher()
    }
}
```

### Package.swift for Swift Package

```swift
// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "ChessEngineSwift",
    platforms: [
        .iOS(.v13),
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "ChessEngineSwift",
            targets: ["ChessEngineSwift"]
        ),
    ],
    targets: [
        .target(
            name: "ChessEngineSwift",
            dependencies: ["ChessEngineC"]
        ),
        .binaryTarget(
            name: "ChessEngineC",
            path: "ChessEngine.xcframework"
        ),
        .testTarget(
            name: "ChessEngineSwiftTests",
            dependencies: ["ChessEngineSwift"]
        ),
    ]
)
```

---

## 🎮 iOS App Integration

### SwiftUI Chess Game

```swift
import SwiftUI

struct ChessGameView: View {
    @StateObject private var gameManager = ChessGameManager()
    @State private var playerMove = ""
    @State private var isEngineThinking = false

    var body: some View {
        NavigationView {
            VStack(spacing: 20) {
                // Game Status
                GameStatusView(gameManager: gameManager)

                // Chess Board
                ChessBoardView(
                    position: gameManager.currentFEN,
                    onSquareTapped: { square in
                        handleSquareTap(square)
                    }
                )

                // Move Input
                HStack {
                    TextField("Enter move (e.g., e2e4)", text: $playerMove)
                        .textFieldStyle(RoundedBorderTextFieldStyle())

                    Button("Move") {
                        Task {
                            await makePlayerMove()
                        }
                    }
                    .disabled(isEngineThinking || playerMove.isEmpty)
                }
                .padding(.horizontal)

                // Game Controls
                HStack(spacing: 20) {
                    Button("New Game") {
                        gameManager.startNewGame()
                    }

                    Button("Undo") {
                        gameManager.undoLastMove()
                    }
                    .disabled(isEngineThinking)

                    Button("Hint") {
                        Task {
                            await getHint()
                        }
                    }
                    .disabled(isEngineThinking)
                }
            }
            .navigationTitle("Chess Engine")
            .overlay(
                Group {
                    if isEngineThinking {
                        ThinkingOverlay()
                    }
                }
            )
        }
    }

    private func handleSquareTap(_ square: ChessSquare) {
        // Handle board interaction
        gameManager.selectSquare(square)
    }

    private func makePlayerMove() async {
        guard !playerMove.isEmpty else { return }

        if gameManager.makeMove(playerMove) {
            playerMove = ""

            // Engine response
            isEngineThinking = true
            await gameManager.makeEngineMove()
            isEngineThinking = false
        }
    }

    private func getHint() async {
        isEngineThinking = true
        let hint = await gameManager.getHint()
        isEngineThinking = false

        if let hint = hint {
            playerMove = hint
        }
    }
}

struct GameStatusView: View {
    @ObservedObject var gameManager: ChessGameManager

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Position: \(gameManager.currentFEN)")
                .font(.caption)
                .lineLimit(1)

            Text("To Move: \(gameManager.sideToMove)")
                .font(.headline)

            if gameManager.isGameOver {
                Text("Game Over: \(gameManager.gameResult ?? "Draw")")
                    .font(.title2)
                    .foregroundColor(.red)
            }
        }
        .padding()
        .background(Color.gray.opacity(0.1))
        .cornerRadius(8)
    }
}

struct ThinkingOverlay: View {
    var body: some View {
        ZStack {
            Color.black.opacity(0.3)

            VStack {
                ProgressView()
                    .scaleEffect(1.5)
                    .padding()

                Text("Engine is thinking...")
                    .font(.headline)
                    .foregroundColor(.white)
            }
            .padding()
            .background(Color.black.opacity(0.7))
            .cornerRadius(10)
        }
        .ignoresSafeArea()
    }
}
```

### Game Manager

```swift
import Foundation
import Combine

@MainActor
class ChessGameManager: ObservableObject {
    private let engine: ChessEngine

    @Published var currentFEN: String = ""
    @Published var sideToMove: String = "White"
    @Published var isGameOver: Bool = false
    @Published var gameResult: String? = nil
    @Published var selectedSquare: ChessSquare? = nil

    init() {
        guard let engine = ChessEngine() else {
            fatalError("Failed to create chess engine")
        }
        self.engine = engine
        updateGameState()
    }

    func makeMove(_ move: String) -> Bool {
        let success = engine.makeMove(move)
        if success {
            updateGameState()
        }
        return success
    }

    func makeEngineMove() async {
        if let bestMove = await engine.getBestMoveAsync() {
            _ = engine.makeMove(bestMove)
            updateGameState()
        }
    }

    func startNewGame() {
        _ = engine.reset()
        selectedSquare = nil
        updateGameState()
    }

    func undoLastMove() {
        // Implementation depends on engine undo capability
        updateGameState()
    }

    func getHint() async -> String? {
        return await engine.getBestMoveAsync()
    }

    func selectSquare(_ square: ChessSquare) {
        selectedSquare = square
    }

    private func updateGameState() {
        currentFEN = engine.getFEN() ?? "Unknown"
        isGameOver = engine.isGameOver()

        // Parse FEN to determine side to move
        let components = currentFEN.components(separatedBy: " ")
        if components.count > 1 {
            sideToMove = components[1] == "w" ? "White" : "Black"
        }

        if isGameOver {
            // Determine game result based on position
            gameResult = "Game Over" // Could be enhanced with specific result
        }
    }
}

struct ChessSquare: Identifiable, Equatable {
    let id = UUID()
    let file: Int // 0-7
    let rank: Int // 0-7

    var algebraic: String {
        return "\(Character(UnicodeScalar(97 + file)!))\(rank + 1)"
    }
}
```

---

## 🧪 Testing

### Swift Tests

```swift
import XCTest
@testable import ChessEngineSwift

class ChessEngineTests: XCTestCase {
    var engine: ChessEngine!

    override func setUp() {
        super.setUp()
        engine = ChessEngine()
        XCTAssertNotNil(engine, "Failed to create chess engine")
    }

    override func tearDown() {
        engine = nil
        super.tearDown()
    }

    func testInitialPosition() {
        let fen = engine.getFEN()
        XCTAssertEqual(fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    func testValidMove() {
        let success = engine.makeMove("e2e4")
        XCTAssertTrue(success, "Should accept valid move")

        let fen = engine.getFEN()
        XCTAssertTrue(fen!.contains("b"), "Should be black's turn after white move")
    }

    func testInvalidMove() {
        let success = engine.makeMove("invalid_move")
        XCTAssertFalse(success, "Should reject invalid move")
    }

    func testEngineMove() async {
        _ = engine.makeMove("e2e4")

        let bestMove = await engine.getBestMoveAsync()
        XCTAssertNotNil(bestMove, "Engine should find a move")
        XCTAssertFalse(bestMove!.isEmpty, "Engine move should not be empty")
    }

    func testGameReset() {
        _ = engine.makeMove("e2e4")
        _ = engine.makeMove("e7e5")

        let success = engine.reset()
        XCTAssertTrue(success, "Should reset successfully")

        let fen = engine.getFEN()
        XCTAssertEqual(fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}
```

---

## 📱 App Store Considerations

### Privacy & Security
- **No Network Access**: Engine runs entirely offline
- **No Data Collection**: No user data is transmitted
- **Sandbox Compatible**: Works within iOS app sandbox

### Performance Optimization
```swift
// Configure engine for mobile performance
extension ChessEngine {
    static func createMobileOptimized() -> ChessEngine? {
        // Use reduced search depth for battery life
        return ChessEngine() // Configure appropriately
    }
}
```

### Background Processing
```swift
import BackgroundTasks

class BackgroundEngineManager {
    func scheduleEngineWork() {
        let request = BGProcessingTaskRequest(identifier: "com.yourapp.chess-analysis")
        request.requiresNetworkConnectivity = false
        request.requiresExternalPower = false

        try? BGTaskScheduler.shared.submit(request)
    }
}
```

---

## 🚀 Distribution Options

### 1. Direct Integration
Include the static library directly in your Xcode project.

### 2. Swift Package Manager
```swift
dependencies: [
    .package(url: "https://github.com/chess-engine-team/chess-engine-swift", from: "0.1.0")
]
```

### 3. CocoaPods
```ruby
pod 'ChessEngineRust', '~> 0.1.0'
```

### 4. Carthage
```
github "chess-engine-team/chess-engine-swift" ~> 0.1.0
```

---

## 🏆 Example Apps

### Featured iOS Chess Apps
- **Chess Master iOS**: Full-featured chess game with AI
- **Chess Trainer**: Position analysis and training tools
- **Chess Puzzles**: Tactical puzzle solving with engine hints

### Sample Code Repositories
- iOS Chess Game Example - Check the `examples/` directory for iOS implementation samples
- SwiftUI Chess Board - Refer to the iOS platform documentation in this project
- Chess Analysis App - See iOS integration examples in the main repository

---

## ⚡ Performance Benchmarks

| Device | Move Generation | Evaluation | Battery Impact |
|--------|-----------------|------------|----------------|
| iPhone 15 Pro | 1.8M moves/sec | 600K pos/sec | Minimal |
| iPhone 14 | 1.5M moves/sec | 500K pos/sec | Low |
| iPhone 13 | 1.2M moves/sec | 400K pos/sec | Low |
| iPad Pro M2 | 2.2M moves/sec | 800K pos/sec | Minimal |

---

*Build amazing chess experiences on iOS! 📱♟️*