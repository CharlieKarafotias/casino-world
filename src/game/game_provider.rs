use super::super::crapless::CraplessCraps;
use super::super::player::Player;
use super::game::Game;
use log::info;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub enum GameNames {
    Crapless,
}

pub struct GameProvider {
    game_name: GameNames,
    game: Vec<Arc<RwLock<dyn Game>>>,
    player_limit: u32,
}

impl GameProvider {
    pub fn new(game_name: GameNames, player_limit: u32) -> GameProvider {
        GameProvider {
            game_name,
            game: Vec::new(),
            player_limit,
        }
    }

    pub fn game_name(&self) -> &GameNames {
        &self.game_name
    }

    pub async fn add_player_to_game(&mut self, player: Arc<RwLock<Player>>) {
        // look for open game and add player
        for game in self.game.iter() {
            let mut game = game.write().await;
            if game.player_count() < self.player_limit {
                info!(
                    "Adding player {} to game: {} - {}",
                    player.read().await.get_name(),
                    game.game_name(),
                    game.game_id()
                );
                game.add_player(player);
                return;
            }
        }

        match self.game_name {
            GameNames::Crapless => {
                let game_id = (self.game.len() + 1) as u32;
                let game = Arc::new(RwLock::new(CraplessCraps::new(game_id, vec![player])));
                self.game.push(game.clone());
                info!(
                    "Creating new session of game: {:#?} - {}",
                    self.game_name, game_id
                );
                // TODO - BUG: existing bug - blocking server
                tokio::spawn(async move {
                    let mut game = game.write().await;
                    game.run().await;
                });
            }
        }
    }

    pub async fn remove_player_from_game(&mut self, player: Arc<RwLock<Player>>) {
        // find games with player
        for game in self.game.iter() {
            let mut game = game.write().await;
            if game.has_player(&player) {
                info!(
                    "Removing player {} from game: {} - {}",
                    player.read().await.get_name(),
                    game.game_name(),
                    game.game_id()
                );
                game.remove_player(&player);
            }
        }

        // TODO: Remove game if no players
    }

    pub fn game_count(&self) -> u32 {
        self.game.len() as u32
    }
}
