use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use chess_core::{Bitboard, Square, MoveGenerator, Color};

fn benchmark_bitboard_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitboard_operations");

    let bb1 = Bitboard::new(0x123456789ABCDEF0);
    let bb2 = Bitboard::new(0xFEDCBA9876543210);

    group.bench_function("bitwise_and", |b| {
        b.iter(|| {
            let result = black_box(bb1 & bb2);
            black_box(result)
        })
    });

    group.bench_function("bitwise_or", |b| {
        b.iter(|| {
            let result = black_box(bb1 | bb2);
            black_box(result)
        })
    });

    group.bench_function("bitwise_xor", |b| {
        b.iter(|| {
            let result = black_box(bb1 ^ bb2);
            black_box(result)
        })
    });

    group.bench_function("bitwise_not", |b| {
        b.iter(|| {
            let result = black_box(!bb1);
            black_box(result)
        })
    });

    group.bench_function("count_bits", |b| {
        b.iter(|| {
            let count = black_box(bb1.count_bits());
            black_box(count)
        })
    });

    group.bench_function("trailing_zeros", |b| {
        b.iter(|| {
            let zeros = black_box(bb1.trailing_zeros());
            black_box(zeros)
        })
    });

    group.bench_function("leading_zeros", |b| {
        b.iter(|| {
            let zeros = black_box(bb1.leading_zeros());
            black_box(zeros)
        })
    });

    group.finish();
}

fn benchmark_bitboard_shifts(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitboard_shifts");

    let bb = Bitboard::new(0x0000001818000000); // Center squares

    group.bench_function("shift_north", |b| {
        b.iter(|| {
            let result = black_box(bb.shift_north());
            black_box(result)
        })
    });

    group.bench_function("shift_south", |b| {
        b.iter(|| {
            let result = black_box(bb.shift_south());
            black_box(result)
        })
    });

    group.bench_function("shift_east", |b| {
        b.iter(|| {
            let result = black_box(bb.shift_east());
            black_box(result)
        })
    });

    group.bench_function("shift_west", |b| {
        b.iter(|| {
            let result = black_box(bb.shift_west());
            black_box(result)
        })
    });

    group.bench_function("shift_northeast", |b| {
        b.iter(|| {
            let result = black_box(bb.shift_northeast());
            black_box(result)
        })
    });

    group.bench_function("shift_northwest", |b| {
        b.iter(|| {
            let result = black_box(bb.shift_northwest());
            black_box(result)
        })
    });

    group.bench_function("shift_southeast", |b| {
        b.iter(|| {
            let result = black_box(bb.shift_southeast());
            black_box(result)
        })
    });

    group.bench_function("shift_southwest", |b| {
        b.iter(|| {
            let result = black_box(bb.shift_southwest());
            black_box(result)
        })
    });

    group.finish();
}

fn benchmark_bit_manipulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("bit_manipulation");

    let bb = Bitboard::new(0xFF000000000000FF);

    group.bench_function("pop_lsb", |b| {
        b.iter(|| {
            let mut test_bb = bb;
            let mut count = 0;
            while let Some(_square) = test_bb.pop_lsb() {
                count += 1;
            }
            black_box(count)
        })
    });

    group.bench_function("lsb", |b| {
        b.iter(|| {
            let lsb = black_box(bb.lsb());
            black_box(lsb)
        })
    });

    group.bench_function("msb", |b| {
        b.iter(|| {
            let msb = black_box(bb.msb());
            black_box(msb)
        })
    });

    group.bench_function("iterator", |b| {
        b.iter(|| {
            let squares: Vec<u32> = black_box(bb.iter().collect());
            black_box(squares.len())
        })
    });

    group.bench_function("squares_vec", |b| {
        b.iter(|| {
            let squares = black_box(bb.squares());
            black_box(squares.len())
        })
    });

    group.finish();
}

fn benchmark_attack_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("attack_generation");

    let move_generator = MoveGenerator::new();
    let center_square = Square::E4;
    let corner_square = Square::A1;
    let _edge_square = Square::E1;

    let empty_board = Bitboard::EMPTY;
    let occupied_board = Bitboard::new(0x0000001818000000);

    // King attacks
    group.bench_function("king_attacks_center", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.king_attacks(center_square));
            black_box(attacks)
        })
    });

    group.bench_function("king_attacks_corner", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.king_attacks(corner_square));
            black_box(attacks)
        })
    });

    // Knight attacks
    group.bench_function("knight_attacks_center", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.knight_attacks(center_square));
            black_box(attacks)
        })
    });

    group.bench_function("knight_attacks_corner", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.knight_attacks(corner_square));
            black_box(attacks)
        })
    });

    // Pawn attacks
    group.bench_function("white_pawn_attacks", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.pawn_attacks(center_square, Color::White));
            black_box(attacks)
        })
    });

    group.bench_function("black_pawn_attacks", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.pawn_attacks(center_square, Color::Black));
            black_box(attacks)
        })
    });

    // Sliding piece attacks
    group.bench_function("rook_attacks_empty", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.rook_attacks(center_square, empty_board));
            black_box(attacks)
        })
    });

    group.bench_function("rook_attacks_occupied", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.rook_attacks(center_square, occupied_board));
            black_box(attacks)
        })
    });

    group.bench_function("bishop_attacks_empty", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.bishop_attacks(center_square, empty_board));
            black_box(attacks)
        })
    });

    group.bench_function("bishop_attacks_occupied", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.bishop_attacks(center_square, occupied_board));
            black_box(attacks)
        })
    });

    group.bench_function("queen_attacks_empty", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.queen_attacks(center_square, empty_board));
            black_box(attacks)
        })
    });

    group.bench_function("queen_attacks_occupied", |b| {
        b.iter(|| {
            let attacks = black_box(move_generator.queen_attacks(center_square, occupied_board));
            black_box(attacks)
        })
    });

    group.finish();
}

