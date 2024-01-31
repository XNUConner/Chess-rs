use crate::chess::MoveValidator;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};
use crate::chess::Square;

// Possible moves:
// x % -9 == 0
// x %  9 == 0
// x % -7 == 0
// x %  7 == 0
// if destination does not contain a same-color piece

impl MoveValidator {
    pub fn validate_bishop_move(src: Square, dst: Square, chess: &Chess) -> bool {

        let mov = Self::move_difference(src, dst);

        if mov.abs() % 9 != 0 && mov.abs() % 7 != 0  { return false; };

        let offset: i32 = if mov.abs() % 9 == 0 {
            if mov < 0 { -9 } else { 9 }
        } else {
            if mov < 0 { -7 } else { 7 }  
        };

        // If there is any piece in the WAY on the path to the destination square, return false.
        let mut square = (src as i32 + offset);
        while square != dst as i32 {
            // Check if piece is present at square
            if !chess.is_square_empty(square as Square) { return false; }
            square += offset;
        }

        // If there is a piece AT the destination square, that is the same color as our bishop, return false.
        if let Some(dst_piece) = chess.get_piece_at_square(dst) {
            let bishop = chess.get_piece_at_square(src).unwrap();
            let src_bishop_color = Chess::get_color_for_piece(bishop);
            let dst_piece_color = Chess::get_color_for_piece(dst_piece);
            
            return src_bishop_color != dst_piece_color;
        }

        // Otherwise, we can take that opposite-color piece.
        // Or move to that empty square.
        true
    }
}