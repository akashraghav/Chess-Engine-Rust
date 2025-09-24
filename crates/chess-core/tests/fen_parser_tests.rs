#[cfg(test)]
mod fen_parser_tests {
    use chess_core::{Position, PieceType, Color, Square};

    #[test]
    fn test_starting_position_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let position = Position::from_fen(fen).expect("Should parse starting position");
        
        // Verify piece counts
        assert_eq!(position.pieces_of_type(PieceType::Pawn, Color::White).count_bits(), 8);
        assert_eq!(position.pieces_of_type(PieceType::Pawn, Color::Black).count_bits(), 8);
        assert_eq!(position.pieces_of_type(PieceType::Rook, Color::White).count_bits(), 2);
        assert_eq!(position.pieces_of_type(PieceType::Rook, Color::Black).count_bits(), 2);
        assert_eq!(position.pieces_of_type(PieceType::Queen, Color::White).count_bits(), 1);
        assert_eq!(position.pieces_of_type(PieceType::Queen, Color::Black).count_bits(), 1);
        assert_eq!(position.pieces_of_type(PieceType::King, Color::White).count_bits(), 1);
        assert_eq!(position.pieces_of_type(PieceType::King, Color::Black).count_bits(), 1);
        
        // Verify side to move
        assert_eq!(position.side_to_move, Color::White);
    }

    #[test]
    fn test_empty_board_fen() {
        let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
        let position = Position::from_fen(fen).expect("Should parse empty board");
        
        // All piece counts should be 0
        for piece_type in PieceType::ALL {
            assert_eq!(position.pieces_of_type(piece_type, Color::White).count_bits(), 0);
            assert_eq!(position.pieces_of_type(piece_type, Color::Black).count_bits(), 0);
        }
    }

    #[test]
    fn test_kings_only_fen() {
        let fen = "4k3/8/8/8/8/8/8/4K3 w - - 0 1";
        let position = Position::from_fen(fen).expect("Should parse kings only");
        
        // Only kings should be present
        assert_eq!(position.pieces_of_type(PieceType::King, Color::White).count_bits(), 1);
        assert_eq!(position.pieces_of_type(PieceType::King, Color::Black).count_bits(), 1);
        assert_eq!(position.pieces_of_type(PieceType::Queen, Color::White).count_bits(), 0);
        assert_eq!(position.pieces_of_type(PieceType::Queen, Color::Black).count_bits(), 0);
    }

    #[test]
    fn test_material_imbalance_fen() {
        // White has extra queen
        let fen = "3qk3/8/8/8/8/8/8/3QK3 w - - 0 1";
        let position = Position::from_fen(fen).expect("Should parse material imbalance");
        
        assert_eq!(position.pieces_of_type(PieceType::Queen, Color::White).count_bits(), 1);
        assert_eq!(position.pieces_of_type(PieceType::Queen, Color::Black).count_bits(), 1);
    }

    #[test]
    fn test_side_to_move_black() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        let position = Position::from_fen(fen).expect("Should parse with black to move");
        assert_eq!(position.side_to_move, Color::Black);
    }

    #[test]
    fn test_complex_position() {
        // Sicilian Defense position
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
        let position = Position::from_fen(fen).expect("Should parse complex position");
        
        // Verify specific pieces
        assert_eq!(position.pieces_of_type(PieceType::Pawn, Color::White).count_bits(), 8);
        assert_eq!(position.pieces_of_type(PieceType::Pawn, Color::Black).count_bits(), 8);
        assert_eq!(position.side_to_move, Color::White);
    }

    #[test]
    fn test_invalid_fen_empty_string() {
        let result = Position::from_fen("");
        assert!(result.is_err(), "Empty FEN should fail");
    }

    #[test]
    fn test_invalid_fen_wrong_rank_count() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"; // Missing one rank
        let result = Position::from_fen(fen);
        assert!(result.is_err(), "Wrong rank count should fail");
    }

    #[test]
    fn test_invalid_fen_wrong_file_count() {
        let fen = "rnbqkbnrr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"; // 9 pieces in first rank
        let result = Position::from_fen(fen);
        assert!(result.is_err(), "Wrong file count should fail");
    }

    #[test]
    fn test_invalid_fen_bad_piece_character() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBXR w KQkq - 0 1"; // 'X' is not a valid piece
        let result = Position::from_fen(fen);
        assert!(result.is_err(), "Invalid piece character should fail");
    }

    #[test]
    fn test_invalid_fen_bad_side_to_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1"; // 'x' is not valid side
        let result = Position::from_fen(fen);
        assert!(result.is_err(), "Invalid side to move should fail");
    }

    #[test]
    fn test_fen_with_promoted_pieces() {
        // Position with promoted queen on 8th rank
        let fen = "rnbqkbQr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let position = Position::from_fen(fen).expect("Should parse promoted pieces");
        assert_eq!(position.pieces_of_type(PieceType::Queen, Color::White).count_bits(), 2); // Original + promoted
    }

    #[test]
    fn test_fen_piece_placement_accuracy() {
        // Test specific piece placement
        let fen = "8/8/8/8/8/8/8/R3K2R w KQkq - 0 1"; // Rooks on a1 and h1, King on e1
        let position = Position::from_fen(fen).expect("Should parse piece placement");
        
        // Check specific squares
        let a1 = Square::new(0).unwrap(); // a1 = index 0
        let e1 = Square::new(4).unwrap(); // e1 = index 4  
        let h1 = Square::new(7).unwrap(); // h1 = index 7
        
        assert!(matches!(position.piece_at(a1), Some(piece) if piece.piece_type == PieceType::Rook && piece.color == Color::White));
        assert!(matches!(position.piece_at(e1), Some(piece) if piece.piece_type == PieceType::King && piece.color == Color::White));
        assert!(matches!(position.piece_at(h1), Some(piece) if piece.piece_type == PieceType::Rook && piece.color == Color::White));
    }
}