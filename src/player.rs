//! This module represents a generic player which could be a human or the computer
use super::Card;

#[derive(Debug)]
pub struct Player {
     pub cards_in_hand: Vec<Card>,
     pub is_turn_to_play: bool
}

impl Player{


     fn can_finish(_player: &Self) -> bool {
          todo!()
      
     }


}

