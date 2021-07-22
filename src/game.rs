use std::collections::HashMap;
use crate::game::Illegal::OutOfBounds;
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
        if is_within_bounds(&turn.intersection, &self.size) {
            self.board_states.insert((turn.intersection.0, turn.intersection.1), State::Stone(turn.stone));
            return Legal;
        }
        TurnResult::Illegal(OutOfBounds)
    }

    pub(crate) fn read(&self, intersection: Intersection) -> Option<State> {
        if is_within_bounds(&intersection, &self.size) {
            if let Some(State::Stone(stone)) = &self.board_states.get(&(intersection.0, intersection.1)) {
                return match stone {
                    Stone::Black => Some(State::Stone(Stone::Black)),
                    _ => Some(State::Stone(Stone::White))
                }
            }
            return Some(State::Vacant);
        }
        None
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