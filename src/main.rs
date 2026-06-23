//! Poker
//! A binary crate for a Kenyan variation of poker.

#![forbid(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub mod cardmodule;
pub mod computer;
pub mod gamestate;
pub mod human;
pub mod player;



pub use crate::cardmodule::{Card, CardEffect, Deck};
pub use gamestate::{GameState, Number, Types};
pub use player::Player;

pub use rand::seq::SliceRandom;
pub use rand::thread_rng;

/// Represents the game loop for the poker game.
fn main() {
    let mut game_state = GameState::new();

    cardmodule::print_top_card(&game_state);

    loop {
        human::player_turn(&mut game_state);
        println!("Can human finish: {}", game_state.human_player.can_finish());  

        computer::computer_turn(&mut game_state);

        cardmodule::print_top_card(&game_state);

        println!(
            "Can computer finish: {}",
            game_state.computer_player.can_finish();
        );
    }
}
