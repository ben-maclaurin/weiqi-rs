use std::collections::HashMap;
use crate::game::Illegal::{OutOfBounds, Suicide};
use crate::game::TurnResult::Legal;

#[derive(Debug, PartialEq)]
pub enum Stone {
    Black,
    White,
}

#[derive(Debug, PartialEq)]
pub enum State {
    Vacant,
    Stone(Stone),
}

#[derive(Debug, PartialEq)]
pub enum Illegal {
    OutOfBounds,
    Suicide,
}

#[derive(Debug, PartialEq)]
pub enum TurnResult {
    Illegal(Illegal),
    Legal,
}

type BoardStates = HashMap<(i8, i8), State>;
type Intersection = (i8, i8);
type BoardSize = i8;

pub struct Turn {
    pub intersection: Intersection,
    pub stone: Stone,
}

pub struct Board {
    pub board_states: BoardStates,
    pub size: BoardSize,
}

impl Board {
    pub fn update(&mut self, turn: Turn) -> TurnResult {
        if !is_within_bounds(&turn.intersection, &self.size) {
            return TurnResult::Illegal(OutOfBounds);
        }

        if turn.is_suicide(&self) {
            return TurnResult::Illegal(Suicide);
        }

        self.board_states.insert((turn.intersection.0, turn.intersection.1), State::Stone(turn.stone));
        Legal
    }

    pub(crate) fn read(&self, intersection: Intersection) -> Option<State> {
        if is_within_bounds(&intersection, &self.size) {
            if let Some(State::Stone(stone)) = &self.board_states.get(&(intersection.0, intersection.1)) {
                return match stone {
                    Stone::Black => Some(State::Stone(Stone::Black)),
                    _ => Some(State::Stone(Stone::White))
                };
            }
            return Some(State::Vacant);
        }
        None
    }
}

impl Turn {
    pub fn is_suicide(&self, board: &Board) -> bool {
        let opponent = match &self.stone {
            Stone::Black => Stone::White,
            _ => Stone::Black,
        };

        for state in orthogonally_adjacent_states(&self.intersection, board) {
            if let Some(State::Stone(stone)) = state {
                if stone != opponent {
                    return false;
                }
            }
        }

        true
    }
}

pub fn orthogonally_adjacent_states(intersection: &Intersection, board: &Board) -> Vec<Option<State>> {
    let mut adjacent_states = Vec::<Option<State>>::new();

    let operations: Vec<i8> = vec![-1, 1];

    for operation in operations {
        adjacent_states.push(board.read((intersection.0 + operation, intersection.1)));
        adjacent_states.push(board.read((intersection.0, intersection.1 + operation)));
    }

    adjacent_states
}

fn is_within_bounds(intersection: &Intersection, size: &BoardSize) -> bool {
    if (intersection.0 < 1 || intersection.1 < 1) || (intersection.0 > *size || intersection.1 > *size) {
        return false;
    }
    true
}