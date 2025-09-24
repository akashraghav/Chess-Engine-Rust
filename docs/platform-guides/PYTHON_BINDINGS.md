# 🐍 Python Integration Guide

*Complete guide to using Chess Engine Rust in your Python projects for game development, AI research, and chess analysis.*

---

## 🚀 Quick Installation

```bash
# Install from PyPI
pip install chess-engine-rust

# Or with specific version
pip install chess-engine-rust==0.1.0

# Development installation
pip install chess-engine-rust[dev]  # Includes testing tools
```

---

## 🎯 Basic Usage

### Simple Chess Game

```python
import chess_engine_rust as chess

def basic_game():
    """Play a simple game against the engine."""

    # Create engine with default settings
    engine = chess.ChessEngine()

    print("🏁 New chess game started!")
    print(f"📋 Starting position: {engine.get_fen()}")

    while not engine.is_game_over():
        # Display current position
        print(f"\n♟️ Current position ({engine.get_side_to_move()} to move):")
        print(f"📋 FEN: {engine.get_fen()}")

        legal_moves = engine.get_legal_moves()
        print(f"🎯 Legal moves ({len(legal_moves)}): {legal_moves[:5]}...")  # Show first 5

        if engine.get_side_to_move() == "White":
            # Human player's turn
            while True:
                move = input("Enter your move (e.g., 'e2e4'): ").strip()
                try:
                    engine.make_move(move)
                    print(f"✅ Move made: {move}")
                    break
                except Exception as e:
                    print(f"❌ Invalid move: {e}")
        else:
            # Engine's turn
            print("🤖 Engine is thinking...")
            best_move = engine.find_best_move()
            if best_move:
                engine.make_move(best_move)
                print(f"🎯 Engine plays: {best_move}")
                print(f"📊 Evaluation: {engine.get_evaluation()} centipawns")

    # Game over
    result = engine.get_game_result()
    print(f"\n🏁 Game over! Result: {result}")

if __name__ == "__main__":
    basic_game()
```

### Advanced Engine Configuration

```python
import chess_engine_rust as chess
from datetime import timedelta

def create_advanced_engine():
    """Create a tournament-strength engine with advanced configuration."""

    config = chess.EngineConfig(
        # Search settings
        depth=10,                              # 10-ply deep search
        time_limit=timedelta(seconds=5),       # 5 second time limit
        use_iterative_deepening=True,          # Gradual depth increase

        # Evaluation settings
        use_advanced_evaluation=True,          # Better position assessment
        enable_endgame_tables=True,           # Endgame knowledge

        # Performance settings
        threads=4,                            # Multi-threading
        hash_size_mb=256,                     # 256MB transposition table

        # Features
        use_opening_book=True,                # Opening knowledge
        enable_pondering=False,               # Don't think on opponent time
        debug_mode=False                      # Production mode
    )

    engine = chess.ChessEngine(config)

    # Test the configuration
    print(f"🔧 Engine configuration:")
    print(f"   Search depth: {config.depth}")
    print(f"   Time limit: {config.time_limit.total_seconds()}s")
    print(f"   Threads: {config.threads}")
    print(f"   Hash size: {config.hash_size_mb}MB")

    return engine
```

---

## 📊 Chess Analysis and Research

### Position Analysis

