use crate::chess::MoveVerifier;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};

// Possible moves:
// -9 -8 -7
// -1  #  1
//  7  8  9
// if destination does not contain a same-color piece

impl MoveVerifier {
    pub fn verify_king_move(src: u8, dst: u8, board: &[u8]) -> bool {

        let attempted_move = dst as i32 - src as i32;

        let is_valid_square = match attempted_move.abs() {
            7 | 8 | 9 | 1 => true,
            _ => false,
        };

        if !is_valid_square { return false; }

        let king = board[src as usize];
        let king_color = Chess::get_color_for_piece(king);

        let dst_square = Self::get_dest(src, attempted_move);

        // We can move there if the square is empty
        if !Self::is_piece_at(dst_square, board) { return true; }

        // We cannot move there if a same-color piece is at that square.
        let piece_at_dst = board[dst_square as usize];
        let piece_at_dst_color = Chess::get_color_for_piece(piece_at_dst);

        if piece_at_dst_color == king_color { return false; }


        true
    }
}