//! Represents a generic player (human or computer)

use super::{Card, Number};

#[derive(Debug)]
pub struct Player {
    pub cards_in_hand: Vec<Card>,
    pub is_turn_to_play: bool,
}

impl Player {
    /// Creates a new player
    pub fn new(cards_in_hand: Vec<Card>, is_turn_to_play: bool) -> Self {
        Self {
            cards_in_hand,
            is_turn_to_play,
        }
    }

    /// Returns true if all cards in hand are "low value" (Four to Ten)
    ///
    /// This can be used as a game rule for allowing a player to finish.
    pub fn can_finish(&self) -> bool {
        self.cards_in_hand.iter().all(|card| {
            matches!(
                card.number,
                Number::Four
                    | Number::Five
                    | Number::Six
                    | Number::Seven
                    | Number::Eight
                    | Number::Nine
                    | Number::Ten
            )
        })
    }

    /// Returns the number of cards in the player's hand
    pub fn hand_size(&self) -> usize {
        self.cards_in_hand.len()
    }

    /// Adds a card to the player's hand
    pub fn draw_card(&mut self, card: Card) {
        self.cards_in_hand.push(card);
    }

    /// Removes a card from hand (if it exists)
    pub fn play_card(&mut self, index: usize) -> Option<Card> {
        if index < self.cards_in_hand.len() {
            Some(self.cards_in_hand.remove(index))
        } else {
            None
        }
    }
}