```python
import chess_engine_rust as chess
import matplotlib.pyplot as plt
import pandas as pd

class ChessAnalyzer:
    """Advanced chess position analyzer using the engine."""

    def __init__(self, depth=12):
        config = chess.EngineConfig(
            depth=depth,
            time_limit=timedelta(seconds=10),
            use_advanced_evaluation=True,
            threads=4
        )
        self.engine = chess.ChessEngine(config)
        self.analysis_history = []

    def analyze_position(self, fen, depth=None):
        """Analyze a single position in depth."""

        # Set position
        self.engine.set_position(fen)

        # Get comprehensive analysis
        analysis = {
            'fen': fen,
            'evaluation': self.engine.get_evaluation(),
            'best_move': self.engine.find_best_move(),
            'legal_moves_count': len(self.engine.get_legal_moves()),
            'is_check': self.engine.is_in_check(),
            'is_checkmate': self.engine.is_checkmate(),
            'is_stalemate': self.engine.is_stalemate(),
            'game_phase': self.engine.get_game_phase(),  # Opening/Middlegame/Endgame
        }

        # Get top candidate moves with evaluations
        top_moves = self.engine.get_top_moves(5)  # Top 5 moves
        analysis['top_moves'] = top_moves

        # Advanced analysis
        if depth:
            deep_analysis = self.engine.analyze_position(depth=depth)
            analysis.update({
                'principal_variation': deep_analysis.principal_variation,
                'nodes_searched': deep_analysis.nodes_searched,
                'time_taken': deep_analysis.time_taken,
                'nps': deep_analysis.nodes_per_second
            })

        self.analysis_history.append(analysis)
        return analysis

    def analyze_opening(self, moves_list):
        """Analyze an opening sequence move by move."""

        self.engine.reset_to_starting_position()
        evaluations = []
        positions = []

        print("📈 Opening Analysis:")
        print("=" * 50)

        for i, move in enumerate(moves_list, 1):
            # Make move
            self.engine.make_move(move)

            # Analyze resulting position
            evaluation = self.engine.get_evaluation()
            fen = self.engine.get_fen()
            best_response = self.engine.find_best_move()

            evaluations.append(evaluation)
            positions.append(fen)

            print(f"{i:2d}. {move:8} | {evaluation:+4d} cp | Best: {best_response or 'None'}")

            # Show significant evaluation changes
            if i > 1:
                change = evaluation - evaluations[-2]
                if abs(change) > 50:  # Significant change
                    symbol = "📈" if change > 0 else "📉"
                    print(f"     {symbol} Evaluation swing: {change:+d} cp")

        return evaluations, positions

    def plot_evaluation_curve(self, evaluations, title="Position Evaluation"):
        """Plot evaluation curve over moves."""

        plt.figure(figsize=(12, 6))
        moves = list(range(1, len(evaluations) + 1))

        plt.plot(moves, evaluations, 'b-', linewidth=2, label='Evaluation')
        plt.axhline(y=0, color='gray', linestyle='--', alpha=0.5)

        # Highlight significant moments
        for i, eval_score in enumerate(evaluations):
            if abs(eval_score) > 200:  # Significant advantage
                color = 'green' if eval_score > 0 else 'red'
                plt.scatter(i+1, eval_score, color=color, s=100, alpha=0.7)

        plt.xlabel('Move Number')
        plt.ylabel('Evaluation (centipawns)')
        plt.title(title)
        plt.grid(True, alpha=0.3)
        plt.legend()

        # Add evaluation bands
        plt.axhspan(100, 300, alpha=0.1, color='green', label='White advantage')
        plt.axhspan(-100, -300, alpha=0.1, color='red', label='Black advantage')

        plt.tight_layout()
        plt.show()

    def find_tactical_motifs(self, fen):
        """Identify tactical patterns in a position."""

        self.engine.set_position(fen)

        # Look for tactical indicators
        tactics = {
            'pins': [],
            'forks': [],
            'skewers': [],
            'discovered_attacks': [],
            'double_attacks': []
        }

        # Analyze all legal moves for tactical themes
        legal_moves = self.engine.get_legal_moves()

        for move in legal_moves:
            # Make move temporarily
            self.engine.make_move(move)
            evaluation = self.engine.get_evaluation()

            # Check if move creates significant advantage (potential tactic)
            if evaluation > 150:  # 1.5 pawn advantage
                tactics['tactical_moves'] = tactics.get('tactical_moves', [])
                tactics['tactical_moves'].append({
                    'move': move,
                    'evaluation': evaluation,
                    'type': self._classify_tactical_move(move, evaluation)
                })

            # Undo move
            self.engine.undo_last_move()

        return tactics

    def _classify_tactical_move(self, move, evaluation):
        """Classify the type of tactical move."""
        if evaluation > 500:
            return "winning_tactic"
        elif evaluation > 300:
            return "major_tactic"
        else:
            return "minor_tactic"

# Usage example
def analyze_famous_game():
    \"\"\"Analyze a famous chess game.\"\"\"

    analyzer = ChessAnalyzer(depth=10)

    # Immortal Game opening moves
    immortal_game_opening = [
        "e2e4", "e7e5", "f2f4", "e5f4", "f1c4", "d8h4",
        "e1f1", "b7b5", "c4b5", "g8f6", "g1f3", "h4h6"
    ]

    print("🏛️ Analyzing the Immortal Game opening...")
    evaluations, positions = analyzer.analyze_opening(immortal_game_opening)

    # Plot the evaluation curve
    analyzer.plot_evaluation_curve(
        evaluations,
        "Immortal Game - Opening Analysis"
    )

    # Analyze final position for tactics
    final_position = positions[-1]
    tactics = analyzer.find_tactical_motifs(final_position)

    print("\n🎯 Tactical analysis of final position:")
    if tactics.get('tactical_moves'):
        for tactic in tactics['tactical_moves']:
            print(f"   {tactic['move']}: {tactic['evaluation']} cp ({tactic['type']})")
    else:
        print("   No immediate tactics found")

if __name__ == "__main__":
    analyze_famous_game()
```

