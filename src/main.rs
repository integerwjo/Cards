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
use crate::gamestate::GameState;
pub use gamestate::{ Number, Types };

/// This represents a player instance or an instance of the model the human player plays with
pub struct Player {
     pub cards_in_hand: Vec<Card>,
     pub is_turn_to_play: bool
}

impl Player{
     fn place_one_or_multiple_cards(no_of_cards_to_place: u8) {

           todo!()
        
     }
      
     fn pick(n: u8, player: &mut Self) {
          assert!(n > 0);
          for _ in 0..n {
               todo!()
          }
     }

     fn can_finish() {
          todo!()
      
     }


}


fn main() {
     let mut gamestate =  gamestate::GameState::new();
     print!("Top card is: ");
     player::player_turn(&mut gamestate);

}
