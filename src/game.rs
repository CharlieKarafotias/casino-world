use std::sync::{Arc, Mutex};
use log::{info};
use super::player::Player;
use super::crapless::CraplessCraps;

pub(super) trait Game: Send + Sync{
    fn run(&mut self);

    fn has_players(&self) -> bool;

    fn player_count(&self) -> u32;

    fn game_id(&self) -> u32;
    fn game_name(&self) -> &str;

    fn add_player(&mut self, player: Arc<Mutex<Player>>);

    fn remove_player(&mut self, player: &Arc<Mutex<Player>>);
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Bet {
    amount: u32,
    player: Arc<Player>,
}

impl Bet {
    pub(crate) fn new(amount: u32, player: &mut Player) -> Result<Bet,BetError > {
        match player.get_amount() >= amount as i32 {
            true => {
                player.add_amount(-(amount as i32));
                Ok(Bet { amount, player: Arc::new(player.clone()) })
            }
            false => Err(BetError::NotEnoughMoney(player.get_name().to_string()))
        }
    }
}

#[derive(Debug)]
pub enum BetError {
    NotEnoughMoney(String),
}

impl std::fmt::Display for BetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetError::NotEnoughMoney(name) => write!(f, "Not enough money to place bet: {}", name)
        }
    }
}

#[cfg(test)]
mod bet_tests {
    use super::Bet;
    use super::Player;
    #[test]
    fn test_new_bet() {
        let mut player = Player::new("John", 100);
        let bet = Bet::new(50, &mut player).unwrap();
        assert_eq!(bet.amount, 50);
        assert_eq!(bet.player.get_amount(), 50);
    }
}

#[derive(Debug)]
pub(super) enum GameNames {
    Crapless
}

pub struct GameProvider {
    game_name: GameNames,
    game: Vec<Box<dyn Game>>,
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

    pub fn add_player_to_game(&mut self, player: Arc<Mutex<Player>>) {
        // look for open game and add player
        for game in self.game.iter_mut() {
            if game.player_count() < self.player_limit {
                info!("Adding player {} to game: {} - {}", player.lock().unwrap().get_name(), game.game_name(), game.game_id());
                game.add_player(player);
                return;
            }
        }

        info!("Unable to find open game to add player: {}. Creating new session of game: {:#?}", player.lock().unwrap().get_name(), self.game_name);
        match self.game_name {
            GameNames::Crapless => {
                self.game.push(Box::new(CraplessCraps::new((self.game.len() + 1) as u32, vec![player])));
            }
        }
    }

    pub fn remove_player_from_game(&mut self, player: Arc<Mutex<Player>>) {
        for game in self.game.iter_mut() {
            game.remove_player(&player);
        }
    }
}