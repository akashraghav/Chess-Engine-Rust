use crate::{Bitboard, Color, PieceType, Square};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoveType {
    Normal,
    Capture,
    EnPassant,
    Castle,
    Promotion { piece: PieceType },
    PromotionCapture { piece: PieceType },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub move_type: MoveType,
}

impl Move {
    #[inline(always)]
    pub const fn new(from: Square, to: Square, move_type: MoveType) -> Self {
        Move { from, to, move_type }
    }

    #[inline(always)]
    pub const fn normal(from: Square, to: Square) -> Self {
        Move::new(from, to, MoveType::Normal)
    }

    #[inline(always)]
    pub const fn capture(from: Square, to: Square) -> Self {
        Move::new(from, to, MoveType::Capture)
    }

    #[inline(always)]
    pub const fn en_passant(from: Square, to: Square) -> Self {
        Move::new(from, to, MoveType::EnPassant)
    }

    #[inline(always)]
    pub const fn castle(from: Square, to: Square) -> Self {
        Move::new(from, to, MoveType::Castle)
    }

    #[inline(always)]
    pub const fn promotion(from: Square, to: Square, piece: PieceType) -> Self {
        Move::new(from, to, MoveType::Promotion { piece })
    }

    #[inline]
    pub const fn promotion_capture(from: Square, to: Square, piece: PieceType) -> Self {
        Move::new(from, to, MoveType::PromotionCapture { piece })
    }

    #[inline]
    pub const fn is_capture(self) -> bool {
        matches!(self.move_type, MoveType::Capture | MoveType::EnPassant | MoveType::PromotionCapture { .. })
    }

    #[inline]
    pub const fn is_promotion(self) -> bool {
        matches!(self.move_type, MoveType::Promotion { .. } | MoveType::PromotionCapture { .. })
    }

    #[inline]
    pub const fn is_castle(self) -> bool {
        matches!(self.move_type, MoveType::Castle)
    }

    #[inline]
    pub const fn is_en_passant(self) -> bool {
        matches!(self.move_type, MoveType::EnPassant)
    }

    pub fn promotion_piece(self) -> Option<PieceType> {
        match self.move_type {
            MoveType::Promotion { piece } | MoveType::PromotionCapture { piece } => Some(piece),
            _ => None,
        }
    }

    pub fn to_uci(self) -> String {
        let promotion = match self.promotion_piece() {
            Some(PieceType::Queen) => "q",
            Some(PieceType::Rook) => "r",
            Some(PieceType::Bishop) => "b",
            Some(PieceType::Knight) => "n",
            _ => "",
        };
        format!("{}{}{}", self.from, self.to, promotion)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_uci())
    }
}

impl std::str::FromStr for Move {
    type Err = crate::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 || s.len() > 5 {
            return Err(crate::ChessError::ParseError(format!("Invalid move format: {}", s)));
        }

        let from_str = &s[0..2];
        let to_str = &s[2..4];

        let from: Square = from_str.parse()?;
        let to: Square = to_str.parse()?;

        if s.len() == 4 {
            Ok(Move::normal(from, to))
        } else {
            let promotion_char = s.chars().nth(4).unwrap();
            let piece = match promotion_char.to_ascii_lowercase() {
                'q' => PieceType::Queen,
                'r' => PieceType::Rook,
                'b' => PieceType::Bishop,
                'n' => PieceType::Knight,
                _ => return Err(crate::ChessError::ParseError(format!("Invalid promotion piece: {}", promotion_char))),
            };
            Ok(Move::promotion(from, to, piece))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveGenerator {
    king_attacks: [Bitboard; 64],
    knight_attacks: [Bitboard; 64],
    pawn_attacks: [[Bitboard; 64]; 2],
}

impl MoveGenerator {
    pub fn new() -> Self {
        let mut generator = MoveGenerator {
            king_attacks: [Bitboard::EMPTY; 64],
            knight_attacks: [Bitboard::EMPTY; 64],
            pawn_attacks: [[Bitboard::EMPTY; 64]; 2],
        };
        generator.initialize_attack_tables();
        generator
    }

    fn initialize_attack_tables(&mut self) {
        for square_idx in 0..64 {
            let square = Square::new(square_idx as u8).unwrap();

            self.king_attacks[square_idx] = self.generate_king_attacks(square);
            self.knight_attacks[square_idx] = self.generate_knight_attacks(square);
            self.pawn_attacks[Color::White.index()][square_idx] = self.generate_pawn_attacks(square, Color::White);
            self.pawn_attacks[Color::Black.index()][square_idx] = self.generate_pawn_attacks(square, Color::Black);
        }
    }

    fn generate_king_attacks(&self, square: Square) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        for target in square.king_moves() {
            attacks |= target.bitboard();
        }
        attacks
    }

    fn generate_knight_attacks(&self, square: Square) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        for target in square.knight_moves() {
            attacks |= target.bitboard();
        }
        attacks
    }

