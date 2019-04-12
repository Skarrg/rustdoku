//! Gamebooard controller

use piston::input::GenericEvent;

use crate::Gameboard;

/// handles events for the sudoku game
pub struct GameboardController {
    ///stores the gameboard state
    pub gameboard: Gameboard,
}

impl GameboardController {
    /// creates a new gameboard controller
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
        }
    }

    /// handles events
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        
    }
}