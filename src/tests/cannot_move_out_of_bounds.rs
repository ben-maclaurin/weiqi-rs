use crate::chain::*;
use crate::mov::*;
use crate::game::*;
use crate::mov::Illegal::OutOfBounds;
use std::collections::HashMap;

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

    assert_eq!(b.update(&m), Interaction::Illegal(OutOfBounds))
}
