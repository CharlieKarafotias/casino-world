use log::info;
use crate::player::Player;

#[derive(Debug)]
pub enum GameNames {
    Crapless,
}

// purpose - keep track of games
pub struct GameProvider {
    game_name: GameNames,
    // game: can I track the threads running the games here?
    player_limit: u32,
}

impl GameProvider {
    pub fn new(game_name: GameNames, player_limit: u32) -> GameProvider {
        GameProvider {
            game_name,
            player_limit,
        }
    }

    pub fn game_name(&self) -> &GameNames {
        &self.game_name
    }

    pub fn add_player(&mut self, player: Player) {
        todo!("implement")
    }
}
