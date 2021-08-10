use warp::Filter;
use serde::{Serialize, Deserialize};
use engine::game::{Board, BoardSize};
use std::collections::HashMap;
use engine::chain::Chain;

#[derive(Deserialize, Serialize)]
struct CreateBoard {
    pub size: BoardSize,
}

#[tokio::main]
async fn main() {
    let create_board = warp::post()
    .and(warp::path("new"))
    .and(warp::path("game"))
    .and(warp::body::json())
    .map(|board: CreateBoard| {
        if board.size > 0 {
            return format!("New game created");
        } else {
            return format!("Game failed to create");
        }
    });

    warp::serve(create_board)
    .run(([127, 0, 0, 1], 8080))
    .await;
}

