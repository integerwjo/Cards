//! Represents a generic player (human or computer)

use super::{Card, Number};

#[derive(Debug)]
pub struct Player {
    pub cards_in_hand: Vec<Card>,
    pub is_turn_to_play: bool,
}


