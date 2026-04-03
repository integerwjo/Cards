//! This module represents a generic player which could be a human or the computer
use crate::{gamestate::GameState, player};

use super::{Card, Number};

#[derive(Debug)]
pub struct Player {
     pub cards_in_hand: Vec<Card>,
     pub is_turn_to_play: bool
}

impl Player{
     fn can_finish(player: &Self) -> bool {
        if player.cards_in_hand.len() == 1 {
            match player.cards_in_hand[0].number {
                Number::Four => true,
                Number::Five => true,
                Number::Six => true,
                Number::Seven => true,
                Number::Eight => true,
                Number::Nine => true,
                Number::Ten => true,
                _ => false
            }
        } else {
            false
        }
     }


}

pub fn can_finish(player: &Player) -> bool {
        if player.cards_in_hand.len() == 1 {
            match player.cards_in_hand[0].number {
                Number::Four => true,
                Number::Five => true,
                Number::Six => true,
                Number::Seven => true,
                Number::Eight => true,
                Number::Nine => true,
                Number::Ten => true,
                _ => false
            }
        } else {
            false
        }
     }

