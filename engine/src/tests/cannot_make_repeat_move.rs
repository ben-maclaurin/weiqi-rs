use crate::chain::*;
use crate::game::*;
use crate::mov::Rule::RepeatMove;
use crate::mov::*;
use std::collections::HashMap;

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

    assert_eq!(
        b.update(&m),
        Interaction::Illegal(Illegal::Rule(RepeatMove))
    )
}
