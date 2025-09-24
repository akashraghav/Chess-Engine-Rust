use crate::{GameState, Color, PieceType, Square, Bitboard};

pub struct Evaluator {
    piece_square_tables: PieceSquareTables,
}

struct PieceSquareTables {
    pawn: [i32; 64],
    knight: [i32; 64],
    bishop: [i32; 64],
    rook: [i32; 64],
    queen: [i32; 64],
    king_middlegame: [i32; 64],
    king_endgame: [i32; 64],
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            piece_square_tables: PieceSquareTables::new(),
        }
    }

    pub fn evaluate(&self, game_state: &GameState) -> i32 {
        let mut score = 0;

        score += self.material_balance(game_state);
        score += self.positional_score(game_state);
        score += self.mobility_score(game_state);
        score += self.king_safety_score(game_state);
        score += self.pawn_structure_score(game_state);

        match game_state.position.side_to_move {
            Color::White => score,
            Color::Black => -score,
        }
    }

    fn material_balance(&self, game_state: &GameState) -> i32 {
        let mut score = 0;

        for piece_type in PieceType::ALL {
            let white_count = game_state.position.pieces_of_type(piece_type, Color::White).count_bits() as i32;
            let black_count = game_state.position.pieces_of_type(piece_type, Color::Black).count_bits() as i32;

            score += (white_count - black_count) * piece_type.value();
        }

        score
    }

    fn positional_score(&self, game_state: &GameState) -> i32 {
        let mut score = 0;
        let is_endgame = self.is_endgame(game_state);

        for square_idx in 0..64 {
            let square = Square::new(square_idx).unwrap();
            if let Some(piece) = game_state.position.piece_at(square) {
                // For black pieces, flip the square index (rank-wise) since tables are from White's perspective
                let table_idx = match piece.color {
                    Color::White => square_idx as usize,
                    Color::Black => (square_idx ^ 56) as usize, // Flip rank: XOR with 56 (7*8)
                };
                
                let piece_score = match piece.piece_type {
                    PieceType::Pawn => self.piece_square_tables.pawn[table_idx],
                    PieceType::Knight => self.piece_square_tables.knight[table_idx],
                    PieceType::Bishop => self.piece_square_tables.bishop[table_idx],
                    PieceType::Rook => self.piece_square_tables.rook[table_idx],
                    PieceType::Queen => self.piece_square_tables.queen[table_idx],
                    PieceType::King => {
                        if is_endgame {
                            self.piece_square_tables.king_endgame[table_idx]
                        } else {
                            self.piece_square_tables.king_middlegame[table_idx]
                        }
                    }
                };

                match piece.color {
                    Color::White => score += piece_score,
                    Color::Black => score -= piece_score,
                }
            }
        }

        score
    }

    fn mobility_score(&self, game_state: &GameState) -> i32 {
        let white_mobility = self.count_mobility(game_state, Color::White);
        let black_mobility = self.count_mobility(game_state, Color::Black);

        (white_mobility - black_mobility) * 10
    }

    fn count_mobility(&self, game_state: &GameState, color: Color) -> i32 {
        let mut mobility = 0;

        for square_idx in 0..64 {
            let square = Square::new(square_idx).unwrap();
            if let Some(piece) = game_state.position.piece_at(square) {
                if piece.color == color {
                    mobility += match piece.piece_type {
                        PieceType::Knight => game_state.move_generator.knight_attacks(square).count_bits(),
                        PieceType::Bishop => game_state.move_generator.bishop_attacks(square, game_state.position.all_pieces()).count_bits(),
                        PieceType::Rook => game_state.move_generator.rook_attacks(square, game_state.position.all_pieces()).count_bits(),
                        PieceType::Queen => game_state.move_generator.queen_attacks(square, game_state.position.all_pieces()).count_bits(),
                        _ => 0,
                    } as i32;
                }
            }
        }

        mobility
    }

    fn king_safety_score(&self, game_state: &GameState) -> i32 {
        let white_safety = self.evaluate_king_safety(game_state, Color::White);
        let black_safety = self.evaluate_king_safety(game_state, Color::Black);

        white_safety - black_safety
    }

    fn evaluate_king_safety(&self, game_state: &GameState, color: Color) -> i32 {
        if let Some(king_square) = game_state.position.king_square(color) {
            let mut safety = 0;

            let king_zone = self.get_king_zone(king_square);
            let enemy_attacks = self.get_attack_map(game_state, color.opposite());

            let attacked_squares = (king_zone & enemy_attacks).count_bits();
            safety -= attacked_squares as i32 * 20;

            let pawn_shield = self.evaluate_pawn_shield(game_state, king_square, color);
            safety += pawn_shield;

            safety
        } else {
            -1000
        }
    }

    fn get_king_zone(&self, king_square: Square) -> Bitboard {
        let mut zone = king_square.bitboard();
        zone |= zone.shift_north() | zone.shift_south();
        zone |= zone.shift_east() | zone.shift_west();
        zone
    }

    fn get_attack_map(&self, game_state: &GameState, color: Color) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;

        for square_idx in 0..64 {
            let square = Square::new(square_idx).unwrap();
            if let Some(piece) = game_state.position.piece_at(square) {
                if piece.color == color {
                    attacks |= match piece.piece_type {
                        PieceType::Pawn => game_state.move_generator.pawn_attacks(square, color),
                        PieceType::Knight => game_state.move_generator.knight_attacks(square),
                        PieceType::Bishop => game_state.move_generator.bishop_attacks(square, game_state.position.all_pieces()),
                        PieceType::Rook => game_state.move_generator.rook_attacks(square, game_state.position.all_pieces()),
                        PieceType::Queen => game_state.move_generator.queen_attacks(square, game_state.position.all_pieces()),
                        PieceType::King => game_state.move_generator.king_attacks(square),
                    };
                }
            }
        }

        attacks
    }

    fn evaluate_pawn_shield(&self, game_state: &GameState, king_square: Square, color: Color) -> i32 {
        let mut shield_score = 0i32;
        let direction = match color {
            Color::White => 1,
            Color::Black => -1,
        };

        for file_offset in -1..=1 {
            let file = king_square.file() as i8 + file_offset;
            if file >= 0 && file < 8 {
                for rank_offset in 1..=2 {
                    let rank = king_square.rank() as i8 + direction * rank_offset;
                    if rank >= 0 && rank < 8 {
                        if let Some(square) = Square::from_file_rank(file as u8, rank as u8) {
                            if let Some(piece) = game_state.position.piece_at(square) {
                                if piece.piece_type == PieceType::Pawn && piece.color == color {
                                    shield_score += 15 - (rank_offset as i32 - 1) * 5;
                                }
                            }
                        }
                    }
                }
            }
        }

        shield_score
    }

    fn pawn_structure_score(&self, game_state: &GameState) -> i32 {
        let white_score = self.evaluate_pawn_structure(game_state, Color::White);
        let black_score = self.evaluate_pawn_structure(game_state, Color::Black);

        white_score - black_score
    }

    fn evaluate_pawn_structure(&self, game_state: &GameState, color: Color) -> i32 {
        let mut score = 0;
        let pawns = game_state.position.pieces_of_type(PieceType::Pawn, color);

        score += self.count_doubled_pawns(pawns) * -10;
        score += self.count_isolated_pawns(pawns) * -15;
        score += self.count_passed_pawns(game_state, color) * 20;

        score
    }

    fn count_doubled_pawns(&self, pawns: Bitboard) -> i32 {
        let mut doubled = 0;
        for file in 0..8 {
            let file_mask = match file {
                0 => Bitboard::FILE_A,
                1 => Bitboard::FILE_B,
                2 => Bitboard::FILE_C,
                3 => Bitboard::FILE_D,
                4 => Bitboard::FILE_E,
                5 => Bitboard::FILE_F,
                6 => Bitboard::FILE_G,
                7 => Bitboard::FILE_H,
                _ => Bitboard::EMPTY,
            };

            let pawns_on_file = (pawns & file_mask).count_bits();
            if pawns_on_file > 1 {
                doubled += pawns_on_file as i32 - 1;
            }
        }
        doubled
    }

    fn count_isolated_pawns(&self, pawns: Bitboard) -> i32 {
        let mut isolated = 0;

        for file in 0..8 {
            let file_mask = match file {
                0 => Bitboard::FILE_A,
                1 => Bitboard::FILE_B,
                2 => Bitboard::FILE_C,
                3 => Bitboard::FILE_D,
                4 => Bitboard::FILE_E,
                5 => Bitboard::FILE_F,
                6 => Bitboard::FILE_G,
                7 => Bitboard::FILE_H,
                _ => Bitboard::EMPTY,
            };

            if (pawns & file_mask).is_not_empty() {
                let adjacent_files = if file > 0 && file < 7 {
                    let left_file = match file - 1 {
                        0 => Bitboard::FILE_A,
                        1 => Bitboard::FILE_B,
                        2 => Bitboard::FILE_C,
                        3 => Bitboard::FILE_D,
                        4 => Bitboard::FILE_E,
                        5 => Bitboard::FILE_F,
                        6 => Bitboard::FILE_G,
                        _ => Bitboard::EMPTY,
                    };
                    let right_file = match file + 1 {
                        1 => Bitboard::FILE_B,
                        2 => Bitboard::FILE_C,
                        3 => Bitboard::FILE_D,
                        4 => Bitboard::FILE_E,
                        5 => Bitboard::FILE_F,
                        6 => Bitboard::FILE_G,
                        7 => Bitboard::FILE_H,
                        _ => Bitboard::EMPTY,
                    };
                    left_file | right_file
                } else if file == 0 {
                    Bitboard::FILE_B
                } else {
                    Bitboard::FILE_G
                };

                if (pawns & adjacent_files).is_empty() {
                    isolated += (pawns & file_mask).count_bits() as i32;
                }
            }
        }

        isolated
    }

    fn count_passed_pawns(&self, game_state: &GameState, color: Color) -> i32 {
        let our_pawns = game_state.position.pieces_of_type(PieceType::Pawn, color);
        let enemy_pawns = game_state.position.pieces_of_type(PieceType::Pawn, color.opposite());
        let mut passed = 0;

        for square_idx in our_pawns.iter() {
            let square = Square::from(square_idx);
            if self.is_passed_pawn(square, color, enemy_pawns) {
                passed += 1;
            }
        }

        passed
    }

    fn is_passed_pawn(&self, pawn_square: Square, color: Color, enemy_pawns: Bitboard) -> bool {
        let file = pawn_square.file();
        let rank = pawn_square.rank();

        let blocking_mask = match color {
            Color::White => {
                let mut mask = Bitboard::EMPTY;
                for r in (rank + 1)..8 {
                    for f in file.saturating_sub(1)..=(file + 1).min(7) {
                        if let Some(sq) = Square::from_file_rank(f, r) {
                            mask |= sq.bitboard();
                        }
                    }
                }
                mask
            }
            Color::Black => {
                let mut mask = Bitboard::EMPTY;
                for r in 0..rank {
                    for f in file.saturating_sub(1)..=(file + 1).min(7) {
                        if let Some(sq) = Square::from_file_rank(f, r) {
                            mask |= sq.bitboard();
                        }
                    }
                }
                mask
            }
        };

        (enemy_pawns & blocking_mask).is_empty()
    }

    fn is_endgame(&self, game_state: &GameState) -> bool {
        let white_material = self.calculate_material(game_state, Color::White);
        let black_material = self.calculate_material(game_state, Color::Black);

        white_material + black_material < 2000
    }

    fn calculate_material(&self, game_state: &GameState, color: Color) -> i32 {
        let mut material = 0;

        for piece_type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
            let count = game_state.position.pieces_of_type(piece_type, color).count_bits() as i32;
            material += count * piece_type.value();
        }

        material
    }
}

