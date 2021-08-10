use crate::chain::*;
use crate::game::*;
use crate::mov::*;
use std::collections::HashMap;

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