### Machine Learning Integration

```python
import chess_engine_rust as chess
import numpy as np
import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.ensemble import RandomForestRegressor
import joblib

class ChessML:
    """Machine learning integration with chess engine."""

    def __init__(self):
        self.engine = chess.ChessEngine()
        self.feature_extractor = chess.FeatureExtractor()

    def extract_position_features(self, fen):
        """Extract numerical features from a chess position."""

        self.engine.set_position(fen)

        # Basic features
        features = {
            # Material features
            'material_balance': self.engine.get_material_balance(),
            'piece_count_white': self.engine.count_pieces('white'),
            'piece_count_black': self.engine.count_pieces('black'),

            # Positional features
            'mobility_white': self.engine.get_mobility('white'),
            'mobility_black': self.engine.get_mobility('black'),
            'king_safety_white': self.engine.get_king_safety('white'),
            'king_safety_black': self.engine.get_king_safety('black'),

            # Control features
            'center_control_white': self.engine.get_center_control('white'),
            'center_control_black': self.engine.get_center_control('black'),

            # Structure features
            'pawn_structure_score': self.engine.evaluate_pawn_structure(),
            'doubled_pawns': self.engine.count_doubled_pawns(),
            'isolated_pawns': self.engine.count_isolated_pawns(),

            # Game phase
            'game_phase': self.engine.get_game_phase_numeric(),  # 0=opening, 1=middlegame, 2=endgame

            # Tactical features
            'pins_count': len(self.engine.find_pins()),
            'checks_available': self.engine.count_checking_moves(),
        }

        return features

    def generate_training_data(self, num_positions=10000):
        """Generate training data from random positions."""

        print(f"🎲 Generating {num_positions} training positions...")

        features_list = []
        evaluations = []

        for i in range(num_positions):
            if i % 1000 == 0:
                print(f"   Progress: {i}/{num_positions}")

            # Generate random position
            random_fen = self.engine.generate_random_position()

            # Extract features
            features = self.extract_position_features(random_fen)

            # Get engine evaluation as target
            evaluation = self.engine.get_evaluation()

            features_list.append(features)
            evaluations.append(evaluation)

        # Convert to DataFrame
        df = pd.DataFrame(features_list)
        df['evaluation'] = evaluations

        print(f"✅ Generated dataset with {len(df)} positions")
        return df

    def train_evaluation_model(self, training_data):
        """Train ML model to predict position evaluation."""

        print("🤖 Training evaluation prediction model...")

        # Prepare features and targets
        X = training_data.drop('evaluation', axis=1)
        y = training_data['evaluation']

        # Split data
        X_train, X_test, y_train, y_test = train_test_split(
            X, y, test_size=0.2, random_state=42
        )

        # Train Random Forest model
        model = RandomForestRegressor(
            n_estimators=100,
            max_depth=10,
            random_state=42,
            n_jobs=-1
        )

        model.fit(X_train, y_train)

        # Evaluate model
        train_score = model.score(X_train, y_train)
        test_score = model.score(X_test, y_test)

        print(f"   Training R² score: {train_score:.3f}")
        print(f"   Testing R² score: {test_score:.3f}")

        # Feature importance
        feature_importance = pd.DataFrame({
            'feature': X.columns,
            'importance': model.feature_importances_
        }).sort_values('importance', ascending=False)

        print("\n🎯 Top 5 most important features:")
        for _, row in feature_importance.head().iterrows():
            print(f"   {row['feature']}: {row['importance']:.3f}")

        return model, feature_importance

    def compare_ml_vs_engine(self, model, test_positions=100):
        """Compare ML model predictions with engine evaluations."""

        print(f"⚔️ Comparing ML model vs Engine on {test_positions} positions...")

        ml_predictions = []
        engine_evaluations = []

        for i in range(test_positions):
            # Generate random position
            fen = self.engine.generate_random_position()

            # Get ML prediction
            features = self.extract_position_features(fen)
            features_df = pd.DataFrame([features])
            ml_pred = model.predict(features_df)[0]

            # Get engine evaluation
            engine_eval = self.engine.get_evaluation()

            ml_predictions.append(ml_pred)
            engine_evaluations.append(engine_eval)

        # Calculate correlation
        correlation = np.corrcoef(ml_predictions, engine_evaluations)[0, 1]
        mae = np.mean(np.abs(np.array(ml_predictions) - np.array(engine_evaluations)))

        print(f"📊 Results:")
        print(f"   Correlation: {correlation:.3f}")
        print(f"   Mean Absolute Error: {mae:.1f} centipawns")

        return ml_predictions, engine_evaluations

# Usage example
def ml_chess_analysis():
    """Example of ML integration with chess engine."""

    ml_chess = ChessML()

    # Generate training data
    training_data = ml_chess.generate_training_data(num_positions=5000)

    # Train model
    model, importance = ml_chess.train_evaluation_model(training_data)

    # Save model
    joblib.dump(model, 'chess_evaluation_model.pkl')
    print("💾 Model saved to 'chess_evaluation_model.pkl'")

    # Compare ML vs Engine
    ml_preds, engine_evals = ml_chess.compare_ml_vs_engine(model, test_positions=100)

    # Plot comparison
    plt.figure(figsize=(10, 6))
    plt.scatter(engine_evals, ml_preds, alpha=0.6)
    plt.plot([-500, 500], [-500, 500], 'r--', label='Perfect correlation')
    plt.xlabel('Engine Evaluation (centipawns)')
    plt.ylabel('ML Prediction (centipawns)')
    plt.title('ML Model vs Chess Engine Evaluation')
    plt.legend()
    plt.grid(True, alpha=0.3)
    plt.show()

if __name__ == "__main__":
    ml_chess_analysis()
```

