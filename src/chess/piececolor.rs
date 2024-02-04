#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
#[repr(u8)]
pub enum PieceColor {
    WHITE = 64,
    BLACK = 128,
}

impl TryFrom<u8> for PieceColor {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
                64  => Ok(PieceColor::WHITE),
                128 => Ok(PieceColor::BLACK),
                _  => Err("Invalid u8 provided during PieceName conversion"),
        }
    }
}