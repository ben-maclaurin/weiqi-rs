use crate::game::Illegal::OutOfBounds;
use crate::game::Outcome::Legal;
use crate::game::Rule::{RepeatMove, Suicide};
use std::collections::HashMap;

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
pub enum Rule {
    Suicide,
    RepeatMove,
}

#[derive(Debug, PartialEq)]
pub enum Illegal {
    Rule(Rule),
    OutOfBounds,
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Illegal(Illegal),
    Legal,
}

type Intersection = (i8, i8);
type BoardStates = HashMap<Intersection, State>;
type BoardSize = i8;

#[derive(PartialEq)]
pub struct Move {
    pub intersection: Intersection,
    pub stone: Stone,
}

pub struct Chain<'a> {
    moves: Vec<&'a Move>,
}

pub struct Board<'a> {
    pub board_states: BoardStates,
    pub size: BoardSize,
    pub chains: Vec<Chain<'a>>,
}

impl<'a> Board<'a> {
    pub fn update(&mut self, mov: Move) -> Outcome {
        if let Outcome::Illegal(illegal) = mov.is_prohibited(&self) {
            return Outcome::Illegal(illegal);
        }

        if let Some(mut chain) = can_connect_move(self, &mov) {
            chain.moves.push(&mov);
        }

        self.board_states.insert(
            (mov.intersection.0, mov.intersection.1),
            State::Stone(mov.stone),
        );
        Legal
    }

    pub(crate) fn read(&self, intersection: Intersection) -> Option<State> {
        if is_within_bounds(&intersection, &self.size) {
            if let Some(State::Stone(stone)) =
                &self.board_states.get(&(intersection.0, intersection.1))
            {
                return match stone {
                    Stone::Black => Some(State::Stone(Stone::Black)),
                    _ => Some(State::Stone(Stone::White)),
                };
            }
            return Some(State::Vacant);
        }
        None
    }
}

pub fn can_connect_move<'a>(board: &mut Board<'a>, mov: &Move) -> Option<Chain<'a>> {
    for c in &board.chains {
        if let Some(chain) = c.move_is_connected(&mov, &board) {
            return Some(chain);
        }
    }

    None
}

impl<'a> Chain<'a> {
    pub fn move_is_connected(&self, mov: &Move, board: &Board) -> Option<Self> {
        for m in &self.moves {
            for state in adjacent_states(&m.intersection, board) {
                if let Some(State::Stone(stone)) = state {
                    if mov.stone == stone {
                        Some(self);
                    }
                }
            }
        }
        None
    }

    pub fn has_liberties(&self, board: &Board) -> bool {
        for m in &self.moves {
            for state in adjacent_states(&m.intersection, &board) {
                if let Some(State::Vacant) = state {
                    return true;
                }
            }
        }
        false
    }
}

impl Move {
    pub fn is_prohibited(&self, board: &Board) -> Outcome {
        if !is_within_bounds(&self.intersection, &board.size) {
            return Outcome::Illegal(OutOfBounds);
        }

        if !self.has_liberties_or_allies(&board) {
            return Outcome::Illegal(Illegal::Rule(Suicide));
        }

        if self.is_repeat(&board) {
            return Outcome::Illegal(Illegal::Rule(RepeatMove));
        }

        Legal
    }

    pub fn is_repeat(&self, board: &Board) -> bool {
        if let Some(State::Stone(_)) = &board.read(self.intersection) {
            return true;
        }
        false
    }

    pub fn has_liberties_or_allies(&self, board: &Board) -> bool {
        let opponent = match &self.stone {
            Stone::Black => Stone::White,
            _ => Stone::Black,
        };

        for state in adjacent_states(&self.intersection, board) {
            match state {
                Some(State::Stone(stone)) => {
                    if stone != opponent {
                        return true;
                    }
                }
                _ => return true,
            };
        }

        false
    }
}

pub fn adjacent_states(intersection: &Intersection, board: &Board) -> Vec<Option<State>> {
    let mut states = Vec::<Option<State>>::new();

    let operations: Vec<i8> = vec![-1, 1];

    for operation in operations {
        states.push(board.read((intersection.0 + operation, intersection.1)));
        states.push(board.read((intersection.0, intersection.1 + operation)));
    }

    states
}

fn is_within_bounds(intersection: &Intersection, size: &BoardSize) -> bool {
    if (intersection.0 < 1 || intersection.1 < 1)
        || (intersection.0 > *size || intersection.1 > *size)
    {
        return false;
    }
    true
}
