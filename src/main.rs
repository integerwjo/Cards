//! Poker
//! A binary crate for a version of poker played in kenya

#![forbid(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub mod cardmodule;
pub mod computer;
pub mod human;
pub mod gamestate;
pub mod player;

pub use crate::cardmodule::{Card, Deck, CardEffect};
pub use rand::seq::SliceRandom;
pub use rand::thread_rng;
use crate::{gamestate::GameState};
pub use gamestate::{ Number, Types };
pub use player::Player;

/// This represents a player instance or an instance of the model the human player plays with

fn main() {
     let mut game_state =  gamestate::GameState::new();
     cardmodule::print_top_card(&game_state);
     loop 
     {       
          human::player_turn(&mut game_state);
          println!("Can human finish: {}", game_state.human_player.can_finish());
          computer::computer_turn(&mut game_state);
          cardmodule::print_top_card(&game_state);
          println!("Can computer finish: {}", game_state.computer_player.can_finish());
         

     }


}
