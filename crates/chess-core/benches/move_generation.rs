use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use chess_core::{
    GameState, MoveGenerator, Square, PieceType, Color, Move
};

// Test positions from different game phases
const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const MIDDLE_GAME_1: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const MIDDLE_GAME_2: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
const MIDDLE_GAME_COMPLEX: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
const ENDGAME_KQK: &str = "8/8/8/8/8/8/4K3/k2Q4 w - - 0 1";
const ENDGAME_ROOK: &str = "8/8/8/8/8/8/R7/K6k w - - 0 1";
const ENDGAME_PAWN: &str = "8/8/8/8/8/8/P7/K6k w - - 0 1";
const ENDGAME_BISHOPS: &str = "8/8/8/8/8/2B5/1B6/K6k w - - 0 1";
const TACTICAL_POSITION: &str = "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 4";
const PROMOTION_POSITION: &str = "8/P1P5/8/8/8/8/p1p5/8 w - - 0 1";

// Fast move generator (current implementation)
#[allow(dead_code)]
struct FastMoveGenerator {
    generator: MoveGenerator,
}

impl FastMoveGenerator {
    fn new() -> Self {
        Self {
            generator: MoveGenerator::new(),
        }
    }

    fn generate_moves(&self, game_state: &GameState) -> Vec<Move> {
        game_state.generate_legal_moves()
    }

    fn generate_pseudo_legal(&self, game_state: &GameState) -> Vec<Move> {
        game_state.generate_pseudo_legal_moves()
    }
}

// Slow move generator (brute force for comparison)
struct SlowMoveGenerator;

impl SlowMoveGenerator {
    fn new() -> Self {
        Self
    }

    fn generate_moves(&self, game_state: &GameState) -> Vec<Move> {
        // Brute force approach: try all possible moves and filter legal ones
        let mut moves = Vec::new();
        let side_to_move = game_state.position.side_to_move;

        // Iterate through all squares
        for from_square in 0..64 {
            let from = Square::new(from_square).unwrap();
            if let Some(piece) = game_state.position.piece_at(from) {
                if piece.color == side_to_move {
                    // Try all possible destination squares
                    for to_square in 0..64 {
                        let to = Square::new(to_square).unwrap();

                        // Generate different move types
                        let normal_move = Move::normal(from, to);
                        let capture_move = Move::capture(from, to);

                        // Test if moves are legal
                        if self.is_move_legal(game_state, normal_move) {
                            moves.push(normal_move);
                        } else if self.is_move_legal(game_state, capture_move) {
                            moves.push(capture_move);
                        }

                        // Test promotions for pawns on 7th/2nd rank
                        if piece.piece_type == PieceType::Pawn {
                            let promotion_rank = match piece.color {
                                Color::White => 7,
                                Color::Black => 0,
                            };

                            if to.rank() == promotion_rank {
                                for piece_type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                                    let promo_move = Move::promotion(from, to, piece_type);
                                    let promo_capture = Move::promotion_capture(from, to, piece_type);

                                    if self.is_move_legal(game_state, promo_move) {
                                        moves.push(promo_move);
                                    } else if self.is_move_legal(game_state, promo_capture) {
                                        moves.push(promo_capture);
                                    }
                                }
                            }
                        }
                    }

                    // Test castling for kings
                    if piece.piece_type == PieceType::King {
                        let castle_moves = match piece.color {
                            Color::White => [
                                Move::castle(Square::E1, Square::G1),
                                Move::castle(Square::E1, Square::C1),
                            ],
                            Color::Black => [
                                Move::castle(Square::E8, Square::G8),
                                Move::castle(Square::E8, Square::C8),
                            ],
                        };

                        for castle_move in castle_moves {
                            if self.is_move_legal(game_state, castle_move) {
                                moves.push(castle_move);
                            }
                        }
                    }

                    // Test en passant for pawns
                    if piece.piece_type == PieceType::Pawn {
                        if let Some(ep_target) = game_state.en_passant_target {
                            let ep_move = Move::en_passant(from, ep_target);
                            if self.is_move_legal(game_state, ep_move) {
                                moves.push(ep_move);
                            }
                        }
                    }
                }
            }
        }

