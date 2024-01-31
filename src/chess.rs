mod piececolor;
mod piecename;
mod movevalidator;

pub use crate::chess::piececolor::PieceColor;
pub use crate::chess::piecename::PieceName;
pub use crate::chess::movevalidator::MoveValidator;

type Piece  = u8;
type Square = usize;

pub struct Chess {
    board: [Option<Piece>; 64],
    turn: PieceColor,
}

impl Chess {
    pub fn new(fen: &str) -> Self {
        let mut chess = Chess {
            board: [None; 64],
            turn: PieceColor::WHITE,
        };

        chess.load_fen(fen);

        chess
    }

    pub fn load_fen(&mut self, fen: &str) {
        assert!( fen.is_ascii() );

        self.clear_board();

        let mut square: Square = 0;
        for ch in fen.chars() {

            // Skip '/'
            if ch == '/' { continue; }

            assert!( ch.is_ascii_alphanumeric() );

            // For digits, increment board_ptr by digit, continue
            if let Some(digit) = ch.to_digit(/* Radix */ 10) {
                assert!(digit > 0 && digit <= 8);
                square += digit as Square;
                continue;
            }

            // For chars setup the corresponding black or white piece at that square
            assert!( ch.is_ascii_alphabetic() );

            let is_black: bool = ch.is_ascii_lowercase();

            let color = match is_black {
                true  => PieceColor::BLACK as u8,
                false => PieceColor::WHITE as u8,
            };

            let name = match ch.to_ascii_lowercase() {
                'p' => PieceName::PAWN   as u8,
                'r' => PieceName::ROOK   as u8,
                'n' => PieceName::KNIGHT as u8,
                'b' => PieceName::BISHOP as u8,
                'q' => PieceName::QUEEN  as u8,
                'k' => PieceName::KING   as u8,
                 _  => panic!("Invalid character in FEN string."),
            };

            let piece = name | color;
            self.set_piece_at_square(piece);
            square += 1;

        }

    }

    pub fn clear_board(&mut self) {
        self.board = [None; 64];
    }

    pub fn attempt_move(&mut self, src: Square, dst: Square) {
        assert!(src >= 0 && src < 64);
        assert!(dst >= 0 && dst < 64);

        if let Some(piece) = self.get_piece_at_square(src) {

            if Self::is_turn_for_piece(piece) {
                if MoveValidator::validate_move(src, dst, &self.board) == true {
                    self.move_piece(src, dst);
                }
            }
        }
    }

    pub fn board(&self) -> &[u8] {
        &self.board
    }

    pub fn get_turn(&self) -> PieceColor {
        self.turn
    }

    fn next_turn(&mut self) {
        self.turn = match self.turn {
            PieceColor::BLACK => PieceColor::WHITE,
            PieceColor::WHITE => PieceColor::BLACK,
        };
    }

    pub fn get_resulting_square_for_move(src: Square, mov: i32) -> Square {
        let dst = src as i32 + mov;
        assert!(dst >= 0 && dst <= 63);
        return dst as u8;
    }

    pub fn is_square_empty(&self, square: Square) -> bool {
        self.board[square].is_none()
    }

    pub fn get_piece_at_square(&self, square: Square) -> Option<Piece> {
        match is_square_empty(square) {
            true  => None,
            false => Some( self.board[square] ),
        }
    }

    pub fn get_color_for_piece(piece: Piece) -> PieceColor {
        assert!(piece > 64);
        match piece > 128 {
            true  => PieceColor::BLACK,
            false => PieceColor::WHITE,
        }
    }

    pub fn get_name_for_piece(piece: Piece) -> PieceName {

        let val = match Self::get_color_for_piece(piece) {
            PieceColor::BLACK => piece - 128,
            PieceColor::WHITE => piece - 64,
        };

        PieceName::try_from(val).unwrap()
    }

    fn set_piece_at_square(&mut self, piece: Piece, square: Square) {
        self.board[square] = Some(piece);
    }

    fn remove_piece_at_square(&mut self, square: Square) {
        self.board[square] = None;
    }

    pub fn is_turn_for_piece(&self, piece: Piece) -> bool {
        Self::get_color_for_piece(piece) == self.get_turn()
    }

    fn move_piece(&mut self, src: Square, dst: Square) {
        let piece = self.get_piece_at_square(src).unwrap();
        assert!(self.get_turn() == Self::get_color_for_piece(piece));
        
        self.set_piece_at_square(piece, dst);
        self.remove_piece_at_square(src);

        self.next_turn();
    }

}