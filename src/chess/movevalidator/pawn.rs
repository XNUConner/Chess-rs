use crate::chess::MoveValidator;
use crate::chess::Chess;
use crate::chess::{PieceName, PieceColor};
// Need previous move for en-passant?
// Chess object could store a vector of (src, dst) tuples as move_history
// And the last entry could be passed to MoveVerifier
impl MoveValidator {


    pub fn validate_pawn_move(src: Square, dst: Square, chess: &Chess) -> bool {
        assert!(src >= 0 && dst <= 63);
        assert!(dst >= 0 && dst <= 63);

        let mov = Self::move_difference(src, dst);

        // Verify attempted move is heading in the valid direction for pawn.
        // Self::check_pawn_direction(pawn, move) is needed
        let pawn = chess.get_piece_at_square(src).unwrap();
        let pawn_color = Chess::get_color_for_piece(pawn);
        let check_direction = match pawn_color {
            // Black's attempted move must be > 0 (Heading downwards on the board).
            PieceColor::BLACK => mov > 0,
            // White must head upwards.
            PieceColor::WHITE => mov < 0,
        };

        if !check_direction { return false; }

        match mov.abs() {
            7 | 9  => Self::verify_pawn_move_diagonal(src, dst, chess),
            8  => Self::verify_pawn_move_forward(src, dst, chess),
            16 => Self::verify_pawn_move_forward_twice(src, dst, chess),
            _ => false,
        }
    }

    fn verify_pawn_move_forward(src: Square, dst: Square, chess: &Chess) -> bool {
        // Must verify there is no piece on the square we are moving to.
        chess.is_square_empty(dst)
    }

    
    fn verify_pawn_move_diagonal(src: Square, dst: Square, chess: &Chess) -> bool {
        // Has to be a piece at the destination, and it must be an opposite color piece.
        let pawn = chess.get_piece_at_square(src).unwrap();
        let pawn_color = Chess::get_color_for_piece(pawn);
        
        // If there is a piece at destination square, and that piece must not be the same color as our pawn
        if let dst_piece = chess.get_piece_at_square(dst) {
            let dst_piece_color = Chess::get_color_for_piece(dst_piece);
            return pawn_color != piece_at_dst_color;
        }

        // Square is empty, so pawn cannot move diagonally.
        false
    }
    
    fn verify_pawn_move_forward_twice(src: Square, dst: Square, chess: &Chess) -> bool {

        // If black, piece must be on squares >= 48 && <= 55 
        // If white, piece must be on squares >=  8 && <= 15
        let piece = chess.get_piece_at_square(src).unwrap();
        let on_correct_row = match Chess::get_color_for_piece(piece) {
            PieceColor::BLACK => src >=  8 && src <= 15,
            PieceColor::WHITE => src >= 48 && src <= 55,
        };

        if !on_correct_row { return false; }

        // Verify no pieces obstructing path
        let one_square_forward = dst - 8;
        let two_squares_forward = dst;

        chess.is_square_empty(one_square_forward) && chess.is_square_empty(two_squares_forward)
    }
}