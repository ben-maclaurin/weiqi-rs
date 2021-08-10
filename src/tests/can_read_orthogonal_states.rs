use crate::chain::*;
use crate::game::*;
use crate::utils::adjacencies::adjacencies;
use std::collections::HashMap;

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
        adjacencies(&(4, 5), &b)[1],
        (Some(State::Stone(&Stone::White)), (4, 4))
    );
}