---

## 🎮 Game Development Integration

### Chess Game Framework

```python
import chess_engine_rust as chess
import tkinter as tk
from tkinter import messagebox
import threading
import time

class ChessGameGUI:
    """Complete chess game with GUI using tkinter and chess engine."""

    def __init__(self):
        self.root = tk.Tk()
        self.root.title("Chess Game - Powered by Rust Engine")
        self.root.geometry("800x600")

        # Initialize engine
        config = chess.EngineConfig(
            depth=8,
            time_limit=timedelta(seconds=3),
            threads=2
        )
        self.engine = chess.ChessEngine(config)

        # Game state
        self.selected_square = None
        self.player_color = "white"
        self.game_mode = "vs_engine"  # vs_engine, vs_human, analysis

        self.setup_ui()
        self.update_board()

    def setup_ui(self):
        """Set up the user interface."""

        # Main frame
        main_frame = tk.Frame(self.root)
        main_frame.pack(fill=tk.BOTH, expand=True, padding=10)

        # Board frame (left side)
        board_frame = tk.Frame(main_frame)
        board_frame.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

        # Create chess board (8x8 grid)
        self.board_buttons = []
        for row in range(8):
            button_row = []
            for col in range(8):
                color = "white" if (row + col) % 2 == 0 else "gray"

                btn = tk.Button(
                    board_frame,
                    width=8, height=4,
                    bg=color,
                    font=("Arial", 16),
                    command=lambda r=row, c=col: self.on_square_click(r, c)
                )
                btn.grid(row=row, column=col, padx=1, pady=1)
                button_row.append(btn)
            self.board_buttons.append(button_row)

        # Info frame (right side)
        info_frame = tk.Frame(main_frame)
        info_frame.pack(side=tk.RIGHT, fill=tk.BOTH, padx=(10, 0))

        # Game info
        self.info_text = tk.Text(info_frame, width=30, height=20)
        self.info_text.pack(fill=tk.BOTH, expand=True)

        # Control buttons
        controls_frame = tk.Frame(info_frame)
        controls_frame.pack(fill=tk.X, pady=(10, 0))

        tk.Button(controls_frame, text="New Game", command=self.new_game).pack(fill=tk.X)
        tk.Button(controls_frame, text="Undo Move", command=self.undo_move).pack(fill=tk.X)
        tk.Button(controls_frame, text="Engine Move", command=self.engine_move).pack(fill=tk.X)
        tk.Button(controls_frame, text="Analyze", command=self.analyze_position).pack(fill=tk.X)

        # Status bar
        self.status_var = tk.StringVar()
        self.status_var.set("Ready to play")
        status_bar = tk.Label(self.root, textvariable=self.status_var, relief=tk.SUNKEN)
        status_bar.pack(side=tk.BOTTOM, fill=tk.X)

    def update_board(self):
        """Update the visual board representation."""

        # Get current position
        position_dict = self.engine.get_position_dict()

        # Unicode chess pieces
        piece_symbols = {
            'white': {'pawn': '♙', 'rook': '♖', 'knight': '♘', 'bishop': '♗', 'queen': '♕', 'king': '♔'},
            'black': {'pawn': '♟', 'rook': '♜', 'knight': '♞', 'bishop': '♝', 'queen': '♛', 'king': '♚'}
        }

        for row in range(8):
            for col in range(8):
                square_name = chr(ord('a') + col) + str(8 - row)  # Convert to chess notation

                # Reset button appearance
                base_color = "white" if (row + col) % 2 == 0 else "lightgray"

                # Highlight selected square
                if self.selected_square == (row, col):
                    base_color = "yellow"

                # Get piece on square
                piece_info = position_dict.get(square_name)
                if piece_info:
                    symbol = piece_symbols[piece_info['color']][piece_info['piece_type']]
                else:
                    symbol = ""

                # Update button
                self.board_buttons[row][col].config(
                    text=symbol,
                    bg=base_color
                )

        # Update info panel
        self.update_info_panel()

    def update_info_panel(self):
        """Update the information panel."""

        info = []
        info.append(f"🎯 Game Status")
        info.append(f"Turn: {self.engine.get_side_to_move()}")
        info.append(f"Evaluation: {self.engine.get_evaluation()} cp")
        info.append(f"Legal moves: {len(self.engine.get_legal_moves())}")
        info.append("")

        # Game state
        if self.engine.is_in_check():
            info.append("⚠️ IN CHECK!")
        if self.engine.is_checkmate():
            info.append("🏁 CHECKMATE!")
        if self.engine.is_stalemate():
            info.append("🤝 STALEMATE!")
        info.append("")

        # Position info
        info.append(f"📋 Position (FEN):")
        info.append(self.engine.get_fen())
        info.append("")

        # Recent moves (if available)
        move_history = self.engine.get_move_history()
        if move_history:
            info.append("📝 Move History:")
            for i, move in enumerate(move_history[-5:], 1):  # Last 5 moves
                info.append(f"{len(move_history)-5+i}. {move}")

        # Update text widget
        self.info_text.delete(1.0, tk.END)
        self.info_text.insert(tk.END, "\n".join(info))

    def on_square_click(self, row, col):
        """Handle square click events."""

        square_name = chr(ord('a') + col) + str(8 - row)

        if self.selected_square is None:
            # Select piece
            if self.engine.has_piece_at(square_name):
                self.selected_square = (row, col)
                self.status_var.set(f"Selected {square_name}")
        else:
            # Try to make move
            from_row, from_col = self.selected_square
            from_square = chr(ord('a') + from_col) + str(8 - from_row)
            to_square = square_name

            move_str = from_square + to_square

            try:
                self.engine.make_move(move_str)
                self.status_var.set(f"Move made: {move_str}")

                # Engine response (if vs engine mode)
                if self.game_mode == "vs_engine":
                    self.root.after(500, self.engine_move)  # Delay for visual effect

            except Exception as e:
                self.status_var.set(f"Invalid move: {e}")

            self.selected_square = None

        self.update_board()

    def engine_move(self):
        """Make engine move in separate thread."""

        def make_move():
            self.status_var.set("Engine is thinking...")

            try:
                best_move = self.engine.find_best_move()
                if best_move:
                    self.engine.make_move(best_move)
                    evaluation = self.engine.get_evaluation()

                    self.root.after(0, lambda: self.status_var.set(
                        f"Engine played: {best_move} ({evaluation} cp)"
                    ))
                    self.root.after(0, self.update_board)
                else:
                    self.root.after(0, lambda: self.status_var.set("No legal moves available"))

            except Exception as e:
                self.root.after(0, lambda: self.status_var.set(f"Engine error: {e}"))

        # Run in background thread
        thread = threading.Thread(target=make_move)
        thread.daemon = True
        thread.start()

    def new_game(self):
        """Start a new game."""
        self.engine.reset_to_starting_position()
        self.selected_square = None
        self.status_var.set("New game started")
        self.update_board()

    def undo_move(self):
        """Undo the last move."""
        try:
            self.engine.undo_last_move()
            self.status_var.set("Move undone")
            self.update_board()
        except Exception as e:
            self.status_var.set(f"Cannot undo: {e}")

    def analyze_position(self):
        """Analyze current position."""
        def analyze():
            self.status_var.set("Analyzing position...")

            try:
                analysis = self.engine.analyze_position(depth=10)

                result = []
                result.append("🔍 Position Analysis:")
                result.append(f"Best move: {analysis.best_move}")
                result.append(f"Evaluation: {analysis.evaluation} cp")
                result.append(f"Depth: {analysis.depth}")
                result.append(f"Nodes: {analysis.nodes_searched:,}")
                result.append(f"Time: {analysis.time_taken:.2f}s")
                result.append("")
                result.append("Principal Variation:")
                result.append(" ".join(analysis.principal_variation[:5]))

                messagebox.showinfo("Position Analysis", "\n".join(result))
                self.status_var.set("Analysis complete")

            except Exception as e:
                self.status_var.set(f"Analysis error: {e}")

        # Run analysis in background
        thread = threading.Thread(target=analyze)
        thread.daemon = True
        thread.start()

    def run(self):
        """Start the GUI application."""
        self.root.mainloop()

# Usage
if __name__ == "__main__":
    game = ChessGameGUI()
    game.run()
```

