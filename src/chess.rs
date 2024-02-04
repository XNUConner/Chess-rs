mod piececolor;
mod piecename;
mod movevalidator;

pub use crate::chess::piececolor::PieceColor;
pub use crate::chess::piecename::PieceName;
pub use crate::chess::movevalidator::MoveValidator;

type Piece  = u8;
type Square = usize;

#[derive(Clone)]
pub struct Chess {
    board: [Option<Piece>; 64],
    turn: PieceColor,
    white_king_square: Square,
    black_king_square: Square,
    white_can_castle: bool,
    black_can_castle: bool,
    move_history: Vec<(Square, Square)>,
}

impl Chess {
    pub fn new(fen: &str) -> Self {
        let mut chess = Chess {
            board: [None; 64],
            turn: PieceColor::WHITE,
            white_king_square: 64,
            black_king_square: 64,
            white_can_castle: false,
            black_can_castle: false,
            move_history: Vec::new(),
        };

        chess.load_fen(fen);

        assert!(chess.white_king_square != 64);
        assert!(chess.black_king_square != 64);

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
            self.set_piece_at_square(square, piece);

            if Chess::get_name_for_piece(piece) == PieceName::KING {
                self.update_king_square(square, Chess::get_color_for_piece(piece));
            }

            square += 1;

        }

    }

    pub fn update_king_square(&mut self, square: Square, color: PieceColor) {
        assert!(square < 64);
        match color {
            PieceColor::BLACK => self.black_king_square = square,
            PieceColor::WHITE => self.white_king_square = square,
        };
    }

    pub fn clear_board(&mut self) {
        self.board = [None; 64];
        self.white_king_square = 64;
        self.black_king_square = 64;
    }

    pub fn attempt_move(&mut self, src: Square, dst: Square) {
        assert!(src < 64);
        assert!(dst < 64);

        if let Some(piece) = self.get_piece_at_square(src) {

            if self.is_turn_for_piece(piece) {
                if MoveValidator::validate_move(src, dst, &self) == true {

                    // Make move if it is validated (Does not process checks)
                    self.move_piece(src, dst);

                    // Check for checks on same-color king
                    let king_square = {
                        let piece_color = Self::get_color_for_piece(piece);
                        self.get_king_square(piece_color)
                    };

                    if MoveValidator::is_king_in_check(king_square, &self) {
                        self.undo_last_move();
                    } else {
                        // Update position of king if it has moved.
                        if PieceName::KING == Chess::get_name_for_piece(piece) {
                            let king_color = Chess::get_color_for_piece(piece);
                            self.update_king_square(dst, king_color);
                            
                            match king_color {
                                PieceColor::WHITE => { self.white_can_castle = false; },
                                PieceColor::BLACK => { self.black_can_castle = false; },
                            };
                        }
                    }
                    
                }
            }
        }
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
        assert!(dst < 64);
        dst as Square
    }

    pub fn is_square_empty(&self, square: Square) -> bool {
        self.board[square].is_none()
    }

    pub fn get_piece_at_square(&self, square: Square) -> Option<Piece> {
        match self.is_square_empty(square) {
            true  => None,
            false => self.board[square],
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

    fn set_piece_at_square(&mut self, square: Square, piece: Piece) {
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
        
        self.set_piece_at_square(dst, piece);
        self.remove_piece_at_square(src);

        self.move_history.push( (src, dst) );

        self.next_turn();
    }

    // Used when a move has put a color's own king in check
    fn undo_last_move(&mut self) {
        assert!(!self.move_history.is_empty());

        let (src, dst) = self.move_history.pop().unwrap();
        self.next_turn();
        self.move_piece(dst, src);

    }

    pub fn get_king_square(&self, color: PieceColor) -> Square {
        match color {
            PieceColor::WHITE => self.white_king_square,
            PieceColor::BLACK => self.black_king_square,
        }
    }

}