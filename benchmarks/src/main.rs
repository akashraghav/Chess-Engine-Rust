use chess_engine::{ChessEngine, ChessEngineBuilder};
use chess_core::{GameState, MoveGenerator, Position, Evaluator};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
// use std::time::Duration;

fn benchmark_move_generation(c: &mut Criterion) {
    let game_state = GameState::new();
    let _move_generator = MoveGenerator::new();

    c.bench_function("move_generation_starting_position", |b| {
        b.iter(|| {
            let moves = black_box(game_state.generate_legal_moves());
            black_box(moves.len())
        })
    });

    let middle_game_fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    let middle_game = GameState::from_fen(middle_game_fen).unwrap();

    c.bench_function("move_generation_middle_game", |b| {
        b.iter(|| {
            let moves = black_box(middle_game.generate_legal_moves());
            black_box(moves.len())
        })
    });
}

fn benchmark_position_evaluation(c: &mut Criterion) {
    let evaluator = Evaluator::new();
    let starting_position = GameState::new();

    c.bench_function("evaluation_starting_position", |b| {
        b.iter(|| {
            let score = black_box(evaluator.evaluate(&starting_position));
            black_box(score)
        })
    });

    let complex_fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    let complex_position = GameState::from_fen(complex_fen).unwrap();

    c.bench_function("evaluation_complex_position", |b| {
        b.iter(|| {
            let score = black_box(evaluator.evaluate(&complex_position));
            black_box(score)
        })
    });
}

fn benchmark_engine_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_operations");

    group.bench_function("engine_creation", |b| {
        b.iter(|| {
            let engine = black_box(ChessEngine::new());
            black_box(engine)
        })
    });

    let mut engine = ChessEngine::new();
    engine.initialize().unwrap();

    group.bench_function("make_move", |b| {
        b.iter(|| {
            let mut test_engine = ChessEngine::new();
            test_engine.initialize().unwrap();
            let result = black_box(test_engine.make_move_from_uci("e2e4"));
            black_box(result)
        })
    });

    group.bench_function("legal_moves", |b| {
        b.iter(|| {
            let moves = black_box(engine.get_legal_moves());
            black_box(moves.len())
        })
    });

    group.bench_function("find_best_move", |b| {
        b.iter(|| {
            let mut test_engine = ChessEngine::new();
            test_engine.initialize().unwrap();
            let best_move = black_box(test_engine.find_best_move());
            black_box(best_move)
        })
    });

    group.finish();
}

fn benchmark_fen_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("fen_operations");

    let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    group.bench_function("fen_parsing", |b| {
        b.iter(|| {
            let game_state = black_box(GameState::from_fen(starting_fen));
            black_box(game_state)
        })
    });

    let position = Position::starting_position();

    group.bench_function("fen_generation", |b| {
        b.iter(|| {
            let fen = black_box(position.to_fen());
            black_box(fen)
        })
    });

    group.finish();
}

fn benchmark_different_depths(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_depth");

    for depth in [1, 2, 3, 4, 5].iter() {
        group.bench_with_input(BenchmarkId::new("depth", depth), depth, |b, &depth| {
            b.iter(|| {
                let mut engine = ChessEngineBuilder::new()
                    .with_depth(depth)
                    .build()
                    .unwrap();

                let best_move = black_box(engine.find_best_move());
                black_box(best_move)
            })
        });
    }

    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    c.bench_function("memory_allocation", |b| {
        b.iter(|| {
            let engines: Vec<ChessEngine> = (0..100)
                .map(|_| black_box(ChessEngine::new()))
                .collect();
            black_box(engines)
        })
    });
}

criterion_group!(
    benches,
    benchmark_move_generation,
    benchmark_position_evaluation,
    benchmark_engine_operations,
    benchmark_fen_operations,
    benchmark_different_depths,
    benchmark_memory_usage
);
criterion_main!(benches);