---

## 📚 Advanced Features

### Tournament Management

```python
import chess_engine_rust as chess
from datetime import datetime, timedelta
import json

class ChessTournament:
    """Manage chess tournaments with multiple engines."""

    def __init__(self):
        self.engines = {}
        self.games = []
        self.results = {}

    def add_engine(self, name, config):
        """Add an engine to the tournament."""
        self.engines[name] = chess.ChessEngine(config)
        self.results[name] = {'wins': 0, 'draws': 0, 'losses': 0, 'points': 0}

    def play_round_robin(self, time_control=timedelta(minutes=5)):
        """Play round-robin tournament between all engines."""

        engine_names = list(self.engines.keys())
        total_games = len(engine_names) * (len(engine_names) - 1)

        print(f"🏆 Starting round-robin tournament with {len(engine_names)} engines")
        print(f"   Total games: {total_games}")
        print(f"   Time control: {time_control.total_seconds()}s per move")

        game_num = 1
        for i, white_name in enumerate(engine_names):
            for j, black_name in enumerate(engine_names):
                if i != j:  # Don't play against self
                    print(f"\n🎮 Game {game_num}/{total_games}: {white_name} vs {black_name}")

                    result = self.play_game(
                        white_engine=self.engines[white_name],
                        black_engine=self.engines[black_name],
                        white_name=white_name,
                        black_name=black_name,
                        time_control=time_control
                    )

                    self.record_result(result)
                    game_num += 1

        self.print_tournament_results()

    def play_game(self, white_engine, black_engine, white_name, black_name, time_control):
        """Play a single game between two engines."""

        # Reset both engines
        white_engine.reset_to_starting_position()
        black_engine.reset_to_starting_position()

        game_record = {
            'white': white_name,
            'black': black_name,
            'moves': [],
            'result': None,
            'termination': None,
            'start_time': datetime.now()
        }

        move_count = 0
        max_moves = 200  # Prevent infinite games

        while not white_engine.is_game_over() and move_count < max_moves:
            current_engine = white_engine if move_count % 2 == 0 else black_engine
            other_engine = black_engine if move_count % 2 == 0 else white_engine
            color = "White" if move_count % 2 == 0 else "Black"

            # Find best move with time control
            start_time = datetime.now()
            best_move = current_engine.find_best_move_with_timeout(time_control)
            think_time = datetime.now() - start_time

            if not best_move:
                # No legal moves
                if current_engine.is_in_check():
                    game_record['result'] = "0-1" if color == "White" else "1-0"
                    game_record['termination'] = "checkmate"
                else:
                    game_record['result'] = "1/2-1/2"
                    game_record['termination'] = "stalemate"
                break

            # Make move on both engines
            current_engine.make_move(best_move)
            other_engine.make_move(best_move)

            game_record['moves'].append({
                'move': best_move,
                'color': color,
                'time': think_time.total_seconds(),
                'evaluation': current_engine.get_evaluation()
            })

            move_count += 1

            # Check for draws
            if current_engine.is_fifty_move_rule_draw():
                game_record['result'] = "1/2-1/2"
                game_record['termination'] = "fifty_move_rule"
                break
            elif current_engine.is_threefold_repetition():
                game_record['result'] = "1/2-1/2"
                game_record['termination'] = "threefold_repetition"
                break
            elif current_engine.is_insufficient_material():
                game_record['result'] = "1/2-1/2"
                game_record['termination'] = "insufficient_material"
                break

        if not game_record['result']:
            # Game didn't finish naturally
            game_record['result'] = "1/2-1/2"
            game_record['termination'] = "max_moves_reached"

        game_record['end_time'] = datetime.now()
        game_record['duration'] = (game_record['end_time'] - game_record['start_time']).total_seconds()

        self.games.append(game_record)

        print(f"   Result: {game_record['result']} ({game_record['termination']})")
        print(f"   Moves: {move_count}, Duration: {game_record['duration']:.1f}s")

        return game_record

    def record_result(self, game_record):
        """Record game result in tournament standings."""

        white_name = game_record['white']
        black_name = game_record['black']
        result = game_record['result']

        if result == "1-0":  # White wins
            self.results[white_name]['wins'] += 1
            self.results[white_name]['points'] += 1
            self.results[black_name]['losses'] += 1
        elif result == "0-1":  # Black wins
            self.results[black_name]['wins'] += 1
            self.results[black_name]['points'] += 1
            self.results[white_name]['losses'] += 1
        else:  # Draw
            self.results[white_name]['draws'] += 1
            self.results[white_name]['points'] += 0.5
            self.results[black_name]['draws'] += 1
            self.results[black_name]['points'] += 0.5

    def print_tournament_results(self):
        """Print final tournament standings."""

        print("\n" + "="*60)
        print("🏆 TOURNAMENT RESULTS")
        print("="*60)

        # Sort by points (descending)
        sorted_results = sorted(
            self.results.items(),
            key=lambda x: x[1]['points'],
            reverse=True
        )

        print(f"{'Rank':<4} {'Engine':<20} {'Points':<8} {'W':<4} {'D':<4} {'L':<4} {'Score%':<8}")
        print("-" * 60)

        for rank, (engine_name, stats) in enumerate(sorted_results, 1):
            total_games = stats['wins'] + stats['draws'] + stats['losses']
            score_percentage = (stats['points'] / total_games * 100) if total_games > 0 else 0

            print(f"{rank:<4} {engine_name:<20} {stats['points']:<8.1f} "
                  f"{stats['wins']:<4} {stats['draws']:<4} {stats['losses']:<4} {score_percentage:<8.1f}")

        # Save results to file
        with open('tournament_results.json', 'w') as f:
            json.dump({
                'results': self.results,
                'games': self.games,
                'timestamp': datetime.now().isoformat()
            }, f, indent=2, default=str)

        print(f"\n📊 Detailed results saved to 'tournament_results.json'")

# Usage example
def run_engine_tournament():
    """Run a tournament between different engine configurations."""

    tournament = ChessTournament()

    # Add different engine configurations
    tournament.add_engine("Aggressive", chess.EngineConfig(
        depth=6, time_limit=timedelta(seconds=3),
        use_aggressive_pruning=True
    ))

    tournament.add_engine("Defensive", chess.EngineConfig(
        depth=8, time_limit=timedelta(seconds=5),
        prioritize_king_safety=True
    ))

    tournament.add_engine("Tactical", chess.EngineConfig(
        depth=7, time_limit=timedelta(seconds=4),
        enhance_tactical_search=True
    ))

    tournament.add_engine("Positional", chess.EngineConfig(
        depth=6, time_limit=timedelta(seconds=3),
        emphasize_positional_play=True
    ))

    # Run tournament
    tournament.play_round_robin(time_control=timedelta(seconds=5))

if __name__ == "__main__":
    run_engine_tournament()
```