        moves
    }

    fn is_move_legal(&self, game_state: &GameState, mv: Move) -> bool {
        // Test if a move is legal by trying to make it
        let mut test_state = game_state.clone();
        test_state.position.make_move(mv).is_ok() &&
        !test_state.is_in_check(game_state.position.side_to_move)
    }
}

fn benchmark_starting_position(c: &mut Criterion) {
    let mut group = c.benchmark_group("starting_position");

    let game_state = GameState::from_fen(STARTING_POSITION).unwrap();
    let fast_gen = FastMoveGenerator::new();
    let slow_gen = SlowMoveGenerator::new();

    group.bench_function("fast_generator", |b| {
        b.iter(|| {
            let moves = black_box(fast_gen.generate_moves(&game_state));
            black_box(moves.len())
        })
    });

    group.bench_function("slow_generator", |b| {
        b.iter(|| {
            let moves = black_box(slow_gen.generate_moves(&game_state));
            black_box(moves.len())
        })
    });

    group.bench_function("fast_pseudo_legal", |b| {
        b.iter(|| {
            let moves = black_box(fast_gen.generate_pseudo_legal(&game_state));
            black_box(moves.len())
        })
    });

    group.finish();
}

fn benchmark_middle_game(c: &mut Criterion) {
    let mut group = c.benchmark_group("middle_game");

    let positions = [
        ("complex", MIDDLE_GAME_COMPLEX),
        ("tactical", MIDDLE_GAME_1),
        ("attacking", MIDDLE_GAME_2),
    ];

    let fast_gen = FastMoveGenerator::new();
    let slow_gen = SlowMoveGenerator::new();

    for (name, fen) in positions.iter() {
        let game_state = GameState::from_fen(fen).unwrap();

        group.bench_with_input(BenchmarkId::new("fast", name), &game_state, |b, gs| {
            b.iter(|| {
                let moves = black_box(fast_gen.generate_moves(gs));
                black_box(moves.len())
            })
        });

        group.bench_with_input(BenchmarkId::new("slow", name), &game_state, |b, gs| {
            b.iter(|| {
                let moves = black_box(slow_gen.generate_moves(gs));
                black_box(moves.len())
            })
        });
    }

    group.finish();
}

fn benchmark_endgame(c: &mut Criterion) {
    let mut group = c.benchmark_group("endgame");

    let positions = [
        ("kqk", ENDGAME_KQK),
        ("rook", ENDGAME_ROOK),
        ("pawn", ENDGAME_PAWN),
        ("bishops", ENDGAME_BISHOPS),
    ];

    let fast_gen = FastMoveGenerator::new();
    let slow_gen = SlowMoveGenerator::new();

    for (name, fen) in positions.iter() {
        let game_state = GameState::from_fen(fen).unwrap();

        group.bench_with_input(BenchmarkId::new("fast", name), &game_state, |b, gs| {
            b.iter(|| {
                let moves = black_box(fast_gen.generate_moves(gs));
                black_box(moves.len())
            })
        });

        group.bench_with_input(BenchmarkId::new("slow", name), &game_state, |b, gs| {
            b.iter(|| {
                let moves = black_box(slow_gen.generate_moves(gs));
                black_box(moves.len())
            })
        });
    }

    group.finish();
}

