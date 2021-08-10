use crate::chain::*;
use crate::mov::*;
use crate::game::*;
use crate::mov::Rule::Suicide;
use std::collections::HashMap;

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

    assert_eq!(b.update(&m), Interaction::Illegal(Illegal::Rule(Suicide)))
}
