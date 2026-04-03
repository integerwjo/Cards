//! This represents a generic player which can be either a computer player or a human player

use crate::gamestate::GameState;

use super::{Card, gamestate, Number};
use std::io;

/// Assign cards to player FROM the shared deck
pub fn assign_player_cards(deck: &mut Vec<Card>) -> Vec<Card> {
    println!("Assigning cards to player...");

    let mut player_cards = Vec::new();
    for _ in 0..4 {
        if let Some(card) = deck.pop() {
            player_cards.push(card);
        }
    }
    player_cards
}

/// Prints all the cards assigned to a player
pub fn print_player_cards(player_cards: &[Card]) {
    println!("\nYour Cards:");
    println!("---------------------------------");
    for (i, card) in player_cards.iter().enumerate() {
        print!("index ({}): ", i);
        card.display();
    }

}

/// Handle player move input
pub fn no_of_cards_player_is_placing() -> usize {
    println!("How many cards are you placing?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<usize>().unwrap_or(0)
}

/// Player draws one card from the deck
pub fn player_picks_a_card(game_state: &mut gamestate::GameState) {
    match game_state.deck.cards.pop() {
        Some(card) => {
            println!("Player picked a card:");
            card.display();
            game_state.human_player.cards_in_hand.push(card);
            print_player_cards(&game_state.human_player.cards_in_hand);
        }
        None => {
            println!("No more cards to pick!");
        }
    }
}

/// Checks if a move is valid


/// Handles the player turn
pub fn player_turn(game_state: &mut gamestate::GameState) {
    print_player_cards(&game_state.human_player.cards_in_hand);

    println!("It's your turn! What would you like to do?");
    println!("1. Place card(s)");
    println!("2. Pick a card");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim() {
        "1" => {
            let num_cards = no_of_cards_player_is_placing();
            println!("You chose to place {} cards.", num_cards);

            for _ in 0..num_cards {
                println!("Enter the index of the card you would like to place: ");

                let mut index_input = String::new();
                io::stdin()
                    .read_line(&mut index_input)
                    .expect("Failed to read line");

                let index: usize = match index_input.trim().parse() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("Please enter a valid number.");
                        continue;
                    }
                };

                if check_if_player_move_is_valid(&game_state.human_player.cards_in_hand[index], game_state) {
                    println!("Valid move! Card placed");
                    // Remove the card from the player's hand and place it on the pile
                    game_state.top_card = game_state.human_player.cards_in_hand[index];
                    game_state.human_player.cards_in_hand.remove(index);
                    print_player_cards(&game_state.human_player.cards_in_hand);
                }
                else {
                    println!("Invalid move!");
                }
            }
        }
        "2" => {
            player_picks_a_card(game_state);
        }
        _ => {
            println!("Invalid choice, please try again.");
        }
    }
}


/// This function takes a card and checks if the card can be placed as the top card
/// If it can, then the fn places the card as the top card
/// else it prints an error msg

fn check_if_player_move_is_valid(card: &Card, gamestate: &GameState) -> bool  {
    if (card.type_of_card == gamestate.top_card.type_of_card) || 
       (card.number == gamestate.top_card.number) || (card.number == Number::Ace) {
            true
       } else {
            false

       }
}
// AI
// AIM-> Understanding the environments
// --------------------------------------

// Intelligent agents
// Types of environmets
// 1. static environment -> remains unchanged excepy by the performance of actions by the agents
// 2. Dynamic env (xtics and how we can utilise it) -> One that has other processes operating on it
// compare and contrast static and dynamic environments
// 3. Deterministic(action has a single guaranteed state) and non deterministic env
// 4. descrete and continous env
// 5. Episodic(the performance of an agent is dependent on a number of discrete(not contionous)) episodes and sequential
// 6. Known vs unknown
// 7. Accessible 

