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

    /// Gets the character at cell location
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]] {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _=> return None,
        })
    }

    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        self.cells[ind[1]][ind[0]] = val;
    }
}