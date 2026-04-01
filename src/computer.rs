use super::Card;



/// Prints all cards assigned to the computer
/// Should not be visible to the player (debug only)
pub fn print_computer_cards(computer_cards: &Vec<Card>) {
    println!("Computer's Cards:");
    for card in computer_cards {
        card.display();
    }
}

/// Assigns the computer 4 starting cards FROM the shared deck
pub fn assign_computer_cards(deck: &mut Vec<Card>) -> Vec<Card> {
    let mut computer_cards = Vec::new();

    for _ in 0..4 {
        if let Some(card) = deck.pop() {
            computer_cards.push(card);
        }
    }

    computer_cards
}

pub fn computer_picks_a_card(game_state: &mut gamestate::GameState) {
    match game_state.deck.pop() {
        Some(card) => {
            println!("Computer picked a card:");
            card.display();
            game_state.computer_hand.push(card);
            print_player_cards(&game_state.computer_hand);
        }
        None => {
            println!("No more cards to pick!");
        }
    }
}
