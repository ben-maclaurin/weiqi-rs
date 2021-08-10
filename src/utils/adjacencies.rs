use crate::game::{Board, Intersection, State};

pub fn adjacencies<'a>(
    intersection: &Intersection,
    board: &'a Board<'a>,
) -> Vec<(Option<State<'a>>, Intersection)> {
    let mut valid_adjacencies = Vec::<(Option<State>, Intersection)>::new();

    let operations: Vec<i8> = vec![-1, 1];

    for operation in operations {
        valid_adjacencies.push((
            board.read((intersection.0 + operation, intersection.1)),
            (intersection.0 + operation, intersection.1),
        ));
        valid_adjacencies.push((
            board.read((intersection.0, intersection.1 + operation)),
            (intersection.0, intersection.1 + operation),
        ));
    }

    valid_adjacencies
}
