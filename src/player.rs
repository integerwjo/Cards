//! Represents a generic player (human or computer)

use super::{Card, Number};

#[derive(Debug)]
pub struct Player {
    pub cards_in_hand: Vec<Card>,
    pub is_turn_to_play: bool,
}

impl Player {
    /// Returns true if the player can finish the game (kadi)
    pub fn can_finish(&self) -> bool {
        matches!(
            self.cards_in_hand.as_slice(),
            [Card { number: Number::Four
                | Number::Five
                | Number::Six
                | Number::Seven
                | Number::Eight
                | Number::Nine
                | Number::Ten, .. }]
        )
    }
}
