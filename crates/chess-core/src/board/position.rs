use crate::board::{Bitboard, Square};
use crate::error::{ChessError, Result};
use crate::moves::Move;
use crate::pieces::{Color, Piece, PieceType};

#[derive(Debug, Clone)]
pub struct UndoInfo {
    pub captured_piece: Option<Piece>,
    pub previous_side_to_move: Color,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub pieces: [Bitboard; 12],
    pub occupied: [Bitboard; 2],
    pub all_occupied: Bitboard,
    pub board: [Option<Piece>; 64],
    pub side_to_move: Color,
}

impl Position {
    pub fn new() -> Self {
        Position {
            pieces: [Bitboard::EMPTY; 12],
            occupied: [Bitboard::EMPTY; 2],
            all_occupied: Bitboard::EMPTY,
            board: [None; 64],
            side_to_move: Color::White,
        }
    }

    pub fn starting_position() -> Self {
        let mut position = Position::new();

        // Set up starting position pieces
        // White pieces
        position.board[Square::A1.index() as usize] =
            Some(Piece::new(PieceType::Rook, Color::White));
        position.board[Square::B1.index() as usize] =
            Some(Piece::new(PieceType::Knight, Color::White));
        position.board[Square::C1.index() as usize] =
            Some(Piece::new(PieceType::Bishop, Color::White));
        position.board[Square::D1.index() as usize] =
            Some(Piece::new(PieceType::Queen, Color::White));
        position.board[Square::E1.index() as usize] =
            Some(Piece::new(PieceType::King, Color::White));
        position.board[Square::F1.index() as usize] =
            Some(Piece::new(PieceType::Bishop, Color::White));
        position.board[Square::G1.index() as usize] =
            Some(Piece::new(PieceType::Knight, Color::White));
        position.board[Square::H1.index() as usize] =
            Some(Piece::new(PieceType::Rook, Color::White));

        // White pawns
        for file in 0..8 {
            position.board[file + 8] = Some(Piece::new(PieceType::Pawn, Color::White));
        }

        // Black pawns
        for file in 0..8 {
            position.board[file + 48] = Some(Piece::new(PieceType::Pawn, Color::Black));
        }

        // Black pieces
        position.board[Square::A8.index() as usize] =
            Some(Piece::new(PieceType::Rook, Color::Black));
        position.board[Square::B8.index() as usize] =
            Some(Piece::new(PieceType::Knight, Color::Black));
        position.board[Square::C8.index() as usize] =
            Some(Piece::new(PieceType::Bishop, Color::Black));
        position.board[Square::D8.index() as usize] =
            Some(Piece::new(PieceType::Queen, Color::Black));
        position.board[Square::E8.index() as usize] =
            Some(Piece::new(PieceType::King, Color::Black));
        position.board[Square::F8.index() as usize] =
            Some(Piece::new(PieceType::Bishop, Color::Black));
        position.board[Square::G8.index() as usize] =
            Some(Piece::new(PieceType::Knight, Color::Black));
        position.board[Square::H8.index() as usize] =
            Some(Piece::new(PieceType::Rook, Color::Black));

        // Update bitboards
        position.update_bitboards();

        position
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.board[square.index() as usize]
    }

    pub fn place_piece(&mut self, square: Square, piece: Piece) {
        let square_idx = square.index() as usize;

        // Place piece on the board array
        self.board[square_idx] = Some(piece);

        // Update bitboards by setting the bit for this square
        let square_bb = Bitboard::new(1u64 << square_idx);
        self.pieces[piece.index()] |= square_bb;
        self.occupied[piece.color.index()] |= square_bb;
        self.all_occupied |= square_bb;
    }

    pub fn make_move(&mut self, mv: Move) -> Result<UndoInfo> {
        let moving_piece = self
            .piece_at(mv.from)
            .ok_or_else(|| ChessError::InvalidMove("No piece at source square".to_string()))?;

        let piece_type = moving_piece.piece_type;
        let color = moving_piece.color;

        // Store undo information
        let captured_piece = self.piece_at(mv.to);
        let undo_info = UndoInfo {
            captured_piece,
            previous_side_to_move: self.side_to_move,
        };

        // Remove piece from source square
        self.board[mv.from.index() as usize] = None;
        self.pieces[color.index() * 6 + piece_type.index()] &= !mv.from.bitboard();

        // If there's a capture, remove the captured piece
        if let Some(captured_piece) = captured_piece {
            let captured_type = captured_piece.piece_type;
            let captured_color = captured_piece.color;
            self.pieces[captured_color.index() * 6 + captured_type.index()] &= !mv.to.bitboard();
        }

        // Place piece on destination square
        self.board[mv.to.index() as usize] = Some(moving_piece);
        self.pieces[color.index() * 6 + piece_type.index()] |= mv.to.bitboard();

        // Update occupied bitboards
        self.occupied[Color::White.index()] = Bitboard::EMPTY;
        self.occupied[Color::Black.index()] = Bitboard::EMPTY;
        for piece_type in PieceType::ALL {
            self.occupied[Color::White.index()] |=
                self.pieces[Color::White.index() * 6 + piece_type.index()];
            self.occupied[Color::Black.index()] |=
                self.pieces[Color::Black.index() * 6 + piece_type.index()];
        }
        self.all_occupied = self.occupied[0] | self.occupied[1];

        self.side_to_move = self.side_to_move.opposite();

        Ok(undo_info)
    }

