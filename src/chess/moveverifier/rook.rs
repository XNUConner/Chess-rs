use crate::chess::MoveVerifier;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};

// Possible moves:
// x % -8 == 0
// x %  8 == 0
// x % -1 == 0
// x %  1 == 0
// if destination does not contain a same-color piece

impl MoveVerifier {
    pub fn verify_rook_move(src: u8, dst: u8, board: &[u8]) -> bool {

        let attempted_move = dst as i32 - src as i32;

        if ( attempted_move.abs() % 8 != 0 && attempted_move.abs() % 1 != 0 )  { return false; };

        let rook_color = Chess::get_color_for_piece(board[src as usize]);


        let offset: i32 = if attempted_move.abs() % 8 == 0 {
            match attempted_move < 0 {
                true  => -8,
                false =>  8,
            }
        } else {
            // When moving horizontally, rook can overshoot it's row and end up "moving horizontally" through multiple rows if unobstructed
            match attempted_move < 0 {
                true  => -1,
                false =>  1,
            }
        };

        if offset.abs() == 1 {
            // src and dst must be in the same row
            if src / 8  != dst / 8 { return false; }
        }

        // If there is any piece in the WAY on the path to the destination square, return false.
        let mut square = src as i32 + offset; // cant be attempted_move, must be -7, 7, -9, or 9
        while square != (dst as i32) {
            // Check if piece is present at square
            if Self::is_piece_at(square.try_into().unwrap(), board) { return false; }
            square += offset;
        }

        // If there is a piece AT the destination square, that is the same color as our bishop, return false.
        if Self::is_piece_at(dst, board) && Chess::get_color_for_piece(board[dst as usize]) == rook_color { return false; }

        true
    }
}