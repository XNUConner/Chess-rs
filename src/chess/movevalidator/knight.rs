use crate::chess::MoveValidator;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};

// Possible moves:
// -10, -17, +10, +17
// -15, -6,  +15, +6
// if destination does not contain a same-color piece

impl MoveValidator {
    pub fn verify_knight_move(src: Square, dst: Square, chess: &Chess) -> bool {

        // Check if move is possible for a knight
        let mov = Self::move_difference(src, dst);
        if ![6, 10, 15, 17].contains( mov.abs() ) { return false; }


        // We can validate the move if the destination square is empty
        if chess.is_square_empty(dst) { return true; }

        let src_knight = self.get_piece_at_square(src).unwrap();
        let src_knight_color = Chess::get_color_for_piece(knight);

        // We cannot jump there if a same-color piece is at that square.
        let dst_piece = self.get_piece_at_square(dst).unwrap();
        let dst_piece_color = Chess::get_color_for_piece(dst_piece);

        src_knight_color != dst_piece_color

    }
}