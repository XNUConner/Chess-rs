use crate::chess::MoveValidator;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};

// Possible moves:
// -9 -8 -7
// -1  #  1
//  7  8  9
// if destination does not contain a same-color piece

impl MoveValidator {
    pub fn validate_king_move(src: Square, dst: Square, chess: &Chess) -> bool {

        let mov = move_difference(src, dst);

        if ![1, 7, 8, 9].contains( mov.abs() ) { return false; }

        let king = chess.get_piece_at_square(src).unwrap();
        let king_color = Chess::get_color_for_piece(king);

        // We can move there if the square is empty
        if chess.is_square_empty(dst) { return true; }

        // We cannot move there if a same-color piece is at that square.
        let dst_piece = chess.get_piece_at_square(dst).unwrap();
        let dst_piece_color = Chess::get_color_for_piece(piece_at_dst);

        dst_piece_color != king_color

    }
}