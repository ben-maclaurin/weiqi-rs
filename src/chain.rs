use crate::game::Move;
use crate::game::{Board, State};
use crate::utils::adjacencies::adjacencies;

#[derive(Debug, PartialEq)]
pub struct Chain<'a> {
    pub(crate) moves: Vec<&'a Move>,
}

impl<'a> Chain<'a> {
    pub fn move_is_connected(&self, mov: &Move, board: &Board) -> bool {
        // Loop through all chain members.
        for m in &self.moves {
            // Get adjacencies of move's intersection.
            for a in adjacencies(&m.intersection, &board) {
                // Check if adjacent state is a stone.
                if let Some(State::Stone(_)) = a.0 {
                    // Return true only if move's stone and intersection match that of
                    // adjacent intersection.
                    if m.stone == mov.stone && a.1 == mov.intersection {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn has_liberties(&self, board: &Board) -> bool {
        for m in &self.moves {
            for a in adjacencies(&m.intersection, &board) {
                if let Some(State::Vacant) = a.0 {
                    return true;
                }
            }
        }
        false
    }
}
