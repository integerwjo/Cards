use super::{Card, cardmodule, player, computer, gamestate, Player};
use crate::cardmodule::Deck;

/// represents all the types of variants a card is
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Types {
    Heart,
    Spade,
    Flower,
    Diamond,
}

/// This doesn't represent a 'real' number
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Number {
    Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jump, Question, Kickback,
}

pub struct  GameState{
    pub human_player: Player,
    pub computer_player: Player,
    pub top_card: Card,
    pub deck: Deck,

}

impl GameState {
   
    pub fn new() -> Self {
        let mut cards = Deck::create_deck();
        cardmodule::shuffle_cards(&mut cards);

        GameState {
            human_player: Player {
                cards_in_hand: assign_cards(&mut cards),
                is_turn_to_play: true,
            },
            computer_player: Player {
                cards_in_hand: assign_cards(&mut cards),
                is_turn_to_play: false,
            },
            top_card: place_initial_top_card(&mut cards),
            deck: cards,
        }
    }
    

    pub fn check_win(game_state: &Self) -> Option<&str> {
        if game_state.human_player.cards_in_hand.is_empty() {
            return Some("Player wins!");
        }

        if game_state.computer_player.cards_in_hand.is_empty() {
            return Some("Computer wins!");
        }

        None
}

    
    
}

pub fn place_first_top_card(deck: &mut Deck) -> Card {
        deck.cards.pop().unwrap()
    }


    pub fn place_initial_top_card(deck: &mut Deck) -> Card {
        deck.cards.pop().unwrap()
    }

    /// Get the current top card 
    /* 
    pub fn get_top_card(card: &self) -> Option<Card> {
        Some(self.top_card)
    } */


pub fn assign_cards(deck: &mut Deck) -> Vec<Card> {
        let mut cards_in_hand = Vec::new();
        for _ in 0..4 {
            cards_in_hand.push(deck.cards.pop().unwrap());
        }
        cards_in_hand
}


pub fn pick_top_card(deck: &mut Deck) -> Option<Card> {
    if let Some(card) = deck.cards.pop() {
        Some(card)
    } else {
        None
    }
}



