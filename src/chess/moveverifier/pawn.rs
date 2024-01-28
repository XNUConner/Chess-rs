use crate::chess::MoveVerifier;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};
// Need previous move for en-passant?
// Chess object could store a vector of (src, dst) tuples as move_history
// And the last entry could be passed to MoveVerifier
impl MoveVerifier {


    pub fn verify_pawn_move(src: u8, dst: u8, board: &[u8]) -> bool {
        assert!(src >= 0 && dst <= 63);
        assert!(dst >= 0 && dst <= 63);

        let pawn = board[src as usize];
        let pawn_color = Chess::get_color_for_piece(pawn);

        let attempted_move = dst as i32 - src as i32;

        // Verify attempted move is heading in the valid direction for pawn.
        let check_direction = match pawn_color {
            // Black's attempted move must be > 0 (Heading downwards on the board).
            PieceColor::BLACK => attempted_move > 0,
            // White must head upwards.
            PieceColor::WHITE => attempted_move < 0,
        };

        if !check_direction { return false; }

        match attempted_move.abs() {
            7 | 9  => Self::verify_pawn_move_diagonal(src, attempted_move, board),
            8  => Self::verify_pawn_move_forward(src, attempted_move, board),
            16 => Self::verify_pawn_move_forward_twice(src, attempted_move, board),
            _ => false,
        }
    }

    fn verify_pawn_move_forward(src: u8, mov: i32, board: &[u8]) -> bool {
        // Must verify there is no piece on the square we are moving to.
        !Self::is_piece_at( Self::get_dest(src, mov), board )
    }

    
    fn verify_pawn_move_diagonal(src: u8, mov: i32, board: &[u8]) -> bool {
        // Has to be a piece at the destination, and it must be an opposite color piece.
        let pawn = board[src as usize];
        let pawn_color = Chess::get_color_for_piece(pawn);
        
        // must be piece at destination square, and that piece must not be the same color as our pawn
        let dst = Self::get_dest(src, mov);
        if !Self::is_piece_at(dst, board) { return false; }

        let piece_at_dst_color = Chess::get_color_for_piece(board[dst as usize]);

        if pawn_color == piece_at_dst_color { return false; }

        true
        
    }
    
    fn verify_pawn_move_forward_twice(src: u8, attempted_move: i32, board: &[u8]) -> bool {
        let piece = board[src as usize];

        // If black, piece must be on squares >= 48 && <= 55 
        // If white, piece must be on squares >=  8 && <= 15
        let on_correct_row = match Chess::get_color_for_piece(piece) {
            PieceColor::BLACK => src >=  8 && src <= 15,
            PieceColor::WHITE => src >= 48 && src <= 55,
        };

        if !on_correct_row { return false; }

        // Verify no pieces obstructing path
        let one_square_forward = Self::get_dest(src, attempted_move / 2);
        let two_squares_forward = Self::get_dest(src, attempted_move);
        if Self::is_piece_at(one_square_forward, board) || Self::is_piece_at(two_squares_forward, board) { return false; }

        true
    }
}