//! Game Board Logic

/// size of game board

const SIZE: usize = 9;

/// stores game board infoormation
pub struct Gameboard {
    /// Stores the contents of a cell
    /// 0 is an empty cell
    pub cells: [[u8; SIZE]; SIZE],
}

impl Gameboard {
    /// creates a new gameboard
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
        }
    }
}