---

## 🚀 Performance Optimization

### Profiling and Benchmarking

```python
import chess_engine_rust as chess
import cProfile
import pstats
import time
from concurrent.futures import ThreadPoolExecutor
import multiprocessing as mp

def profile_engine_performance():
    """Profile engine performance to identify bottlenecks."""

    engine = chess.ChessEngine()

    def benchmark_function():
        """Function to benchmark."""
        for _ in range(100):
            # Generate random position
            engine.reset_to_starting_position()

            # Make some random moves
            for _ in range(10):
                legal_moves = engine.get_legal_moves()
                if legal_moves:
                    move = legal_moves[0]  # Pick first legal move
                    engine.make_move(move)

            # Find best move
            best_move = engine.find_best_move()

            # Evaluate position
            evaluation = engine.get_evaluation()

    # Profile the function
    profiler = cProfile.Profile()
    profiler.enable()

    start_time = time.time()
    benchmark_function()
    end_time = time.time()

    profiler.disable()

    # Analyze results
    stats = pstats.Stats(profiler)
    stats.sort_stats('cumulative')

    print(f"⚡ Performance Benchmark Results:")
    print(f"   Total time: {end_time - start_time:.2f} seconds")
    print(f"   Operations per second: {100 / (end_time - start_time):.1f}")
    print("\n🔍 Top function calls by time:")
    stats.print_stats(10)

def benchmark_parallel_analysis(num_positions=1000):
    """Benchmark parallel position analysis."""

    print(f"🔄 Benchmarking parallel analysis of {num_positions} positions...")

    # Generate test positions
    engine = chess.ChessEngine()
    test_positions = []

    for _ in range(num_positions):
        engine.reset_to_starting_position()
        # Make a few random moves to get diverse positions
        for _ in range(5):
            legal_moves = engine.get_legal_moves()
            if legal_moves:
                engine.make_move(legal_moves[0])
        test_positions.append(engine.get_fen())

    def analyze_position(fen):
        """Analyze a single position."""
        local_engine = chess.ChessEngine()
        local_engine.set_position(fen)
        return local_engine.get_evaluation()

    # Sequential benchmark
    start_time = time.time()
    sequential_results = []
    for fen in test_positions:
        sequential_results.append(analyze_position(fen))
    sequential_time = time.time() - start_time

    # Parallel benchmark
    start_time = time.time()
    with ThreadPoolExecutor(max_workers=mp.cpu_count()) as executor:
        parallel_results = list(executor.map(analyze_position, test_positions))
    parallel_time = time.time() - start_time

    # Results
    speedup = sequential_time / parallel_time

    print(f"📊 Parallel Analysis Results:")
    print(f"   Positions analyzed: {num_positions}")
    print(f"   Sequential time: {sequential_time:.2f}s ({num_positions/sequential_time:.1f} pos/s)")
    print(f"   Parallel time: {parallel_time:.2f}s ({num_positions/parallel_time:.1f} pos/s)")
    print(f"   Speedup: {speedup:.2f}x")
    print(f"   CPU cores used: {mp.cpu_count()}")

    # Verify results are identical
    if sequential_results == parallel_results:
        print("✅ Results verified: Sequential and parallel analysis match")
    else:
        print("⚠️ Warning: Sequential and parallel results differ")

if __name__ == "__main__":
    profile_engine_performance()
    print("\n" + "="*50 + "\n")
    benchmark_parallel_analysis()
```