    fn generate_pawn_attacks(&self, square: Square, color: Color) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let square_bb = square.bitboard();

        match color {
            Color::White => {
                if square.file() > 0 {
                    attacks |= square_bb.shift_northwest();
                }
                if square.file() < 7 {
                    attacks |= square_bb.shift_northeast();
                }
            }
            Color::Black => {
                if square.file() > 0 {
                    attacks |= square_bb.shift_southwest();
                }
                if square.file() < 7 {
                    attacks |= square_bb.shift_southeast();
                }
            }
        }
        attacks
    }

    #[inline]
    pub fn king_attacks(&self, square: Square) -> Bitboard {
        self.king_attacks[square.index() as usize]
    }

    #[inline]
    pub fn knight_attacks(&self, square: Square) -> Bitboard {
        self.knight_attacks[square.index() as usize]
    }

    #[inline]
    pub fn pawn_attacks(&self, square: Square, color: Color) -> Bitboard {
        self.pawn_attacks[color.index()][square.index() as usize]
    }

    pub fn rook_attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;

        attacks |= self.ray_attacks(square, occupied, 8, 7 - square.rank());    // Up
        attacks |= self.ray_attacks(square, occupied, -8, square.rank());       // Down
        attacks |= self.ray_attacks(square, occupied, 1, 7 - square.file());   // Right
        attacks |= self.ray_attacks(square, occupied, -1, square.file());       // Left

        attacks
    }

    pub fn bishop_attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let file = square.file();
        let rank = square.rank();

        attacks |= self.ray_attacks(square, occupied, 9, (7 - file).min(7 - rank));
        attacks |= self.ray_attacks(square, occupied, 7, file.min(7 - rank));
        attacks |= self.ray_attacks(square, occupied, -7, (7 - file).min(rank));
        attacks |= self.ray_attacks(square, occupied, -9, file.min(rank));

        attacks
    }

    pub fn queen_attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        self.rook_attacks(square, occupied) | self.bishop_attacks(square, occupied)
    }

    fn ray_attacks(&self, square: Square, occupied: Bitboard, delta: i8, max_distance: u8) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let mut current_square = square.index() as i8;

        for _ in 0..max_distance {
            current_square += delta;
            if current_square < 0 || current_square >= 64 {
                break;
            }

            let target_square = Square::new(current_square as u8).unwrap();
            attacks |= target_square.bitboard();

            if occupied & target_square.bitboard() != Bitboard::EMPTY {
                break;
            }
        }

        attacks
    }

    pub fn generate_pawn_moves(&self, square: Square, color: Color, occupied: Bitboard, enemy_pieces: Bitboard) -> Vec<Move> {
        let mut moves = Vec::new();
        let _square_bb = square.bitboard();

        let (forward_direction, start_rank, promotion_rank) = match color {
            Color::White => (8i8, 1, 7),
            Color::Black => (-8i8, 6, 0),
        };

        let forward_square = Square::new((square.index() as i8 + forward_direction) as u8);
        if let Some(forward) = forward_square {
            if forward.rank() <= 7 && occupied & forward.bitboard() == Bitboard::EMPTY {
                if forward.rank() == promotion_rank {
                    moves.push(Move::promotion(square, forward, PieceType::Queen));
                    moves.push(Move::promotion(square, forward, PieceType::Rook));
                    moves.push(Move::promotion(square, forward, PieceType::Bishop));
                    moves.push(Move::promotion(square, forward, PieceType::Knight));
                } else {
                    moves.push(Move::normal(square, forward));

                    if square.rank() == start_rank {
                        let double_forward = Square::new((forward.index() as i8 + forward_direction) as u8);
                        if let Some(double) = double_forward {
                            if occupied & double.bitboard() == Bitboard::EMPTY {
                                moves.push(Move::normal(square, double));
                            }
                        }
                    }
                }
            }
        }

        let attack_squares = self.pawn_attacks(square, color);
        for target_square_idx in attack_squares.iter() {
            let target_square = Square::from(target_square_idx);
            if enemy_pieces & target_square.bitboard() != Bitboard::EMPTY {
                if target_square.rank() == promotion_rank {
                    moves.push(Move::promotion_capture(square, target_square, PieceType::Queen));
                    moves.push(Move::promotion_capture(square, target_square, PieceType::Rook));
                    moves.push(Move::promotion_capture(square, target_square, PieceType::Bishop));
                    moves.push(Move::promotion_capture(square, target_square, PieceType::Knight));
                } else {
                    moves.push(Move::capture(square, target_square));
                }
            }
        }

        moves
    }

    pub fn generate_piece_moves(&self, square: Square, piece_type: PieceType, occupied: Bitboard, enemy_pieces: Bitboard) -> Vec<Move> {
        let mut moves = Vec::new();

        let attacks = match piece_type {
            PieceType::Knight => self.knight_attacks(square),
            PieceType::Bishop => self.bishop_attacks(square, occupied),
            PieceType::Rook => self.rook_attacks(square, occupied),
            PieceType::Queen => self.queen_attacks(square, occupied),
            PieceType::King => self.king_attacks(square),
            PieceType::Pawn => return moves,
        };

        for target_square_idx in attacks.iter() {
            let target_square = Square::from(target_square_idx);
            if occupied & target_square.bitboard() == Bitboard::EMPTY {
                moves.push(Move::normal(square, target_square));
            } else if enemy_pieces & target_square.bitboard() != Bitboard::EMPTY {
                moves.push(Move::capture(square, target_square));
            }
        }

        moves
    }

    pub fn is_square_attacked(&self, square: Square, by_color: Color, occupied: Bitboard, piece_positions: &[Bitboard; 12]) -> bool {
        let color_offset = by_color.index() * 6;

        if self.pawn_attacks(square, by_color.opposite()) & piece_positions[color_offset + PieceType::Pawn.index()] != Bitboard::EMPTY {
            return true;
        }

        if self.knight_attacks(square) & piece_positions[color_offset + PieceType::Knight.index()] != Bitboard::EMPTY {
            return true;
        }

        if self.bishop_attacks(square, occupied) & piece_positions[color_offset + PieceType::Bishop.index()] != Bitboard::EMPTY {
            return true;
        }

        if self.rook_attacks(square, occupied) & piece_positions[color_offset + PieceType::Rook.index()] != Bitboard::EMPTY {
            return true;
        }

        if self.queen_attacks(square, occupied) & piece_positions[color_offset + PieceType::Queen.index()] != Bitboard::EMPTY {
            return true;
        }

        if self.king_attacks(square) & piece_positions[color_offset + PieceType::King.index()] != Bitboard::EMPTY {
            return true;
        }

        false
    }

    /// Generate all legal moves for the current position
    pub fn generate_legal_moves(&self, position: &crate::Position) -> Vec<Move> {
        let mut moves = Vec::new();
        let side_to_move = position.side_to_move();
        let occupied = position.all_occupied;
        let _own_pieces = position.occupied[side_to_move.index()];
        let enemy_pieces = position.occupied[side_to_move.opposite().index()];
        
        // Generate moves for each piece
        for square_idx in 0..64 {
            let square = Square::new(square_idx as u8).unwrap();
            if let Some(piece) = position.piece_at(square) {
                if piece.color == side_to_move {
                    match piece.piece_type {
                        PieceType::Pawn => {
                            moves.extend(self.generate_pawn_moves(square, piece.color, occupied, enemy_pieces));
                        }
                        piece_type => {
                            moves.extend(self.generate_piece_moves(square, piece_type, occupied, enemy_pieces));
                        }
                    }
                }
            }
        }
        
        // TODO: Add castling, en passant, and legal move filtering
        // For now, return pseudo-legal moves
        moves
    }
}

