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
pub enum State<'a> {
    Vacant,
    Stone(&'a Stone),
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
type BoardStates<'a> = HashMap<Intersection, State<'a>>;
type BoardSize = i8;

#[derive(Debug, PartialEq)]
pub struct Move {
    pub intersection: Intersection,
    pub stone: Stone,
}

#[derive(Debug, PartialEq)]
pub struct Chain<'a> {
    pub(crate) moves: Vec<&'a Move>,
}

#[derive(Debug, PartialEq)]
pub struct Board<'a> {
    pub board_states: BoardStates<'a>,
    pub size: BoardSize,
    pub chains: Vec<Chain<'a>>,
}

impl<'a> Board<'a> {
    pub fn update(&mut self, mov: &'a Move) -> Outcome {
        if let Outcome::Illegal(illegal) = mov.is_prohibited(&self) {
            return Outcome::Illegal(illegal);
        }

        self.board_states.insert(
            (mov.intersection.0, mov.intersection.1),
            State::Stone(&mov.stone),
        );

        if !self.can_connect_move(&mov) {
            self.chains.push( Chain {
                moves: vec![&mov],
            })
        }

        Legal
    }

    pub(crate) fn read(&self, intersection: Intersection) -> Option<State> {
        if is_within_bounds(&intersection, &self.size) {
            if let Some(State::Stone(stone)) =
                &self.board_states.get(&(intersection.0, intersection.1))
            {
                return match stone {
                    Stone::Black => Some(State::Stone(&Stone::Black)),
                    _ => Some(State::Stone(&Stone::White)),
                };
            }
            return Some(State::Vacant);
        }
        None
    }

    fn can_connect_move(&mut self, mov: &'a Move) -> bool {
        for c in self.chains.iter_mut() {
            if c.move_is_connected(&mov, self) {
                c.moves.push(&mov);
                return true;
            }
        }
        false
    }
}



impl<'a> Chain<'a> {
    pub fn move_is_connected(&self, mov: &Move, board: &Board) -> bool {
        for m in &self.moves {
            for state in adjacent_states(&m.intersection, &board) {
                if let Some(State::Stone(stone)) = state {
                    if &mov.stone == stone {
                        return true;
                    }
                }
            }
        }
        false
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
                    if stone != &opponent {
                        return true;
                    }
                }
                _ => return true,
            };
        }

        false
    }
}

pub fn adjacent_states<'a>(intersection: &Intersection, board: &'a Board<'a>) -> Vec<Option<State<'a>>> {
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