fn benchmark_piece_specific(c: &mut Criterion) {
    let mut group = c.benchmark_group("piece_specific");

    // Create positions that emphasize specific pieces
    let positions = [
        ("pawn_heavy", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        ("knight_heavy", "n1n1k1n1/8/8/8/8/8/8/N1N1K1N1 w - - 0 1"),
        ("bishop_heavy", "b1b1k1b1/8/8/8/8/8/8/B1B1K1B1 w - - 0 1"),
        ("rook_heavy", "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1"),
        ("queen_heavy", "q3k2q/8/8/8/8/8/8/Q3K2Q w - - 0 1"),
        ("mixed_pieces", "rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1"),
    ];

    let fast_gen = FastMoveGenerator::new();

    for (name, fen) in positions.iter() {
        let game_state = GameState::from_fen(fen).unwrap();

        group.bench_with_input(BenchmarkId::new("moves", name), &game_state, |b, gs| {
            b.iter(|| {
                let moves = black_box(fast_gen.generate_moves(gs));
                black_box(moves.len())
            })
        });
    }

    group.finish();
}

fn benchmark_special_moves(c: &mut Criterion) {
    let mut group = c.benchmark_group("special_moves");

    let positions = [
        ("castling", "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1"),
        ("en_passant", "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3"),
        ("promotion", PROMOTION_POSITION),
        ("tactical", TACTICAL_POSITION),
    ];

    let fast_gen = FastMoveGenerator::new();
    let slow_gen = SlowMoveGenerator::new();

    for (name, fen) in positions.iter() {
        let game_state = GameState::from_fen(fen).unwrap();

        group.bench_with_input(BenchmarkId::new("fast", name), &game_state, |b, gs| {
            b.iter(|| {
                let moves = black_box(fast_gen.generate_moves(gs));
                black_box(moves.len())
            })
        });

        group.bench_with_input(BenchmarkId::new("slow", name), &game_state, |b, gs| {
            b.iter(|| {
                let moves = black_box(slow_gen.generate_moves(gs));
                black_box(moves.len())
            })
        });
    }

    group.finish();
}

fn benchmark_move_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_validation");

    let game_state = GameState::from_fen(MIDDLE_GAME_COMPLEX).unwrap();
    let fast_gen = FastMoveGenerator::new();

    // Generate some moves to validate
    let legal_moves = fast_gen.generate_moves(&game_state);
    let pseudo_moves = fast_gen.generate_pseudo_legal(&game_state);

    group.bench_function("legal_move_check", |b| {
        b.iter(|| {
            for mv in legal_moves.iter().take(10) {
                black_box(game_state.is_legal_move(*mv));
            }
        })
    });

    group.bench_function("pseudo_legal_generation", |b| {
        b.iter(|| {
            let moves = black_box(fast_gen.generate_pseudo_legal(&game_state));
            black_box(moves.len())
        })
    });

    group.bench_function("legal_from_pseudo", |b| {
        b.iter(|| {
            let mut legal_count = 0;
            for mv in pseudo_moves.iter().take(20) {
                if game_state.is_legal_move(*mv) {
                    legal_count += 1;
                }
            }
            black_box(legal_count)
        })
    });

    group.finish();
}

fn benchmark_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_operations");

    let positions = [
        STARTING_POSITION,
        MIDDLE_GAME_1,
        MIDDLE_GAME_2,
        ENDGAME_KQK,
        TACTICAL_POSITION,
    ];

    let fast_gen = FastMoveGenerator::new();

    group.bench_function("bulk_move_generation", |b| {
        b.iter(|| {
            let mut total_moves = 0;
            for fen in positions.iter() {
                let game_state = GameState::from_fen(fen).unwrap();
                let moves = fast_gen.generate_moves(&game_state);
                total_moves += moves.len();
            }
            black_box(total_moves)
        })
    });

    group.bench_function("repeated_same_position", |b| {
        let game_state = GameState::from_fen(MIDDLE_GAME_COMPLEX).unwrap();
        b.iter(|| {
            for _ in 0..100 {
                let moves = black_box(fast_gen.generate_moves(&game_state));
                black_box(moves.len());
            }
        })
    });

    group.finish();
}

fn benchmark_memory_pressure(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pressure");

    let game_state = GameState::from_fen(MIDDLE_GAME_COMPLEX).unwrap();
    let fast_gen = FastMoveGenerator::new();

    group.bench_function("many_generators", |b| {
        b.iter(|| {
            let generators: Vec<FastMoveGenerator> = (0..100)
                .map(|_| FastMoveGenerator::new())
                .collect();

            let mut total_moves = 0;
            for gen in generators.iter() {
                let moves = gen.generate_moves(&game_state);
                total_moves += moves.len();
            }
            black_box(total_moves)
        })
    });

    group.bench_function("large_move_vectors", |b| {
        b.iter(|| {
            let mut all_moves = Vec::new();
            for _ in 0..1000 {
                let moves = fast_gen.generate_moves(&game_state);
                all_moves.extend(moves);
            }
            black_box(all_moves.len())
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_starting_position,
    benchmark_middle_game,
    benchmark_endgame,
    benchmark_piece_specific,
    benchmark_special_moves,
    benchmark_move_validation,
    benchmark_bulk_operations,
    benchmark_memory_pressure
);
criterion_main!(benches);