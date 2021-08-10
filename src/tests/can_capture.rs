use crate::chain::*;
use crate::game::*;
use crate::mov::*;
use std::collections::HashMap;

#[test]
fn can_capture() {
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
        intersection: (1, 2),
        stone: Stone::White,
    };

    b.update(&m);

    let m = Move {
        intersection: (2, 2),
        stone: Stone::White,
    };

    b.update(&m);

    let m = Move {
        intersection: (3, 1),
        stone: Stone::White,
    };

    b.update(&m);

    assert_eq!(b.read((1, 1)), Some(State::Vacant))
}
