use crate::chess::Chess;
use crate::chess::PieceColor;
use crate::chess::PieceName;

mod pawn;
mod knight;
mod bishop;
mod rook;
mod king;

pub struct MoveVerifier {

}

impl MoveVerifier {
    pub fn verify_move(src: u8, dst: u8, board: &[u8]) -> bool {
        // Sanity check
        assert!(src >= 0 && dst <= 63);
        assert!(dst >= 0 && dst <= 63);

        // Cannot move a piece onto itself.
        if src == dst { return false; }

        let piece = board[src as usize];

        // Having the source square be an empty square should be checked before this function is called.
        assert!(piece != 0);

        let piecename = Chess::get_name_for_piece(piece);
        
        // Execute rules check depending on type of piece
        match piecename {
            PieceName::PAWN   => Self::verify_pawn_move(src, dst, board),
            PieceName::KNIGHT => Self::verify_knight_move(src, dst, board),
            PieceName::BISHOP => Self::verify_bishop_move(src, dst, board),
            PieceName::ROOK   => Self::verify_rook_move(src, dst, board),
            PieceName::QUEEN  => Self::verify_rook_move(src, dst, board) || Self::verify_bishop_move(src, dst, board),
            PieceName::KING   => Self::verify_king_move(src, dst, board),
            _ => false,
        }
    }

    // Get destination square, perhaps a better name would be get_dst_square()
    fn get_dest(src: u8, mov: i32) -> u8 {
        let dst = src as i32 + mov;
        assert!(dst >= 0 && dst <= 63);
        return dst as u8;
    }

    fn is_piece_at(dst: u8, board: &[u8]) -> bool {
        board[dst as usize] != 0
    }

    // Need to implement
    fn get_piece_at() {

    }
}