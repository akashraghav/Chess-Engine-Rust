// Phase 2 optimization benchmarks
// Comprehensive performance testing for SIMD, search, parallel, and memory optimizations

use chess_core::search::{ParallelEvaluator, ParallelMoveGenerator};
use chess_core::utils::memory::NodeType;
use chess_core::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// Test positions for benchmarking
const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE_POSITION: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const MIDDLE_GAME_POSITION: &str =
    "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4";
const ENDGAME_POSITION: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";

fn create_test_position(fen: &str) -> Position {
    Position::from_fen(fen).unwrap_or_else(|_| {
        // Fallback to starting position if FEN parsing fails
        Position::default()
    })
}

// Helper: convert a Position to a minimal GameState for the basic Evaluator
fn game_state_from_position(position: &Position) -> GameState {
    let mut gs = GameState::new();
    gs.position = position.clone();
    gs
}

fn benchmark_simd_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_operations");

    // Test data for SIMD benchmarks
    let test_bitboards = [
        Bitboard::new(0xFF00000000000000), // 8th rank
        Bitboard::new(0x00FF000000000000), // 7th rank
        Bitboard::new(0x0000FF0000000000), // 6th rank
        Bitboard::new(0x000000FF00000000), // 5th rank
    ];

    group.bench_function("simd_batch_and", |b| {
        b.iter(|| OptimizedBitboard::batch_and_4(&test_bitboards, &test_bitboards))
    });

    group.bench_function("simd_batch_or", |b| {
        b.iter(|| OptimizedBitboard::batch_or_4(&test_bitboards, &test_bitboards))
    });

    group.bench_function("simd_batch_popcount", |b| {
        b.iter(|| OptimizedBitboard::batch_popcount_4(&test_bitboards))
    });

    group.bench_function("simd_batch_shifts", |b| {
        b.iter(|| OptimizedBitboard::batch_north_shifts(&test_bitboards))
    });

    // Compare with sequential operations
    group.bench_function("sequential_and", |b| {
        b.iter(|| {
            [
                test_bitboards[0] & test_bitboards[1],
                test_bitboards[1] & test_bitboards[2],
                test_bitboards[2] & test_bitboards[3],
                test_bitboards[3] & test_bitboards[0],
            ]
        })
    });

    group.bench_function("sequential_popcount", |b| {
        b.iter(|| {
            [
                test_bitboards[0].count_bits(),
                test_bitboards[1].count_bits(),
                test_bitboards[2].count_bits(),
                test_bitboards[3].count_bits(),
            ]
        })
    });

    group.finish();
}

fn benchmark_advanced_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("advanced_search");

    let positions = [
        ("starting", create_test_position(STARTING_POSITION)),
        ("kiwipete", create_test_position(KIWIPETE_POSITION)),
        ("middlegame", create_test_position(MIDDLE_GAME_POSITION)),
        ("endgame", create_test_position(ENDGAME_POSITION)),
    ];

    for (name, position) in &positions {
        // Basic alpha-beta search
        group.bench_with_input(BenchmarkId::new("alpha_beta", name), position, |b, pos| {
            let config = SearchConfig {
                max_depth: 4,
                max_time: None,
                max_nodes: Some(10000),
                ..SearchConfig::default()
            };
            let mut engine = SearchEngine::new(config);

            b.iter(|| engine.search(pos))
        });

        // Search with null move pruning disabled
        group.bench_with_input(
            BenchmarkId::new("no_null_move", name),
            position,
            |b, pos| {
                let config = SearchConfig {
                    max_depth: 4,
                    max_time: None,
                    max_nodes: Some(10000),
                    use_null_move_pruning: false,
                    ..SearchConfig::default()
                };
                let mut engine = SearchEngine::new(config);

                b.iter(|| engine.search(pos))
            },
        );

        // Iterative deepening performance
        group.bench_with_input(
            BenchmarkId::new("iterative_deepening", name),
            position,
            |b, pos| {
                let config = SearchConfig {
                    max_depth: 6,
                    max_time: None,
                    max_nodes: Some(50000),
                    ..SearchConfig::default()
                };
                let mut engine = SearchEngine::new(config);

                b.iter(|| engine.search(pos))
            },
        );
    }

    group.finish();
}

