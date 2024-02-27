struct Piece {
    name: Name,
    color: Color,
}

impl Piece {
    fn new(name: Name, color: Color) {
        Piece {
            name,
            color,
        }
    }
}

enum Name {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

enum Color {
    White,
    Black,
}