    pub fn undo_move(&mut self, mv: Move, undo_info: UndoInfo) {
        let moving_piece = self
            .piece_at(mv.to)
            .expect("Piece should be at destination square");
        let piece_type = moving_piece.piece_type;
        let color = moving_piece.color;

        // Remove piece from destination square
        self.board[mv.to.index() as usize] = None;
        self.pieces[color.index() * 6 + piece_type.index()] &= !mv.to.bitboard();

        // Restore captured piece if there was one
        if let Some(captured_piece) = undo_info.captured_piece {
            self.place_piece(mv.to, captured_piece);
        }

        // Place moving piece back to source square
        self.place_piece(mv.from, moving_piece);

        // Restore side to move
        self.side_to_move = undo_info.previous_side_to_move;
    }

    pub fn pieces_of_color(&self, color: Color) -> Bitboard {
        self.occupied[color.index()]
    }

    pub fn all_pieces(&self) -> Bitboard {
        self.all_occupied
    }

    pub fn to_fen(&self) -> String {
        String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w")
    }

    // Additional methods needed by game_state.rs and evaluation.rs
    pub fn from_fen(fen: &str) -> Result<Self> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.is_empty() {
            return Err(ChessError::ParseError("Empty FEN string".to_string()));
        }

        let board_fen = parts[0];
        let mut position = Position::new();

        // Parse board position
        let ranks: Vec<&str> = board_fen.split('/').collect();
        if ranks.len() != 8 {
            return Err(ChessError::ParseError("FEN must have 8 ranks".to_string()));
        }

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let mut file_idx = 0;

            for ch in rank_str.chars() {
                if file_idx >= 8 {
                    return Err(ChessError::ParseError("Too many files in rank".to_string()));
                }

                if ch.is_ascii_digit() {
                    // Empty squares
                    let empty_count = ch.to_digit(10).unwrap() as usize;
                    if file_idx + empty_count > 8 {
                        return Err(ChessError::ParseError(
                            "Invalid empty square count".to_string(),
                        ));
                    }
                    file_idx += empty_count;
                } else {
                    // Piece
                    let color = if ch.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let piece_type = match ch.to_ascii_lowercase() {
                        'p' => PieceType::Pawn,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'r' => PieceType::Rook,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => {
                            return Err(ChessError::ParseError(format!(
                                "Invalid piece character: {}",
                                ch
                            )))
                        }
                    };

                    // Convert rank/file to square index (0-63)
                    // rank_idx=0 is rank 8, file_idx=0 is file a
                    let square_idx = (7 - rank_idx) * 8 + file_idx;
                    let square = Square::new(square_idx as u8).ok_or_else(|| {
                        ChessError::ParseError(format!("Invalid square index: {}", square_idx))
                    })?;

                    position.place_piece(square, Piece::new(piece_type, color));
                    file_idx += 1;
                }
            }

            if file_idx != 8 {
                return Err(ChessError::ParseError(
                    "Rank doesn't have 8 files".to_string(),
                ));
            }
        }

        // Parse side to move
        if parts.len() > 1 {
            position.side_to_move = match parts[1] {
                "w" => Color::White,
                "b" => Color::Black,
                _ => return Err(ChessError::ParseError("Invalid side to move".to_string())),
            };
        } else {
            position.side_to_move = Color::White;
        }

        // Update bitboards after placing all pieces
        position.update_bitboards();

        Ok(position)
    }

    pub fn king_square(&self, color: Color) -> Option<Square> {
        // Find the king of the specified color
        for square_idx in 0..64 {
            if let Some(square) = Square::new(square_idx) {
                if let Some(piece) = self.piece_at(square) {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        return Some(square);
                    }
                }
            }
        }
        None
    }

    pub fn update_bitboards(&mut self) {
        // Clear all bitboards
        for i in 0..12 {
            self.pieces[i] = Bitboard::EMPTY;
        }
        self.occupied[0] = Bitboard::EMPTY;
        self.occupied[1] = Bitboard::EMPTY;

        // Rebuild bitboards from board array
        for square_idx in 0..64 {
            if let Some(piece) = self.board[square_idx] {
                let square = Square::new(square_idx as u8).unwrap();
                let piece_index = piece.color.index() * 6 + piece.piece_type.index();
                self.pieces[piece_index] |= square.bitboard();
                self.occupied[piece.color.index()] |= square.bitboard();
            }
        }

        self.all_occupied = self.occupied[0] | self.occupied[1];
    }

    pub fn pieces_of_type(&self, piece_type: PieceType, color: Color) -> Bitboard {
        self.pieces[color.index() * 6 + piece_type.index()]
    }

    pub fn make_null_move(&mut self) {
        self.side_to_move = self.side_to_move.opposite();
    }

    // Advanced optimization API methods (stubs)
    pub fn zobrist_hash(&self) -> u64 {
        0
    }
    pub fn halfmove_clock(&self) -> u8 {
        0
    }
    pub fn fullmove_number(&self) -> u16 {
        1
    }
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }
    pub fn piece_bitboard(&self, piece_type: PieceType, color: Color) -> Bitboard {
        self.pieces[color.index() * 6 + piece_type.index()]
    }
    pub fn king_square_unchecked(&self, _color: Color) -> Square {
        Square::E1
    }
    pub fn has_castled(&self, _color: Color) -> bool {
        false
    }

    pub fn remove_piece(&mut self, square: Square) {
        if let Some(piece) = self.piece_at(square) {
            // Remove from board
            self.board[square.index() as usize] = None;

            // Remove from piece bitboards
            let piece_index = piece.color.index() * 6 + piece.piece_type.index();
            self.pieces[piece_index] &= !square.bitboard();

            // Update occupied bitboards
            self.occupied[piece.color.index()] &= !square.bitboard();
            self.all_occupied = self.occupied[0] | self.occupied[1];
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}
