use super::{Card, cardmodule, player, computer, gamestate};

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

pub struct GameState {
    // represents cards given to the player
    pub player_hand: Vec<Card>, 
    // represents cards assigned to the computer   
    pub computer_hand: Vec<Card>, 
    // this refers to the cards that is placed at the top when the game starts 
    pub top_card: Card, 
    // the cards available to pick from *before and after assignment             
    pub deck: Vec<Card>, 
    // the cards that have been thrown either by the player or the compute            
    pub pile: Vec<Card>,             

    // understand that the top card is part of the pile, and is gonna be the first card on the pile
    // therefore we can safely assume that the pile is gonna have a card and for that reason we dont 
    // use the option enum as the type of pile
} 

impl GameState {
    pub fn initialize_game_state() -> Self {
        // Create and shuffle deck
        let mut deck = cardmodule::Deck::create_deck();
        deck.shuffle_cards();

        // Deal cards from SAME deck
        let mut cards = deck.cards;

        let player_hand = player::assign_player_cards(&mut cards);
        player::print_player_cards(&player_hand);

        let computer_hand = computer::assign_computer_cards(&mut cards);
        computer::print_computer_cards(&computer_hand);

        // Place top card
        let top_card;

       // this is done first, so there WILL ALWAYS exist a top card, we dont use an option enum 
        if let Some(card) = cards.pop() {
            top_card = card;
        } else {
            // This situation is NOT going to occur because at the start of the game there will be 
            // a deck of cards
            panic!("Failed to assign the top card")
        }

        GameState {
            player_hand,
            computer_hand,
            top_card,
            deck: cards,
            pile: vec![top_card]
        }
    }

    /// Draw a new top card from the deck
    pub fn draw_card(&mut self) -> Option<Card> {
        self.deck.pop()
    }

    /// Replace the current top card
    pub fn set_top_card(&mut self, card: Card) {
        self.top_card = card;
    }

    /// Get the current top card
    pub fn get_top_card(&self) -> Option<Card> {
        Some(self.top_card)
    }

    pub fn check_win(game_state: &Self) -> Option<&str> {
        if game_state.player_hand.is_empty() {
            return Some("Player wins!");
        }

        if game_state.computer_hand.is_empty() {
            return Some("Computer wins!");
        }

        None
}
}

