//! This module represents a generic player which could be a human or the computer
use crate::{gamestate::GameState, player};

use super::{Card, Number};

#[derive(Debug)]
pub struct Player {
     pub cards_in_hand: Vec<Card>,
     pub is_turn_to_play: bool
}

impl Player{

    /// simple case for checking if a player is able to finish the game (kadi)
    fn can_finish(player: &Self) -> bool {
        if player.cards_in_hand.len() == 1 {
            match player.cards_in_hand[0].number {
                Number::Four | Number::Five | Number::Six | Number::Seven | 
                Number::Nine | Number::Ten => true,
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
                Number::Four | Number::Five | Number::Six | Number::Seven |
                Number::Eight| Number::Nine | Number::Ten => true,
                _ => false
            }
        } else {
            false
        }
     }

