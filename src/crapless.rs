use super::game::{Bet, Game};
use super::player::Player;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Eq, Hash, PartialEq)]
enum Position {
    Two,
    Three,
    Four,
    Five,
    Six,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    COME,
    FIELD,
    PassLine,
    HardFour,
    HardSix,
    HardEight,
    HardTen,
    OneRollSeven,
    OneRollTwo,
    OneRollThree,
    OneRollEleven,
    OneRollTwelve,
}

enum GameState {
    Rolling,
    Betting,
}
struct Board {
    bets: HashMap<Position, Vec<Bet>>,
}

impl Board {
    fn new() -> Self {
        Board {
            bets: HashMap::new(),
        }
    }

    pub fn add_bet(&mut self, position: Position, bet: Bet) {
        self.bets.entry(position).or_insert(Vec::new()).push(bet);
    }

    pub fn clear_bet(&mut self, position: Position) {
        self.bets.remove(&position);
    }

    pub fn clear_all_bets(&mut self) {
        self.bets.clear();
    }

    pub fn get_bet(&self, position: Position) -> Option<&Vec<Bet>> {
        self.bets.get(&position)
    }
}

struct CraplessCrapsGameState {
    board: Board,
    point: Option<u8>,
    state: GameState,
    is_come_out_roll: bool,
}

impl CraplessCrapsGameState {
    fn new() -> Self {
        CraplessCrapsGameState {
            state: GameState::Betting,
            board: Board::new(),
            point: None,
            is_come_out_roll: true,
        }
    }

    fn set_point(&mut self, point: u8) {
        self.point = Some(point);
    }

    fn clear_point(&mut self) {
        self.point = None;
    }

    fn point(&self) -> Option<u8> {
        self.point
    }

    fn is_come_out_roll(&self) -> bool {
        self.is_come_out_roll
    }

    fn set_come_out_roll(&mut self, is_come_out_roll: bool) {
        self.is_come_out_roll = is_come_out_roll;
    }
}

pub(super) struct CraplessCraps {
    game_id: u32,
    game_state: CraplessCrapsGameState,
    players: Vec<Arc<RwLock<Player>>>,
}

impl CraplessCraps {
    pub fn new(game_id: u32, players: Vec<Arc<RwLock<Player>>>) -> Self {
        CraplessCraps {
            game_id,
            game_state: CraplessCrapsGameState::new(),
            players,
        }
    }

    pub(crate) async fn run(&mut self) {
        info!(
            "Started running of {} - {}",
            self.game_name(),
            self.game_id()
        );
        while self.has_players() {
            match (&self.game_state.state, self.game_state.is_come_out_roll()) {
                (GameState::Betting, true) => {
                    // wait for bets.
                    // switch to rolling
                }
                (GameState::Betting, false) => {
                    // wait for bets.
                    // switch to rolling
                }
                (GameState::Rolling, true) => {
                    // roll dice
                    // come out roll logic.
                    // If point is set, switch to betting, false.
                    // If point is not set, switch to betting, true.
                }
                (GameState::Rolling, false) => {
                    // roll dice
                    // regular roll logic.
                    // If point is hit, switch to betting, true.
                    // If point is not hit, switch to rolling, false.
                }
            }
        }
    }
}

impl Game for CraplessCraps {
    fn has_player(&self, player: &Arc<RwLock<Player>>) -> bool {
        self.players.iter().any(|p| Arc::ptr_eq(p, player))
    }

    fn has_players(&self) -> bool {
        !self.players.is_empty()
    }

    fn player_count(&self) -> u32 {
        self.players.len() as u32
    }

    fn game_id(&self) -> u32 {
        self.game_id
    }

    fn game_name(&self) -> &str {
        "Crapless"
    }

    fn add_player(&mut self, player: Arc<RwLock<Player>>) {
        self.players.push(player);
    }

    fn remove_player(&mut self, player: &Arc<RwLock<Player>>) {
        info!(
            "Player count in {} - {} before remove: {}",
            self.game_name(),
            self.game_id(),
            self.players.len()
        );
        self.players.retain(|p| !Arc::ptr_eq(p, &player));
        info!(
            "Player count in {} - {} after remove: {}",
            self.game_name(),
            self.game_id(),
            self.players.len()
        );
    }
}

#[cfg(test)]
mod board_tests {
    #[test]
    fn test_new_board() {
        let board = super::Board::new();
        assert!(board.bets.is_empty());
    }

    #[test]
    fn test_add_bet() {
        let mut board = super::Board::new();
        let mut player = super::Player::new("Player 1", 100);
        let bet = super::Bet::new(10, &mut player).unwrap();
        board.add_bet(super::Position::Two, bet.clone());
        assert_eq!(board.bets.len(), 1);
        assert_eq!(board.get_bet(super::Position::Two).unwrap(), &vec![bet]);
    }

    #[test]
    fn test_two_bets_same_position() {
        let mut board = super::Board::new();
        let mut player = super::Player::new("Player 1", 100);
        let bet = super::Bet::new(10, &mut player).unwrap();
        let bet2 = super::Bet::new(10, &mut player).unwrap();
        board.add_bet(super::Position::Two, bet.clone());
        board.add_bet(super::Position::Two, bet2.clone());
        // 1 position has bets on it
        assert_eq!(board.bets.len(), 1);
        assert_eq!(
            board.get_bet(super::Position::Two).unwrap(),
            &vec![bet, bet2]
        );
    }

    #[test]
    fn test_clear_bet() {
        let mut board = super::Board::new();
        let mut player = super::Player::new("Player 1", 100);
        let bet = super::Bet::new(10, &mut player).unwrap();
        board.add_bet(super::Position::Two, bet.clone());
        assert!(!board.bets.is_empty());
        board.clear_bet(super::Position::Two);
        assert!(board.bets.is_empty());
    }

    #[test]
    fn test_clear_all_bets() {
        let mut board = super::Board::new();
        let mut player = super::Player::new("Player 1", 100);
        let bet = super::Bet::new(10, &mut player).unwrap();
        let bet2 = super::Bet::new(10, &mut player).unwrap();
        board.add_bet(super::Position::Two, bet.clone());
        board.add_bet(super::Position::Ten, bet2.clone());
        assert_eq!(board.bets.len(), 2);
        board.clear_all_bets();
        assert!(board.bets.is_empty());
    }
}
