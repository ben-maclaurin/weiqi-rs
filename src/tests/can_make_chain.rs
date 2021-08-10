use crate::chain::*;
use crate::game::*;
use std::collections::HashMap;

#[test]
fn can_make_chain() {
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