---

## 📦 Package Distribution

### Creating Installable Packages

```python
# setup.py for custom chess application
from setuptools import setup, find_packages

setup(
    name="my-chess-app",
    version="1.0.0",
    author="Your Name",
    author_email="your.email@example.com",
    description="Advanced chess application powered by Rust engine",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/yourusername/my-chess-app",
    packages=find_packages(),

    install_requires=[
        "chess-engine-rust>=0.1.0",
        "numpy>=1.20.0",
        "matplotlib>=3.3.0",
        "pandas>=1.3.0",
        "tkinter-tooltip>=2.1.0",
    ],

    extras_require={
        "ml": ["scikit-learn>=1.0.0", "tensorflow>=2.8.0"],
        "analysis": ["plotly>=5.0.0", "jupyter>=1.0.0"],
        "dev": ["pytest>=6.0.0", "black>=22.0.0", "mypy>=0.910"],
    },

    entry_points={
        "console_scripts": [
            "my-chess-gui=my_chess_app.gui:main",
            "my-chess-analysis=my_chess_app.analysis:main",
            "my-chess-tournament=my_chess_app.tournament:main",
        ],
    },

    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Topic :: Games/Entertainment :: Board Games",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
    ],

    python_requires=">=3.8",
)
```

---

## 📚 Resources and Next Steps

### Documentation and Learning

- 📖 **Chess Engine API**: [Full Python API documentation](https://chess-engine-rust.readthedocs.io/python/)
- 🧠 **Chess Programming**: [Chess Programming Wiki](https://www.chessprogramming.org/)
- 🐍 **Python-Rust Integration**: [PyO3 Guide](https://pyo3.rs/)
- 📊 **Machine Learning in Chess**: [Chess AI Research Papers](https://github.com/topics/chess-ai)

### Community and Support

- 💬 **Discussions**: [GitHub Discussions](https://github.com/username/chess-engine-rust/discussions)
- 🐛 **Issues**: [Report Bugs](https://github.com/username/chess-engine-rust/issues)
- 📧 **Email**: support@chess-engine-rust.com
- 🔗 **Discord**: [Chess Engine Community](https://discord.gg/chess-engine-rust)

---

*Ready to build amazing chess applications with Python? The engine is at your service! 🐍♟️*