fn benchmark_parallel_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_operations");

    let positions: Vec<Position> = (0..100)
        .map(|i| match i % 4 {
            0 => create_test_position(STARTING_POSITION),
            1 => create_test_position(KIWIPETE_POSITION),
            2 => create_test_position(MIDDLE_GAME_POSITION),
            _ => create_test_position(ENDGAME_POSITION),
        })
        .collect();

    let parallel_config = ParallelConfig {
        num_threads: rayon::current_num_threads(),
        chunk_size: 10,
        enable_parallel_moves: true,
        enable_parallel_eval: true,
        enable_parallel_search: true,
    };

    let sequential_config = ParallelConfig {
        enable_parallel_moves: false,
        enable_parallel_eval: false,
        enable_parallel_search: false,
        ..parallel_config.clone()
    };

    // Parallel move generation
    group.bench_function("parallel_move_generation", |b| {
        let generator = ParallelMoveGenerator::new(parallel_config.clone());
        b.iter(|| generator.bulk_generate_moves(&positions))
    });

    group.bench_function("sequential_move_generation", |b| {
        let generator = ParallelMoveGenerator::new(sequential_config.clone());
        b.iter(|| generator.bulk_generate_moves(&positions))
    });

    // Parallel evaluation
    group.bench_function("parallel_evaluation", |b| {
        let mut evaluator = ParallelEvaluator::new(parallel_config.clone());
        b.iter(|| evaluator.bulk_evaluate(&positions))
    });

    group.bench_function("sequential_evaluation", |b| {
        let mut evaluator = ParallelEvaluator::new(sequential_config.clone());
        b.iter(|| evaluator.bulk_evaluate(&positions))
    });

    // Parallel search (smaller set due to computational cost)
    let search_positions = &positions[0..4];

    group.bench_function("parallel_search", |b| {
        let engine = ParallelSearchEngine::new(parallel_config.clone());
        b.iter(|| {
            for pos in search_positions {
                engine.parallel_root_search(pos, 3);
            }
        })
    });

    group.bench_function("sequential_search", |b| {
        let engine = ParallelSearchEngine::new(sequential_config.clone());
        b.iter(|| {
            for pos in search_positions {
                engine.parallel_root_search(pos, 3);
            }
        })
    });

    group.finish();
}

fn benchmark_advanced_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("advanced_evaluation");

    let positions = [
        ("starting", create_test_position(STARTING_POSITION)),
        ("kiwipete", create_test_position(KIWIPETE_POSITION)),
        ("middlegame", create_test_position(MIDDLE_GAME_POSITION)),
        ("endgame", create_test_position(ENDGAME_POSITION)),
    ];

    for (name, position) in &positions {
        // Standard evaluator
        group.bench_with_input(
            BenchmarkId::new("standard_eval", name),
            position,
            |b, pos| {
                let evaluator = Evaluator::new();
                b.iter(|| {
                    let gs = game_state_from_position(pos);
                    evaluator.evaluate(&gs)
                })
            },
        );

        // Optimized evaluator with caching
        group.bench_with_input(
            BenchmarkId::new("optimized_eval", name),
            position,
            |b, pos| {
                let mut evaluator = OptimizedEvaluator::new();
                b.iter(|| evaluator.evaluate(pos))
            },
        );

        // Incremental evaluation
        group.bench_with_input(
            BenchmarkId::new("incremental_eval", name),
            position,
            |b, pos| {
                let mut evaluator = OptimizedEvaluator::new();
                // Prime the cache
                evaluator.evaluate(pos);

                b.iter(|| {
                    // Simulate incremental update
                    evaluator.evaluate_incremental(pos, Some(pos.zobrist_hash()), None)
                })
            },
        );

        // SIMD material evaluation
        group.bench_with_input(
            BenchmarkId::new("simd_material", name),
            position,
            |b, pos| {
                let evaluator = OptimizedEvaluator::new();
                b.iter(|| evaluator.simd_material_evaluation(pos))
            },
        );
    }

    group.finish();
}

fn benchmark_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");

    // Transposition table operations
    group.bench_function("tt_store_probe", |b| {
        let mut tt = TranspositionTable::new(16); // 16MB
        let move_item = Move::new(
            Square::new(8).unwrap(),
            Square::new(16).unwrap(),
            MoveType::Normal,
        );

        b.iter(|| {
            // Store entries
            for i in 0..1000 {
                tt.store(i, i as i32, 5, NodeType::Exact, move_item);
            }

            // Probe entries
            for i in 0..1000 {
                tt.probe(i);
            }
        })
    });

    // Move pool operations
    group.bench_function("move_pool_allocation", |b| {
        let mut pool = MovePool::new(10000);

        b.iter(|| {
            pool.reset();

            // Allocate move batches
            for _ in 0..100 {
                pool.get_moves(50);
            }
        })
    });

    // Optimized move list operations
    group.bench_function("optimized_move_list", |b| {
        b.iter(|| {
            let mut list = OptimizedMoveList::new();
            let dummy_move = Move::new(
                Square::new(0).unwrap(),
                Square::new(1).unwrap(),
                MoveType::Normal,
            );

            // Add moves (testing stack + heap)
            for _ in 0..50 {
                list.push(dummy_move);
            }

            // Iterate through moves
            for move_item in list.iter() {
                criterion::black_box(move_item);
            }
        })
    });

    // Standard Vec comparison
    group.bench_function("standard_vec", |b| {
        b.iter(|| {
            let mut list = Vec::new();
            let dummy_move = Move::new(
                Square::new(0).unwrap(),
                Square::new(1).unwrap(),
                MoveType::Normal,
            );

            // Add moves
            for _ in 0..50 {
                list.push(dummy_move);
            }

            // Iterate through moves
            for move_item in &list {
                criterion::black_box(*move_item);
            }
        })
    });

    group.finish();
}

