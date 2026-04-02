//! Poker
//! A binary crate for a version of poker played in kenya

#![forbid(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub mod cardmodule;
pub mod computer;
pub mod player;
pub mod gamestate;

pub use rand::seq::SliceRandom;
pub use rand::thread_rng;
pub use crate::cardmodule::{Card, Deck};
use crate::gamestate::{GameState};
pub use gamestate::{ Number, Types };

/// This represents a player instance or an instance of the model the human player plays with
#[derive(Debug)]
pub struct Player {
     pub cards_in_hand: Vec<Card>,
     pub is_turn_to_play: bool
}

impl Player{


     fn can_finish(player: &Self) -> bool {
          todo!()
      
     }


}


fn main() {
     let mut game_state =  gamestate::GameState::new();
     print!("Top card is: {:?}", game_state.top_card);
     player::player_turn(&mut game_state);
     computer::computer_turn(&mut game_state);
     
     
     
     

}
