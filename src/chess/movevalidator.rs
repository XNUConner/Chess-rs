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
        assert!(src != dst);

        if let Some(piece) = chess.get_piece_at_square(src) {

            let name = Chess::get_name_for_piece(piece);
            
            return match name {
                PieceName::PAWN   => Self::validate_pawn_move(src, dst, chess),
                PieceName::KNIGHT => Self::validate_knight_move(src, dst, chess),
                PieceName::BISHOP => Self::validate_bishop_move(src, dst, chess),
                PieceName::ROOK   => Self::validate_rook_move(src, dst, chess),
                PieceName::QUEEN  => Self::validate_rook_move(src, dst, chess) || Self::validate_bishop_move(src, dst, chess),
                PieceName::KING   => Self::validate_king_move(src, dst, chess),
            };
        }

        false
    }

    fn move_difference(src: Square, dst: Square) -> i32 {
        dst as i32 - src as i32
    }

}