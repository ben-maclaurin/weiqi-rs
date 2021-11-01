use crate::game::{Board, State};
use crate::mov::*;
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
            for (state, intersection) in adjacencies(&m.intersection, &board) {
                // Check if adjacent state is a stone.
                if let Some(State::Stone(_)) = state {
                    // Return true only if move's stone and intersection match that of
                    // adjacent intersection.
                    if m.stone == mov.stone && intersection == mov.intersection {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn has_liberties(&self, board: &Board) -> bool {
        for m in &self.moves {
            for (state, _) in adjacencies(&m.intersection, &board) {
                if let Some(State::Vacant) = state {
                    return true;
                }
            }
        }
        false
    }
}
