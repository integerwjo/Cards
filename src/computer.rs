//! Computer player logic

use super::{Card, GameState};

/// Prints all cards assigned to the computer.
/// Debug only — should not be visible to the player.
pub fn print_computer_cards(cards: &[Card]) {
    println!("\nComputer's Cards:");

    for card in cards {
        card.display();
    }
}

/// Executes the computer player's turn.
pub fn computer_turn(game_state: &mut GameState) {
    print_computer_cards(&game_state.computer_player.cards_in_hand);

    println!("Computer's turn");

    match find_playable_card_index(game_state) {
        Some(index) => {
            println!("Computer placing");

            let played_card = game_state
                .computer_player
                .cards_in_hand
                .remove(index);

            game_state.top_card = played_card;
        }
        None => {
            println!("Computer picking a card...");
            computer_picks_a_card(game_state);
        }
    }
}

/// Returns the index of the first playable card in hand.
fn find_playable_card_index(game_state: &GameState) -> Option<usize> {
    game_state
        .computer_player
        .cards_in_hand
        .iter()
        .position(|card| is_playable(card, &game_state.top_card))
}

/// Determines whether a card can be played on top of the current card.
fn is_playable(card: &Card, top_card: &Card) -> bool {
    card.type_of_card == top_card.type_of_card
        || card.number == top_card.number
}

/// Draws one card from the deck for the computer player.
pub fn computer_picks_a_card(game_state: &mut GameState) {
    match game_state.deck.cards.pop() {
        Some(card) => {
            println!("Computer picked a card:");
            card.display();

            game_state.computer_player.cards_in_hand.push(card);

            print_computer_cards(&game_state.computer_player.cards_in_hand);
        }
        None => {
            println!("No more cards to pick!");
        }
    }
}


