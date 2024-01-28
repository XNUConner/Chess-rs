#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[repr(u8)]
pub enum PieceName {
    PAWN = 1,
    ROOK = 2,
    KNIGHT = 4,
    BISHOP = 8,
    QUEEN = 16,
    KING = 32,
}

impl TryFrom<u8> for PieceName {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
                1  => Ok(Self::PAWN),
                2  => Ok(Self::ROOK),
                4  => Ok(Self::KNIGHT),
                8  => Ok(Self::BISHOP),
                16 => Ok(Self::QUEEN),
                32 => Ok(Self::KING),
                _  => Err("Invalid u8 provided during PieceName conversion"),
        }
    }
}

