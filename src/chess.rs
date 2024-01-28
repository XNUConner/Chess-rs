mod piececolor;
mod piecename;
mod moveverifier;

pub use crate::chess::piececolor::PieceColor;
pub use crate::chess::piecename::PieceName;
pub use crate::chess::moveverifier::MoveVerifier;

pub struct Chess {
    board: [u8; 64],
}

impl Chess {
    pub fn new(fen: &str) -> Self {
        let mut chess = Chess {
            board: [0; 64],
        };

        chess.load_fen(fen);

        chess
    }

    pub fn load_fen(&mut self, fen: &str) {
        assert!( fen.is_ascii() );

        let mut board_ptr: usize = 0;
        for ch in fen.chars() {

            // Skip '/'
            if ch == '/' { continue; }

            assert!( ch.is_ascii_alphanumeric() );

            // For digits, increment board_ptr by digit, continue
            if let Some(digit) = ch.to_digit(/* Radix */ 10) {
                assert!(digit > 0 && digit <= 8);
                board_ptr += digit as usize;
                continue;
            }

            // For chars setup the corresponding black or white piece at that square
            assert!( ch.is_ascii_alphabetic() );

            let is_black: bool = ch.is_ascii_lowercase();

            let mut piece: u8 = match is_black {
                true  => PieceColor::BLACK as u8,
                false => PieceColor::WHITE as u8,
            };

            piece |= match ch.to_ascii_lowercase() {
                'p' => PieceName::PAWN   as u8,
                'r' => PieceName::ROOK   as u8,
                'n' => PieceName::KNIGHT as u8,
                'b' => PieceName::BISHOP as u8,
                'q' => PieceName::QUEEN  as u8,
                'k' => PieceName::KING   as u8,
                 _  => panic!("Invalid character in FEN string."),
            };

            self.board[board_ptr] = piece;
            board_ptr += 1;

        }

    }

    pub fn attempt_move(&mut self, src: u8, dst: u8) {
        assert!(src >= 0 && src < 64);
        assert!(dst >= 0 && dst < 64);

        if MoveVerifier::verify_move(src, dst, &self.board) == true {
            let piece = self.board[src as usize];
            self.board[src as usize] = 0;
            self.board[dst as usize] = piece;
        }

    }

    pub fn get_color_for_piece(piece: u8) -> PieceColor {
        assert!(piece > 64);
        match piece > 128 {
            true  => PieceColor::BLACK,
            false => PieceColor::WHITE,
        }
    }

    pub fn get_name_for_piece(piece: u8) -> PieceName {

        let val = match Self::get_color_for_piece(piece) {
            PieceColor::BLACK => piece - 128,
            PieceColor::WHITE => piece - 64,
        };

        PieceName::try_from(val).unwrap()
    }

    pub fn board(&self) -> &[u8] {
        &self.board
    }
}