impl PieceSquareTables {
    fn new() -> Self {
        PieceSquareTables {
            pawn: [
                 0,  0,  0,  0,  0,  0,  0,  0,
                50, 50, 50, 50, 50, 50, 50, 50,
                10, 10, 20, 30, 30, 20, 10, 10,
                 5,  5, 10, 25, 25, 10,  5,  5,
                 0,  0,  0, 20, 20,  0,  0,  0,
                 5, -5,-10,  0,  0,-10, -5,  5,
                 5, 10, 10,-20,-20, 10, 10,  5,
                 0,  0,  0,  0,  0,  0,  0,  0
            ],
            knight: [
                -50,-40,-30,-30,-30,-30,-40,-50,
                -40,-20,  0,  0,  0,  0,-20,-40,
                -30,  0, 10, 15, 15, 10,  0,-30,
                -30,  5, 15, 20, 20, 15,  5,-30,
                -30,  0, 15, 20, 20, 15,  0,-30,
                -30,  5, 10, 15, 15, 10,  5,-30,
                -40,-20,  0,  5,  5,  0,-20,-40,
                -50,-40,-30,-30,-30,-30,-40,-50,
            ],
            bishop: [
                -20,-10,-10,-10,-10,-10,-10,-20,
                -10,  0,  0,  0,  0,  0,  0,-10,
                -10,  0,  5, 10, 10,  5,  0,-10,
                -10,  5,  5, 10, 10,  5,  5,-10,
                -10,  0, 10, 10, 10, 10,  0,-10,
                -10, 10, 10, 10, 10, 10, 10,-10,
                -10,  5,  0,  0,  0,  0,  5,-10,
                -20,-10,-10,-10,-10,-10,-10,-20,
            ],
            rook: [
                 0,  0,  0,  0,  0,  0,  0,  0,
                 5, 10, 10, 10, 10, 10, 10,  5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                 0,  0,  0,  5,  5,  0,  0,  0
            ],
            queen: [
                -20,-10,-10, -5, -5,-10,-10,-20,
                -10,  0,  0,  0,  0,  0,  0,-10,
                -10,  0,  5,  5,  5,  5,  0,-10,
                 -5,  0,  5,  5,  5,  5,  0, -5,
                  0,  0,  5,  5,  5,  5,  0, -5,
                -10,  5,  5,  5,  5,  5,  0,-10,
                -10,  0,  5,  0,  0,  0,  0,-10,
                -20,-10,-10, -5, -5,-10,-10,-20
            ],
            king_middlegame: [
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -20,-30,-30,-40,-40,-30,-30,-20,
                -10,-20,-20,-20,-20,-20,-20,-10,
                 20, 20,  0,  0,  0,  0, 20, 20,
                 20, 30, 10,  0,  0, 10, 30, 20
            ],
            king_endgame: [
                -50,-40,-30,-20,-20,-30,-40,-50,
                -30,-20,-10,  0,  0,-10,-20,-30,
                -30,-10, 20, 30, 30, 20,-10,-30,
                -30,-10, 30, 40, 40, 30,-10,-30,
                -30,-10, 30, 40, 40, 30,-10,-30,
                -30,-10, 20, 30, 30, 20,-10,-30,
                -30,-30,  0,  0,  0,  0,-30,-30,
                -50,-30,-30,-30,-30,-30,-30,-50
            ],
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator_creation() {
        let evaluator = Evaluator::new();
        let game_state = GameState::new();

        let score = evaluator.evaluate(&game_state);
        
        // Starting position should be approximately equal (both sides have same material and position)
        // A score of 0 or close to 0 is expected for the starting position
        assert_eq!(score, 0);
    }

    #[test]
    fn test_material_evaluation() {
        let evaluator = Evaluator::new();
        let mut game_state = GameState::new();

        game_state.position.remove_piece(Square::D8);

        let score = evaluator.evaluate(&game_state);
        assert!(score > 0);
    }

    #[test]
    fn test_endgame_detection() {
        let evaluator = Evaluator::new();
        let game_state = GameState::from_fen("8/8/8/8/8/8/8/K6k w - - 0 1").unwrap();

        assert!(evaluator.is_endgame(&game_state));
    }
}