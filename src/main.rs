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
pub use crate::cardmodule::Card;
pub use gamestate::{ Number, Types };


fn main() {
     let mut gamestate =  gamestate::GameState::initialize_game_state();
     print!("Top card is: ");
     gamestate.get_top_card().display();
     player::player_turn(&mut gamestate);
     


 

}
