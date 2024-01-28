use crate::chess::MoveVerifier;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};

// Possible moves:
// -10, -17, +10, +17
// -15, -6,  +15, +6
// if destination does not contain a same-color piece

impl MoveVerifier {
    pub fn verify_knight_move(src: u8, dst: u8, board: &[u8]) -> bool {

        let attempted_move = dst as i32 - src as i32;

        let is_valid_square = match attempted_move.abs() {
            6 | 10 | 15 | 17 => true,
            _ => false,
        };

        if !is_valid_square { return false; }

        let knight = board[src as usize];
        let knight_color = Chess::get_color_for_piece(knight);

        let dst_square = Self::get_dest(src, attempted_move);

        // We can jump there if the square is empty
        if !Self::is_piece_at(dst_square, board) { return true; }

        // We cannot jump there if a same-color piece is at that square.
        let piece_at_dst = board[dst_square as usize];
        let piece_at_dst_color = Chess::get_color_for_piece(piece_at_dst);

        if piece_at_dst_color == knight_color { return false; }


        true
    }
}