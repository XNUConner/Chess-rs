use crate::chess::MoveVerifier;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};

// Possible moves:
// x % -9 == 0
// x %  9 == 0
// x % -7 == 0
// x %  7 == 0
// if destination does not contain a same-color piece

impl MoveVerifier {
    pub fn verify_bishop_move(src: u8, dst: u8, board: &[u8]) -> bool {

        let attempted_move = dst as i32 - src as i32;

        if ( attempted_move.abs() % 9 != 0 && attempted_move.abs() % 7 != 0 )  { return false; };

        let bishop_color = Chess::get_color_for_piece(board[src as usize]);

        let offset = if attempted_move.abs() % 9 == 0 {
            match attempted_move < 0 {
                true  => -9,
                false =>  9,
            }
        } else {
            match attempted_move < 0 {
                true  => -7,
                false =>  7,
            }
        };

        // If there is any piece in the WAY on the path to the destination square, return false.
        let mut square = src as i32 + offset; // cant be attempted_move, must be -7, 7, -9, or 9
        while square != (dst as i32) {
            // Check if piece is present at square
            if Self::is_piece_at(square.try_into().unwrap(), board) { return false; }
            square += offset;
        }

        // If there is a piece AT the destination square, that is the same color as our bishop, return false.
        if Self::is_piece_at(dst, board) && Chess::get_color_for_piece(board[dst as usize]) == bishop_color { return false; }

        true
    }
}