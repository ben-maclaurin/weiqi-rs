use warp::Filter;
use serde::{Serialize, Deserialize};
use engine::game::{Board, BoardSize};

#[derive(Deserialize, Serialize)]
struct CreateBoard {
    pub size: BoardSize,
}

#[tokio::main]
async fn main() {
    let create_board = warp::post()
    .and(warp::path("board"))
    .and(warp::path("create"))
    .and(warp::body::json())
    .map(|board: CreateBoard| {
        if board.size > 0 {
            return format!("Board created");
        } else {
            return format!("Board not created");
        }
    });


    warp::serve(create_board)
    .run(([127, 0, 0, 1], 8080))
    .await;
}

