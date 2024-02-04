use crate::chess::Chess;
use crate::chess::PieceColor;
use crate::chess::PieceName;
use crate::chess::Square;

mod pawn;
mod knight;
mod bishop;
mod rook;
mod king;

pub struct MoveValidator {

}

impl MoveValidator {
    pub fn validate_move(src: Square, dst: Square, chess: &Chess) -> bool {
        // Sanity check. UI should prevent these scenarios.
        assert!(src >= 0 && dst <= 63);
        assert!(dst >= 0 && dst <= 63);


        if src == dst { return false; }

        if let Some(piece) = chess.get_piece_at_square(src) {

            let name  = Chess::get_name_for_piece(piece);
            
            let is_valid_move = match name {
                PieceName::PAWN   => Self::validate_pawn_move(src, dst, chess),
                PieceName::KNIGHT => Self::validate_knight_move(src, dst, chess),
                PieceName::BISHOP => Self::validate_bishop_move(src, dst, chess),
                PieceName::ROOK   => Self::validate_rook_move(src, dst, chess),
                PieceName::QUEEN  => Self::validate_rook_move(src, dst, chess) || Self::validate_bishop_move(src, dst, chess),
                PieceName::KING   => Self::validate_king_move(src, dst, chess),
            };

            if !is_valid_move { return false; }

            // Make a copy of the chess board, make the above move, then check if the same-color king is in check.
            // If king is in check, then move cannot be valid and thus return false, else true
            let mut hypothetical_chess = (*chess).clone();
            //println!("Turn for chess: {}", chess.get_turn());
            //println!("Turn for hypothetical: {}", hypothetical_chess.get_turn())
            let color_of_piece_being_moved = Chess::get_color_for_piece(piece);
            let king_square = match name {
                PieceName::KING => dst,
                _ => hypothetical_chess.get_king_square(color_of_piece_being_moved),
            };
            hypothetical_chess.move_piece(src, dst);
            
            return !Self::is_king_in_check(king_square, &hypothetical_chess);
        }

        // If there is no piece at src square, return false.
        false
    }

    fn move_difference(src: Square, dst: Square) -> i32 {
        dst as i32 - src as i32
    }

}