fn benchmark_square_attacks(c: &mut Criterion) {
    let mut group = c.benchmark_group("square_attacks");

    let move_generator = MoveGenerator::new();
    let pieces = [Bitboard::EMPTY; 12]; // Empty piece array for testing

    let squares = [
        ("center", Square::E4),
        ("corner", Square::A1),
        ("edge", Square::E1),
        ("near_corner", Square::B2),
    ];

    for (name, square) in squares.iter() {
        group.bench_with_input(
            BenchmarkId::new("is_attacked_by_white", name),
            square,
            |b, &sq| {
                b.iter(|| {
                    let attacked = black_box(move_generator.is_square_attacked(
                        sq,
                        Color::White,
                        Bitboard::EMPTY,
                        &pieces,
                    ));
                    black_box(attacked)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("is_attacked_by_black", name),
            square,
            |b, &sq| {
                b.iter(|| {
                    let attacked = black_box(move_generator.is_square_attacked(
                        sq,
                        Color::Black,
                        Bitboard::EMPTY,
                        &pieces,
                    ));
                    black_box(attacked)
                })
            },
        );
    }

    group.finish();
}

fn benchmark_bitboard_constants(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitboard_constants");

    group.bench_function("rank_masks", |b| {
        b.iter(|| {
            let ranks = [
                Bitboard::RANK_1, Bitboard::RANK_2, Bitboard::RANK_3, Bitboard::RANK_4,
                Bitboard::RANK_5, Bitboard::RANK_6, Bitboard::RANK_7, Bitboard::RANK_8,
            ];
            black_box(ranks)
        })
    });

    group.bench_function("file_masks", |b| {
        b.iter(|| {
            let files = [
                Bitboard::FILE_A, Bitboard::FILE_B, Bitboard::FILE_C, Bitboard::FILE_D,
                Bitboard::FILE_E, Bitboard::FILE_F, Bitboard::FILE_G, Bitboard::FILE_H,
            ];
            black_box(files)
        })
    });

    group.bench_function("color_masks", |b| {
        b.iter(|| {
            let colors = [Bitboard::LIGHT_SQUARES, Bitboard::DARK_SQUARES];
            black_box(colors)
        })
    });

    group.finish();
}

fn benchmark_complex_bitboard_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_operations");

    let bb1 = Bitboard::new(0x123456789ABCDEF0);
    let bb2 = Bitboard::new(0xFEDCBA9876543210);
    let bb3 = Bitboard::new(0x0F0F0F0F0F0F0F0F);

    group.bench_function("triple_operation", |b| {
        b.iter(|| {
            let result = black_box((bb1 | bb2) & bb3);
            black_box(result)
        })
    });

    group.bench_function("multiple_shifts", |b| {
        b.iter(|| {
            let result = black_box(
                bb1.shift_north()
                    .shift_east()
                    .shift_south()
                    .shift_west()
            );
            black_box(result)
        })
    });

    group.bench_function("complex_pattern", |b| {
        b.iter(|| {
            let pattern = black_box(
                (bb1 | bb2.shift_north()) &
                (!bb3.shift_east()) |
                (bb1.shift_south() & bb2.shift_west())
            );
            black_box(pattern)
        })
    });

    group.bench_function("iterative_operations", |b| {
        b.iter(|| {
            let mut result = bb1;
            for _ in 0..8 {
                result = result.shift_north() | result.shift_south();
            }
            black_box(result)
        })
    });

    group.finish();
}

fn benchmark_memory_access_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");

    // Sequential access
    group.bench_function("sequential_squares", |b| {
        b.iter(|| {
            let mut total = 0u32;
            for i in 0..64 {
                if let Some(square) = Square::new(i) {
                    let bb = square.bitboard();
                    total += bb.count_bits();
                }
            }
            black_box(total)
        })
    });

    // Random access pattern
    let random_indices = [21, 7, 45, 3, 62, 18, 9, 33, 51, 14, 39, 6, 58, 27, 41, 12];
    group.bench_function("random_squares", |b| {
        b.iter(|| {
            let mut total = 0u32;
            for &i in random_indices.iter() {
                if let Some(square) = Square::new(i) {
                    let bb = square.bitboard();
                    total += bb.count_bits();
                }
            }
            black_box(total)
        })
    });

    // Cache-friendly operations
    group.bench_function("cache_friendly", |b| {
        let bitboards: Vec<Bitboard> = (0..64).map(|i| Bitboard::new(1u64 << i)).collect();

        b.iter(|| {
            let mut result = Bitboard::EMPTY;
            for bb in bitboards.iter() {
                result |= *bb;
            }
            black_box(result)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_bitboard_operations,
    benchmark_bitboard_shifts,
    benchmark_bit_manipulation,
    benchmark_attack_generation,
    benchmark_square_attacks,
    benchmark_bitboard_constants,
    benchmark_complex_bitboard_operations,
    benchmark_memory_access_patterns
);
criterion_main!(benches);