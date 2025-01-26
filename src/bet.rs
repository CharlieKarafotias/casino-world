use crate::player::Player;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Bet {
    amount: u32,
    player: Arc<Player>,
}

impl Bet {
    pub(crate) fn new(amount: u32, player: &mut Player) -> Result<Bet, BetError> {
        match player.get_amount() >= amount as i32 {
            true => {
                player.add_amount(-(amount as i32));
                Ok(Bet {
                    amount,
                    player: Arc::new(player.clone()),
                })
            }
            false => Err(BetError::NotEnoughMoney(player.get_name().to_string())),
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
            BetError::NotEnoughMoney(name) => write!(f, "Not enough money to place bet: {}", name),
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
