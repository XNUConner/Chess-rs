use crate::chess::MoveValidator;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};
use crate::chess::Square;

// Possible moves:
// -9 -8 -7
// -1  #  1
//  7  8  9
// if destination does not contain a same-color piece

impl MoveValidator {
    pub fn validate_king_move(src: Square, dst: Square, chess: &Chess) -> bool {

        let mov = Self::move_difference(src, dst);

        if ![1, 7, 8, 9].contains( &mov.abs() ) { return false; }

        let king = chess.get_piece_at_square(src).unwrap();
        let king_color = Chess::get_color_for_piece(king);

        // We can move there if the square is empty
        if chess.is_square_empty(dst) { return true; }

        // We cannot move there if a same-color piece is at that square.
        let dst_piece = chess.get_piece_at_square(dst).unwrap();
        let dst_piece_color = Chess::get_color_for_piece(dst_piece);

        dst_piece_color != king_color

    }

    pub fn is_king_in_check(king_square: Square, chess: &Chess) -> bool {
        // search from the king in every direction, is there an enemy piece there? run validate_move(enemy_piece_square, king_square). If true, return true.
        // This search pattern won't find enemy knights.
        // ^ You could find knights by searching -15, 15, 6, -6, etc FROM the king_square
        // -9 -8 -7
        // -1  #  1
        //  7  8  9

        // Iterate over board, is piece opposite color? run validate_move(enemy_piece_square, king_square). If true, return true.
        let king = chess.get_piece_at_square(king_square).unwrap();
        let king_color = Chess::get_color_for_piece(king);
        for square in 0..=63 {

            if let Some(piece) = chess.get_piece_at_square(square) {
                if king_color != Chess::get_color_for_piece(piece) {

                    let takes_king = match Chess::get_name_for_piece(piece) {
                        PieceName::PAWN   => Self::validate_pawn_move(square, king_square, chess),
                        PieceName::KNIGHT => Self::validate_knight_move(square, king_square, chess),
                        PieceName::BISHOP => Self::validate_bishop_move(square, king_square, chess),
                        PieceName::ROOK   => Self::validate_rook_move(square, king_square, chess),
                        PieceName::QUEEN  => Self::validate_rook_move(square, king_square, chess) || Self::validate_bishop_move(square, king_square, chess),
                        PieceName::KING   => Self::validate_king_move(square, king_square, chess),
                    };

                    if takes_king { return true; }

                }
            }

        }

        false
    }
}