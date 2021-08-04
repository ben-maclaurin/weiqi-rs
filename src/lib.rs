#![allow(dead_code)]
mod game;

#[cfg(test)]
mod tests {
    use crate::game::Illegal::OutOfBounds;
    use crate::game::Rule::{RepeatMove, Suicide};
    use crate::game::{adjacent_states, Board, Chain, Illegal, Move, Outcome, State, Stone};
    use std::collections::HashMap;

    #[test]
    fn can_make_group() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
            chains: Vec::<Chain>::new(),
        };

        let m = Move {
            intersection: (1, 1),
            stone: Stone::Black,
        };

        b.update(&m);

        let m = Move {
            intersection: (2, 1),
            stone: Stone::Black,
        };

        b.update(&m);

        let m = Move {
            intersection: (7, 7),
            stone: Stone::Black,
        };

        b.update(&m);

        assert_eq!(b.chains[0].moves.len(), 2);
    }

    #[test]
    fn cannot_make_suicidal_move() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
            chains: Vec::<Chain>::new(),
        };

        let m = Move {
            intersection: (3, 2),
            stone: Stone::White,
        };

        b.update(&m);

        let m = Move {
            intersection: (4, 3),
            stone: Stone::White,
        };

        b.update(&m);

        let m = Move {
            intersection: (3, 4),
            stone: Stone::White,
        };

        b.update(&m);

        let m = Move {
            intersection: (2, 3),
            stone: Stone::White,
        };

        b.update(&m);

        let m = Move {
            intersection: (3, 3),
            stone: Stone::Black,
        };

        assert_eq!(b.update(&m), Outcome::Illegal(Illegal::Rule(Suicide)))
    }

    #[test]
    fn cannot_make_repeat_move() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
            chains: Vec::<Chain>::new(),
        };

        let m = Move {
            intersection: (1, 1),
            stone: Stone::White,
        };

        b.update(&m);

        let m = Move {
            intersection: (1, 1),
            stone: Stone::White,
        };

        assert_eq!(b.update(&m), Outcome::Illegal(Illegal::Rule(RepeatMove)))
    }

    #[test]
    fn can_read_orthogonal_states() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
            chains: Vec::<Chain>::new(),
        };

        let m = Move {
            intersection: (4, 4),
            stone: Stone::White,
        };

        b.update(&m);

        assert_eq!(
            adjacent_states(&(4, 5), &b)[1],
            Some((Some(State::Stone(&Stone::White)), (4 as i8, 4 as i8)))
        );
    }

    #[test]
    fn cannot_move_out_of_bounds() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
            chains: Vec::<Chain>::new(),
        };

        let m = Move {
            intersection: (10, 10),
            stone: Stone::Black,
        };

        assert_eq!(b.update(&m), Outcome::Illegal(OutOfBounds))
    }

    #[test]
    fn can_update_board() {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
            chains: Vec::<Chain>::new(),
        };

        let m = Move {
            intersection: (3, 3),
            stone: Stone::Black,
        };

        b.update(&m);

        assert_eq!(b.read((3, 3)), Some(State::Stone(&Stone::Black)));
    }
}
