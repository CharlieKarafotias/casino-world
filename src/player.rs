#[derive(Debug, PartialEq, Clone)]
pub(super) struct Player {
    name: String,
    amount: i32,
}

impl Player {
    pub fn new(name: impl Into<String>, amount: i32) -> Player {
        Player {
            name: name.into(),
            amount,
        }
    }

    pub(super) fn get_name(&self) -> &str {
        &self.name
    }

    pub(super) fn get_amount(&self) -> i32 {
        self.amount
    }

    fn set_amount(&mut self, amount: i32) {
        self.amount = amount;
    }

    pub(super) fn add_amount(&mut self, amount: i32) {
        self.amount += amount;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_player() {
        let player = super::Player::new("John".to_string(), 100);
        assert_eq!(
            player,
            super::Player {
                name: "John".to_string(),
                amount: 100
            }
        );
    }

    #[test]
    fn test_get_name() {
        let player = super::Player::new("John".to_string(), 100);
        assert_eq!(player.get_name(), "John");
    }

    #[test]
    fn test_get_amount() {
        let player = super::Player::new("John".to_string(), 100);
        assert_eq!(player.get_amount(), 100);
    }

    #[test]
    fn test_set_amount() {
        let mut player = super::Player::new("John".to_string(), 100);
        player.set_amount(200);
        assert_eq!(player.get_amount(), 200);
    }

    #[test]
    fn test_add_amount() {
        let mut player = super::Player::new("John".to_string(), 100);
        player.add_amount(100);
        assert_eq!(player.get_amount(), 200);
    }

    #[test]
    fn test_add_negative_amount() {
        let mut player = super::Player::new("John".to_string(), 100);
        player.add_amount(-100);
        assert_eq!(player.get_amount(), 0);
    }
}
