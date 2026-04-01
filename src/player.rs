//! This represents a generic player which can be either a computer player or a human player

use super::{Card, gamestate};
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
pub fn print_player_cards(player_cards: &Vec<Card>) {
    println!("Player's Cards:");
    for (i, card) in player_cards.iter().enumerate() {
        print!("index ({}): ", i);
        card.display();
    }
}

/// Handle player move input
pub fn player_places_a_card() -> usize {
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
pub fn check_if_player_move_is_valid(game_state: &gamestate::GameState, index: usize) -> bool {
    if index > game_state.human_player.cards_in_hand.len() {
        println!("Invalid index!");
        return false;
    }

    let top_card = game_state.top_card;
    println!("Top card on the pile: {:?}", top_card);
    false
    
}

/// Handles the player turn
pub fn player_turn(game_state: &mut gamestate::GameState) {
    print_player_cards(&game_state.human_player.cards_in_hand);

    println!("It's your turn! What would you like to do?");
    println!("1. Place cards");
    println!("2. Pick a card");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim() {
        "1" => {
            let num_cards = player_places_a_card();
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

                if check_if_player_move_is_valid(game_state, index) {
                    println!("Valid move! Placing card.");
                    // Remove the card from the player's hand and place it on the pile
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

