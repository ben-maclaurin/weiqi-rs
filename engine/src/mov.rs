use crate::game::*;
use crate::mov::Illegal::OutOfBounds;
use crate::mov::Interaction::Legal;
use crate::mov::Rule::RepeatMove;
use crate::mov::Rule::Suicide;
use crate::utils::adjacencies::adjacencies;
use crate::utils::bounds::is_within_bounds;

#[derive(Debug, PartialEq)]
pub enum Stone {
    Black,
    White,
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
pub enum Interaction {
    Illegal(Illegal),
    Legal,
}

pub type Intersection = (i8, i8);

#[derive(Debug, PartialEq)]
pub struct Move {
    pub intersection: Intersection,
    pub stone: Stone,
}

impl Move {
    pub fn is_prohibited(&self, board: &Board) -> Interaction {
        if !is_within_bounds(&self.intersection, &board.size) {
            return Interaction::Illegal(OutOfBounds);
        }

        if !self.has_liberties_or_allies(&board) {
            return Interaction::Illegal(Illegal::Rule(Suicide));
        }

        if self.is_repeat(&board) {
            return Interaction::Illegal(Illegal::Rule(RepeatMove));
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

        for (state, _) in adjacencies(&self.intersection, board) {
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
