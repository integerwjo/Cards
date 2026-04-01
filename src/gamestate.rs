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
        cards = cards.shuffle_cards();

        GameState {
            human_player: Player {
                cards_in_hand: assign_human_cards(),
                is_turn_to_play: true,
            },
            computer_player: Player {
                cards_in_hand: assign_computer_cards(),
                is_turn_to_play: false,
            },
            top_card: place_first_top_card(),
            deck: Deck { cards },
        }
    }


    
    pub fn place_first_top_card(&mut self) -> Card {
        self.deck.cards.pop().unwrap()
    }


    pub fn set_top_card(&mut self, card: Card) {
        self.top_card = card;
    }

    /// Get the current top card 
    /* 
    pub fn get_top_card(card: &self) -> Option<Card> {
        Some(self.top_card)
    } */

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


pub fn assign_human_cards(gamestate: &mut GameState) -> Vec<Card> {
        for _ in 0..4 {
            gamestate.human_player.cards_in_hand.push(gamestate.deck.cards.pop().unwrap());
        }
        gamestate.human_player.cards_in_hand.clone()
}

pub fn assign_computer_cards(gamestate: &mut GameState) -> Vec<Card> {
        for _ in 0..4 {
            gamestate.computer_player.cards_in_hand.push(gamestate.deck.cards.pop().unwrap());
        }
        gamestate.computer_player.cards_in_hand.clone()

}



