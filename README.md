# weiqi-rs

![Go players](https://upload.wikimedia.org/wikipedia/commons/e/e3/Korean_Game_from_the_Carpenter_Collection%2C_ca._1910-1920.jpg)

A simple Weiqi (Go) library written in Rust. t

### What is Go?

Go is a board game for two players. It is called Wei-k'i (or Wei-chi) in Chinese, Patok or Baduk in Korean and I-go in Japanese.

Go is played on a board with black and white game pieces called stones. Players take turns placing a stone of their color on intersections of a 19x19 square grid. The player with the black stones goes first. A normal Go board has 19 rows and columns of lines. Some players use 9x9 or 13x13 boards because smaller boards usually mean shorter, less complex games.

_Definition from Simple English Wikipedia. [Read more](https://simple.wikipedia.org/wiki/Go_(board_game))._

### Getting started

```rust
//
// Initialise an empty board. The board size can be any number
// but the standard sizes are 9x9, 13x13 and 19x19.
//
let mut board = Board {
    board_states: HashMap::new(),
    size: 19,
    chains: Vec::<Chain>::new(),
};

//
// Place a white stone at intersection (1, 1).
//
let first_move = Move {
    intersection: (1, 1),
    stone: Stone::White,
};

//
// Update the board and handle the result.
// Output: Some(State::Stone(Stone::White))
//
match board.update(&first_move) {
    Interaction::Legal => {
        println!("{:?}", &board.read((1, 1)).unwrap());
    }
    Interaction::Illegal(violated_rule) => {
        println!("{:?}", violated_rule);
        // Do error handling here...
    }
};
```

### State of Library

This library is being actively developed. Feel free to contribute and improve.

Project status is as follows:

- [x] Basic game rules (suicide, repeat moves, board bounds)
- [x] Chains (or groups)
- [x] Capture
- [ ] Client
- [ ] Scoring
- [ ] Examples
- [ ] Server?