impl Default for MoveGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_creation() {
        let mv = Move::normal(Square::E2, Square::E4);
        assert_eq!(mv.from, Square::E2);
        assert_eq!(mv.to, Square::E4);
        assert!(!mv.is_capture());
        assert!(!mv.is_promotion());
        assert!(!mv.is_castle());

        let capture = Move::capture(Square::E4, Square::D5);
        assert!(capture.is_capture());

        let promotion = Move::promotion(Square::E7, Square::E8, PieceType::Queen);
        assert!(promotion.is_promotion());
        assert_eq!(promotion.promotion_piece(), Some(PieceType::Queen));
    }

    #[test]
    fn test_move_uci() {
        assert_eq!(Move::normal(Square::E2, Square::E4).to_uci(), "e2e4");
        assert_eq!(Move::promotion(Square::E7, Square::E8, PieceType::Queen).to_uci(), "e7e8q");
        assert_eq!(Move::promotion(Square::A7, Square::B8, PieceType::Knight).to_uci(), "a7b8n");
    }

    #[test]
    fn test_move_from_str() {
        let mv: Move = "e2e4".parse().unwrap();
        assert_eq!(mv.from, Square::E2);
        assert_eq!(mv.to, Square::E4);

        let promotion: Move = "e7e8q".parse().unwrap();
        assert_eq!(promotion.from, Square::E7);
        assert_eq!(promotion.to, Square::E8);
        assert_eq!(promotion.promotion_piece(), Some(PieceType::Queen));

        assert!("e2e4x".parse::<Move>().is_err());
        assert!("e2".parse::<Move>().is_err());
    }

    #[test]
    fn test_move_generator_creation() {
        let generator = MoveGenerator::new();

        assert!(generator.king_attacks(Square::E4).is_not_empty());
        assert!(generator.knight_attacks(Square::E4).is_not_empty());
        assert!(generator.pawn_attacks(Square::E4, Color::White).is_not_empty());
    }

    #[test]
    fn test_king_attacks() {
        let generator = MoveGenerator::new();
        let attacks = generator.king_attacks(Square::E4);

        assert_eq!(attacks.count_bits(), 8);
        assert!(attacks & Square::D3.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::E3.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::F3.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::D4.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::F4.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::D5.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::E5.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::F5.bitboard() != Bitboard::EMPTY);
    }

    #[test]
    fn test_knight_attacks() {
        let generator = MoveGenerator::new();
        let attacks = generator.knight_attacks(Square::E4);

        assert_eq!(attacks.count_bits(), 8);
        assert!(attacks & Square::D2.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::F2.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::C3.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::G3.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::C5.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::G5.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::D6.bitboard() != Bitboard::EMPTY);
        assert!(attacks & Square::F6.bitboard() != Bitboard::EMPTY);
    }

    #[test]
    fn test_pawn_attacks() {
        let generator = MoveGenerator::new();

        let white_attacks = generator.pawn_attacks(Square::E4, Color::White);
        assert_eq!(white_attacks.count_bits(), 2);
        assert!(white_attacks & Square::D5.bitboard() != Bitboard::EMPTY);
        assert!(white_attacks & Square::F5.bitboard() != Bitboard::EMPTY);

        let black_attacks = generator.pawn_attacks(Square::E5, Color::Black);
        assert_eq!(black_attacks.count_bits(), 2);
        assert!(black_attacks & Square::D4.bitboard() != Bitboard::EMPTY);
        assert!(black_attacks & Square::F4.bitboard() != Bitboard::EMPTY);
    }

    #[test]
    fn test_sliding_pieces() {
        let generator = MoveGenerator::new();
        let empty_board = Bitboard::EMPTY;

        let rook_attacks = generator.rook_attacks(Square::E4, empty_board);
        assert!(rook_attacks.count_bits() > 10);
        assert!(rook_attacks & Square::E1.bitboard() != Bitboard::EMPTY);
        assert!(rook_attacks & Square::E8.bitboard() != Bitboard::EMPTY);
        assert!(rook_attacks & Square::A4.bitboard() != Bitboard::EMPTY);
        assert!(rook_attacks & Square::H4.bitboard() != Bitboard::EMPTY);

        let bishop_attacks = generator.bishop_attacks(Square::E4, empty_board);
        assert!(bishop_attacks.count_bits() > 10);
        assert!(bishop_attacks & Square::A8.bitboard() != Bitboard::EMPTY);
        assert!(bishop_attacks & Square::H1.bitboard() != Bitboard::EMPTY);

        let queen_attacks = generator.queen_attacks(Square::E4, empty_board);
        assert!(queen_attacks.count_bits() > 20);
        assert_eq!(queen_attacks, rook_attacks | bishop_attacks);
    }
}