#[derive(Debug, PartialEq)]
pub enum Stone {
    Black,
    White,
}


#[derive(Debug, PartialEq)]
pub enum Rule {
    Suicide,
    RepeatMove,
}

#[derive(Debug, PartialEq)]
pub enum Illegal {
    Rule(Rule),
    OutOfBounds,
}

#[derive(Debug, PartialEq)]
pub enum Interaction {
    Illegal(Illegal),
    Legal,
}

pub type Intersection = (i8, i8);

#[derive(Debug, PartialEq)]
pub struct Move {
    pub intersection: Intersection,
    pub stone: Stone,
}