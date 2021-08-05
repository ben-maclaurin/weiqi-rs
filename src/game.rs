use crate::game::Illegal::OutOfBounds;
use crate::game::Interaction::Legal;
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
pub enum Interaction {
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
    pub fn update(&mut self, mov: &'a Move) -> Interaction {
        if let Interaction::Illegal(illegal) = mov.is_prohibited(&self) {
            return Interaction::Illegal(illegal);
        }

        self.board_states.insert(
            (mov.intersection.0, mov.intersection.1),
            State::Stone(&mov.stone),
        );

        let mut chain_index: (Option<usize>, bool) = (None, false);

        for (index, c) in self.chains.iter().enumerate() {
            if c.move_is_connected(&mov, &self) {
                chain_index = (Some(index), true);
            }
        }

        if chain_index.1 {
            // Add move to existing chain.
            if let Some(index) = chain_index.0 {
                self.chains[index].moves.push(&mov);
            }
        } else {
            self.chains.push(Chain { moves: vec![&mov] })
        }

        // Get indexes of dead chains.
        for index in self.get_dead_chains() {
            // Loop through each move in dead chain.
            for m in &self.chains[index].moves {
                // Remove old board state.
                self.board_states.remove(&m.intersection);
                // Insert modified board state.
                self.board_states.insert(
                    (mov.intersection.0, mov.intersection.1),
                    State::Vacant,
                );
            }
            
            // Remove chain from current list of chains.
            self.chains.remove(index);
        }

        Legal
    }

    fn get_dead_chains(&mut self) -> Vec<usize> {
        let mut dead_chains: Vec<usize> = vec![];

        for (index, chain) in self.chains.iter().enumerate() {
            if chain.has_liberties(&self) != true {
                dead_chains.push(index);
            }
        }

        dead_chains
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
        // Return none because intersection is not a valid board position.
        None
    }
}

impl<'a> Chain<'a> {
    pub fn move_is_connected(&self, mov: &Move, board: &Board) -> bool {
        // Loop through all chain members.
        for m in &self.moves {
            // Get adjacencies of move's intersection.
            for a in adjacencies(&m.intersection, &board) {
                // Check if adjacent state is a stone.
                if let Some(State::Stone(stone)) = a.0 {
                    // Return true only if move's stone and intersection match that of
                    // adjacent intersection.
                    println!("{:?}", &self);
                    if stone == &mov.stone && &mov.intersection == &a.1 {
                        println!("{:?} {:?}", stone, &mov.stone);
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

        for a in adjacencies(&self.intersection, board) {
            match a.0 {
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

pub fn adjacencies<'a>(
    intersection: &Intersection,
    board: &'a Board<'a>,
) -> Vec<(Option<State<'a>>, Intersection)> {
    let mut valid_adjacencies = Vec::<(Option<State>, Intersection)>::new();

    let operations: Vec<i8> = vec![-1, 1];

    for operation in operations {
        valid_adjacencies.push((
            board.read((intersection.0 + operation, intersection.1)),
            (intersection.0 + operation, intersection.1),
        ));
        valid_adjacencies.push((
            board.read((intersection.0, intersection.1 + operation)),
            (intersection.0, intersection.1 + operation),
        ));
    }

    valid_adjacencies
}

fn is_within_bounds(intersection: &Intersection, size: &BoardSize) -> bool {
    if (intersection.0 < 1 || intersection.1 < 1)
        || (intersection.0 > *size || intersection.1 > *size)
    {
        return false;
    }
    true
}
