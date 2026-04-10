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


/// Dummy fn for computer move
pub fn computer_turn(gamestate: &mut GameState) {
    print_computer_cards(&gamestate.computer_player.cards_in_hand);
    println!("Computers turn");

    if computer_can_place(&gamestate) {
        assert!(!gamestate.computer_player.cards_in_hand.is_empty());
        println!("Computer placing");
        for i in 0..gamestate.computer_player.cards_in_hand.len() {
            if gamestate.computer_player.cards_in_hand[i].type_of_card == gamestate.top_card.type_of_card
                || gamestate.computer_player.cards_in_hand[i].number == gamestate.top_card.number
            {
                gamestate.top_card = gamestate.computer_player.cards_in_hand[i];
                gamestate.computer_player.cards_in_hand.remove(i);
                break; // stop after playing one card
            }
        }
    }
    else {
        println!("Computer picking a card...");
        computer_picks_a_card(gamestate);
    }
}


fn computer_can_place(gamestate: &GameState) -> bool {
    if gamestate.computer_player.cards_in_hand.is_empty() {
       return false
    }

    gamestate
        .computer_player
        .cards_in_hand
        .iter()
        .any(|card| {
            card.type_of_card == gamestate.top_card.type_of_card
                || card.number == gamestate.top_card.number
        })
}


pub fn computer_picks_a_card(game_state: &mut GameState) {
    match game_state.deck.cards.pop() {
        Some(card) => {
            println!("Player picked a card:");
            card.display();
            game_state.computer_player.cards_in_hand.push(card);
            print_computer_cards(&game_state.computer_player.cards_in_hand);
        },
        None => {
            println!("No more cards to pick!");
        }
    }
}


