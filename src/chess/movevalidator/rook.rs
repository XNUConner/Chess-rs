use crate::chess::MoveValidator;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};

// Possible moves:
// x % -8 == 0
// x %  8 == 0
// x % -1 == 0
// x %  1 == 0
// if destination does not contain a same-color piece

impl MoveValidator {
    pub fn validate_rook_move(src: Square, dst: Square, chess: &Chess) -> bool {

        let mov = Self::move_difference(src, dst);

        if ( mov.abs() % 8 != 0 && mov.abs() % 1 != 0 )  { return false; };

        let rook = chess.get_piece_at_square(src);
        let rook_color = Chess::get_color_for_piece(rook);


        let offset: i32 = if mov.abs() % 8 == 0 {
            if mov < 0 { -8 } else { 8 }
        } else {
            if mov < 0 { -1 } else { 1 }
        };

        // Make a MoveValidator::in_same_row(src, dst)
        // Make a MoveValidator::in_same_column(src, dst)
        // Make a MoveValidator::cmp_piece_colors(piece, piece)
        // When moving horizontally, rook can overshoot it's row and end up "moving horizontally" through multiple rows if unobstructed
        if offset.abs() == 1 {
            // src and dst must be in the same row
            if src / 8  != dst / 8 { return false; }
        }

        // If there is any piece in the WAY on the path to the destination square, return false.
        // Make a MoveVerifier::is_piece_in_path(src, dst, mov)
        let mut square = (src as i32 + offset) as Square;
        while square != dst {
            // Check if piece is present at square
            if !chess.is_square_empty(square) { return false }
            square += offset;
        }

        // If there is a piece AT the destination square, that is the same color as our bishop, return false.
        if let dst_piece = chess.get_piece_at_square(dst) {
            let rook = chess.get_piece_at_square(src).unwrap();
            let src_rook_color = Chess::get_color_for_piece(rook);
            let dst_piece_color = Chess::get_color_for_piece(dst_piece)

            return src_rook_color != dst_piece_color
        }

        // Otherwise we take the opposite-color piece.
        // Or move to that empty square.
        true
    }
}