use crate::player::Player;
use std::sync::Arc;

pub(crate) trait Game: Send + Sync {
    fn has_player(&self, player: Arc<Player>) -> bool;

    fn has_players(&self) -> bool;

    fn player_count(&self) -> u32;

    fn game_id(&self) -> u32;
    fn game_name(&self) -> &str;

    fn add_player(&mut self, player: Arc<Player>);

    fn remove_player(&mut self, player: Arc<Player>);
}
