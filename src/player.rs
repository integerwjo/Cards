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
    match game_state.deck.pop() {
        Some(card) => {
            println!("Player picked a card:");
            card.display();
            game_state.player_hand.push(card);
            print_player_cards(&game_state.player_hand);
        }
        None => {
            println!("No more cards to pick!");
        }
    }
}

/// Checks if a move is valid
pub fn check_if_player_move_is_valid(game_state: &gamestate::GameState, index: usize) -> bool {
    if index > game_state.player_hand.len() {
        println!("Invalid index!");
        return false;
    }

    let _top_card = game_state.get_top_card();
    println!("Top card on the pile:");
    false
    
}

/// Handles the player turn
pub fn player_turn(game_state: &mut gamestate::GameState) {
    print_player_cards(&game_state.player_hand);

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
                    let card = game_state.player_hand.remove(index);
                    game_state.pile.push(card);
                    print_player_cards(&game_state.player_hand);
                } else {
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


