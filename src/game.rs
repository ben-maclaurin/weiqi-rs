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

type BoardStates = HashMap<(u8, u8), State>;
type Intersection = (u8, u8);
type BoardSize = u8;

pub struct Turn {
    pub intersection: Intersection,
    pub stone: Stone,
}

pub struct Board {
    pub board_states: BoardStates,
    pub size: u8,
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

fn is_within_bounds(intersection: &Intersection, size: &BoardSize) -> bool {
    if (intersection.0 < 1 || intersection.1 < 1) || (intersection.0 > *size || intersection.1 > *size) {
        return false;
    }
    true
}