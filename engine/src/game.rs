use crate::chain::Chain;
use crate::mov::Interaction::Legal;
use crate::mov::{Interaction, Intersection, Move, Stone};
use crate::utils::bounds::is_within_bounds;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum State<'a> {
    Vacant,
    Stone(&'a Stone),
}

pub type BoardStates<'a> = HashMap<Intersection, State<'a>>;
pub type BoardSize = i8;

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

        let (index, found) = chain_index;

        if found {
            // Add move to existing chain.
            if let Some(index) = index {
                self.chains[index].moves.push(&mov);
            }
        } else {
            self.chains.push(Chain { moves: vec![&mov] });
        }

        // Get indexes of dead chains.
        for index in self.get_dead_chains() {
            // Loop through each move in dead chain.
            for m in &self.chains[index].moves {
                // Remove old board state.
                self.board_states.remove(&m.intersection);
                // Insert modified board state.

                self.board_states
                    .insert((mov.intersection.0, mov.intersection.1), State::Vacant);
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
