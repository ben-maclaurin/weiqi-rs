#![allow(dead_code)]
mod game;

#[cfg(test)]
mod tests {
    use crate::game::{Board, Stone, State, orthogonally_adjacent_states, Move, Outcome, Illegal};
    use std::collections::HashMap;
    use crate::game::Illegal::{OutOfBounds};
    use crate::game::Rule::{RepeatMove, Suicide};

    #[test]
    fn cannot_make_suicidal_move() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
        };

        let m = Move {
            intersection: (3, 2),
            stone: Stone::White,
        };

        b.update(m);

        let m = Move {
            intersection: (4, 3),
            stone: Stone::White,
        };

        b.update(m);

        let m = Move {
            intersection: (3, 4),
            stone: Stone::White,
        };

        b.update(m);

        let m = Move {
            intersection: (2, 3),
            stone: Stone::White,
        };

        b.update(m);

        let m = Move {
            intersection: (3, 3),
            stone: Stone::Black,
        };

        assert_eq!(b.update(m), Outcome::Illegal(Illegal::Rule(Suicide)))
    }

    #[test]
    fn cannot_make_repeat_move() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
        };

        let m = Move {
            intersection: (1, 1),
            stone: Stone::White,
        };

        b.update(m);

        let m = Move {
            intersection: (1, 1),
            stone: Stone::White,
        };

        assert_eq!(b.update(m), Outcome::Illegal(Illegal::Rule(RepeatMove)))
    }

    #[test]
    fn can_read_orthogonal_states() {
       let mut b = Board {
           board_states: HashMap::new(),
           size: 9,
       };

       let m = Move {
           intersection: (4, 4),
           stone: Stone::White,
       };

       b.update(m);

       assert_eq!(orthogonally_adjacent_states(&(4, 5), &b)[1], Some(State::Stone(Stone::White)));
    }

    #[test]
    fn cannot_move_out_of_bounds() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
        };

        let m = Move {
            intersection: (10, 10),
            stone: Stone::Black,
        };

        assert_eq!(b.update(m), Outcome::Illegal(OutOfBounds))
    }

    #[test]
    fn can_update_board() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
        };

        let m = Move {
            intersection: (3, 3),
            stone: Stone::Black,
        };

        b.update(m);

        assert_eq!(b.read((3, 3)), Some(State::Stone(Stone::Black)));
    }
}
