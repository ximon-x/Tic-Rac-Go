use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Player {
    X,
    O,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub board: Vec<Vec<u8>>,
    pub player: Player,
    pub game_over: bool,
}

impl Game {
    pub fn new(player: Option<Player>, dim: usize) -> Game {
        let rows = vec![0; dim];
        let board = vec![rows; dim];

        let player = player.unwrap_or(Player::X);
        let game_over = false;

        Game {
            board,
            player,
            game_over,
        }
    }
}
