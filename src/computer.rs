//! The computer functions are defined here
//! 
use super::{Card, GameState};


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


pub fn create_combos(gamestate: &GameState) -> Vec<Vec<Card>> {
    let hand = &gamestate.computer_player.cards_in_hand;
    let top = gamestate.top_card;

    let mut valid_plays: Vec<Vec<Card>> = Vec::new();

    // Helper: can a single card be placed on top?
    let can_play = |c: &Card| {
        c.number == top.number || c.type_of_card == top.type_of_card
    };

    // -------------------------
    // 1. Collect all playable singles
    // -------------------------
    let singles: Vec<Vec<Card>> = hand
        .iter()
        .filter(|c| can_play(c))
        .map(|c| vec![*c])
        .collect();

    // -------------------------
    // 2. Build combos (same number OR same type)
    // -------------------------
    use std::collections::HashMap;

    let mut number_groups: HashMap<Number, Vec<Card>> = HashMap::new();
    let mut type_groups: HashMap<Types, Vec<Card>> = HashMap::new();

    for &card in hand {
        number_groups.entry(card.number).or_default().push(card);
        type_groups.entry(card.type_of_card).or_default().push(card);
    }

    let mut combos: Vec<Vec<Card>> = Vec::new();

    // Only keep groups with 2+ cards AND whose FIRST card can be played
    for group in number_groups.values().chain(type_groups.values()) {
        if group.len() >= 2 {
            // Check if at least one card in the group can start the combo
            if let Some(start_card) = group.iter().find(|c| can_play(c)) {
                let mut combo = group.clone();

                // Put playable card first
                combo.sort_by(|a, b| {
                    let a_playable = *a == *start_card;
                    let b_playable = *b == *start_card;
                    b_playable.cmp(&a_playable)
                });

                combos.push(combo);
            }
        }
    }

    // -------------------------
    // 3. Choose best plays
    // -------------------------
    if !combos.is_empty() {
        // Sort by size descending
        combos.sort_by(|a, b| b.len().cmp(&a.len()));

        let max_size = combos[0].len();

        // Keep only largest combos
        valid_plays = combos
            .into_iter()
            .filter(|c| c.len() == max_size)
            .collect();
    } else {
        // fallback to singles
        valid_plays = singles;
    }

    valid_plays
}