fn benchmark_scaling_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling_analysis");

    // Test different thread counts for parallel operations
    let thread_counts = [1, 2, 4, 8];
    let positions: Vec<Position> = (0..100)
        .map(|i| match i % 4 {
            0 => create_test_position(STARTING_POSITION),
            1 => create_test_position(KIWIPETE_POSITION),
            2 => create_test_position(MIDDLE_GAME_POSITION),
            _ => create_test_position(ENDGAME_POSITION),
        })
        .collect();

    for &thread_count in &thread_counts {
        let config = ParallelConfig {
            num_threads: thread_count,
            chunk_size: 10,
            enable_parallel_moves: true,
            enable_parallel_eval: true,
            enable_parallel_search: true,
        };

        group.bench_with_input(
            BenchmarkId::new("move_generation_scaling", thread_count),
            &thread_count,
            |b, _| {
                let generator = ParallelMoveGenerator::new(config.clone());
                b.iter(|| generator.bulk_generate_moves(&positions))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("evaluation_scaling", thread_count),
            &thread_count,
            |b, _| {
                let mut evaluator = ParallelEvaluator::new(config.clone());
                b.iter(|| evaluator.bulk_evaluate(&positions))
            },
        );
    }

    // Memory scaling tests
    let tt_sizes_mb = [4, 16, 64, 256];

    for &size_mb in &tt_sizes_mb {
        group.bench_with_input(
            BenchmarkId::new("tt_performance_scaling", size_mb),
            &size_mb,
            |b, _| {
                let mut tt = TranspositionTable::new(size_mb);
                let move_item = Move::new(
                    Square::new(8).unwrap(),
                    Square::new(16).unwrap(),
                    MoveType::Normal,
                );

                b.iter(|| {
                    // Fill table to ~50% capacity
                    let num_entries = (size_mb * 1024 * 512) / 16; // Rough calculation
                    for i in 0..num_entries {
                        tt.store(i as u64, i as i32, 5, NodeType::Exact, move_item);

                        // Interleave some probes
                        if i % 10 == 0 && i > 100 {
                            tt.probe((i - 50) as u64);
                        }
                    }
                })
            },
        );
    }

    group.finish();
}

fn benchmark_comprehensive_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("integration_tests");

    let position = create_test_position(KIWIPETE_POSITION);

    // Full Phase 2 optimized engine
    group.bench_function("phase2_optimized_engine", |b| {
        let memory_config = MemoryConfig {
            tt_size_mb: 32,
            move_pool_size: 5000,
            position_pool_size: 500,
            enable_prefetch: true,
            cache_line_alignment: true,
        };

        let search_config = SearchConfig {
            max_depth: 5,
            max_time: None,
            max_nodes: Some(25000),
            use_null_move_pruning: true,
            use_late_move_reductions: true,
            use_futility_pruning: true,
            aspiration_window: 50,
        };

        let parallel_config = ParallelConfig {
            num_threads: 4,
            chunk_size: 8,
            enable_parallel_moves: true,
            enable_parallel_eval: true,
            enable_parallel_search: true,
        };

        b.iter(|| {
            let mut memory_manager = MemoryManager::new(memory_config.clone());
            let mut search_engine = SearchEngine::new(search_config.clone());
            let mut optimized_evaluator = OptimizedEvaluator::new();
            let parallel_engine = ParallelSearchEngine::new(parallel_config.clone());

            // Simulate a complete search with all optimizations
            memory_manager.new_search();
            let result = search_engine.search(&position);

            // Test parallel components
            let moves = parallel_engine.parallel_root_search(&position, 3);

            // Test evaluation caching
            let eval = optimized_evaluator.evaluate(&position);

            (result, moves, eval)
        })
    });

    // Comparison with Phase 1 (basic) engine
    group.bench_function("phase1_basic_engine", |b| {
        let config = SearchConfig {
            max_depth: 5,
            max_time: None,
            max_nodes: Some(25000),
            use_null_move_pruning: false,
            use_late_move_reductions: false,
            use_futility_pruning: false,
            aspiration_window: 0,
        };

        b.iter(|| {
            let mut basic_engine = SearchEngine::new(config.clone());
            let basic_evaluator = Evaluator::new();
            let move_generator = MoveGenerator::new();

            // Basic search without optimizations
            let result = basic_engine.search(&position);
            let moves = move_generator.generate_legal_moves(&position);
            let eval = {
                let gs = game_state_from_position(&position);
                basic_evaluator.evaluate(&gs)
            };

            (result, moves.len(), eval)
        })
    });

    group.finish();
}

// Criterion benchmark groups
criterion_group!(simd_benches, benchmark_simd_operations);

criterion_group!(search_benches, benchmark_advanced_search);

criterion_group!(parallel_benches, benchmark_parallel_operations);

criterion_group!(eval_benches, benchmark_advanced_evaluation);

criterion_group!(memory_benches, benchmark_memory_operations);

criterion_group!(scaling_benches, benchmark_scaling_analysis);

criterion_group!(integration_benches, benchmark_comprehensive_integration);

criterion_main!(
    simd_benches,
    search_benches,
    parallel_benches,
    eval_benches,
    memory_benches,
    scaling_benches,
    integration_benches
);
