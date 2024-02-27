use super::piece::Piece;

struct Board {
    squares: [Square; 64],
    movement_history: Vec<Movement>
}

impl Board {

    pub fn new() -> Self {
        Board {
            squares: [Square::new(); 64],
            movement_history: Vec::new(),
        }
    }

    pub fn place_piece_at_square(piece: Piece, square: usize) {
        self.squares[square].piece = Some(piece);
    }

    pub fn move_piece_from_square_to(from: usize, to: usize) {

    }

}

struct Square {
    piece: Option<Piece>
    attacked_by_white: bool,
    attacked_by_black: bool,
}

impl Square {

    fn new() -> Self {
        piece: None,
        attacked_by_white: false,
        attacked_by_black: false,
    }

}

struct Movement {
    piece: Piece,
    to:   usize, // Location in squares array
    from: usize, // Location in squares array
}

