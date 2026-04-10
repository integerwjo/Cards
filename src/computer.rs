//! The computer functions are defined here
//! 
use super::{Card, GameState};
use crate::Types;
use crate::Number;

/// Prints all cards assigned to the computer
/// Should not be visible to the player (debug only)
pub fn print_computer_cards(computer_cards: &Vec<Card>) {
    println!("\nComputer's Cards:");
    for card in computer_cards {
        card.display();
    }
}




