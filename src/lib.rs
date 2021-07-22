#![allow(dead_code)]
mod game;

#[cfg(test)]
mod tests {
    use crate::game::{Board, Turn, Stone, State, TurnResult, orthogonally_adjacent_states};
    use std::collections::HashMap;
    use crate::game::Illegal::OutOfBounds;

    #[test]
    fn can_read_orthogonal_states() {
       let mut b = Board {
           board_states: HashMap::new(),
           size: 9,
       };

       let t = Turn {
           intersection: (4, 4),
           stone: Stone::White,
       };

       b.update(t);

       assert_eq!(orthogonally_adjacent_states(&(4, 5), &b)[1], Some(State::Stone(Stone::White)));
    }

    #[test]
    fn cannot_move_out_of_bounds() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
        };

        let t = Turn {
            intersection: (10, 10),
            stone: Stone::Black,
        };

        assert_eq!(b.update(t), TurnResult::Illegal(OutOfBounds))
    }

    #[test]
    fn can_update_board() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
        };

        let t = Turn {
            intersection: (3, 3),
            stone: Stone::Black,
        };

        b.update(t);

        assert_eq!(b.read((3, 3)), Some(State::Stone(Stone::Black)));
    }
}
