use super::{Number, Types, gamestate};
use rand::seq::SliceRandom;
use rand::thread_rng;

// This trait will assign special xtics to special cards
trait CardEffect {
    fn apply_effect(&self, game_state: &mut gamestate::GameState);
    fn counter_effect (&self, game_state: &mut gamestate::GameState); 
}




#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub number: Number,
    pub type_of_card: Types,
}

impl Card {
    pub fn new(number: Number, type_of_card: Types) -> Self {
        Card { number, type_of_card }
    }

    pub fn display(&self) {
        println!("Card: {:?} of {:?}", self.number, self.type_of_card);
    }
}


impl CardEffect for Card {
    fn apply_effect(&self, game_state: &mut gamestate::GameState) {
             match self.number {
            Number::Jump => {
                println!("Jump card played!.");
                // Implement logic to skip computer's turn
            }
            Number::Question => {
                println!("Question card played!");
                // Implement logic for computer to answer a question
            }
            Number::Kickback => {
                println!("Kickback card played!");
                // Implement logic for computer to draw a card
            }
            Number::Ace => {
                println!("Call the card you want to be placed next");
                println!("1. Heart");
                println!("2. Spade"); 
                println!("3. Flower");
                println!("4. Diamond");

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                let choice: u32 = input.trim().parse().expect("Please enter a number");

                let card = match choice {
                        1 => Card::new(self.number, Types::Heart),
                        2 => Card::new(self.number, Types::Spade),
                        3 => Card::new(self.number, Types::Flower),
                        4 => Card::new(self.number, Types::Diamond),
                        _ => {
                            println!("Invalid choice, defaulting to Heart.");
                            Card::new(self.number, Types::Heart)
                        }
                    };

                if does_other_player_have_that_card(game_state, &card) {
                      println!("Are you placing or picking")
                } else {
                      
                }



            }

            Number::Eight => {
                println!("Question placed, cover or pick");

                
                // Implement logic for player to change the suit
            }

            Number::Three => {
                 counter_or_pick_two_or_three(game_state, self);
            }

            Number::Two => {
                 counter_or_pick_two_or_three(game_state, self);
            }
            _ => {
                // No special effect for other cards
            }

        }
    }


    /// This fuction will 'respond' to the effects of the top card
    fn counter_effect(&self, game_state: &mut gamestate::GameState) {
             match self.number {
            Number::Jump => {
                println!("Countering the jump card.");

                // Implement logic to skip computer's turn
            }
            Number::Question => {
                println!("Countering the question card.");
                // Implement logic for computer to answer a question
            }
            Number::Kickback => {
                println!("Countering the kickback card.");
                // Implement logic for computer to draw a card
            }
            Number::Ace => {
                println!("Countering the ace card.");
                // Implement logic for player to choose an action
            }

            Number::Eight => {
                println!("Countering the eight card.");
                // Implement logic for player to change the suit
            }

            Number::Three => {
                println!("Countering the three card.");
                // Implement logic for computer to draw three cards
            }

            Number::Two => {
                println!("Countering the two card.");
                // Implement logic for computer to draw two cards
            }
            _ => {
                // No special effect for other cards
            }

        }
    }
}





pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// Create a full deck
    pub fn create_deck() -> Self {
        let mut cards = Vec::new();

        let types = [
            Types::Heart,
            Types::Spade,
            Types::Flower,
            Types::Diamond,
        ];

        let numbers = [
            Number::Ace, Number::Two, Number::Three, Number::Four, Number::Five,
            Number::Six, Number::Seven, Number::Eight, Number::Nine, Number::Ten,
            Number::Jump, Number::Question, Number::Kickback,
        ];

        for t in types.iter() {
            for n in numbers.iter() {
                cards.push(Card::new(n.clone(), t.clone()));
            }
        }
        
        Deck { cards }
    }
  
    /// Shuffle the deck
    pub fn shuffle_cards(&mut self)  {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);

    }

    /// Display deck (debugging)
    pub fn display_deck(&self) {
        for card in &self.cards {
            card.display();
        }
    }

    /// Draw top card
    pub fn get_card_on_top(&mut self) -> Option<Card> {
        self.cards.pop()  
    }
}


pub fn does_other_player_have_that_card(game_state: &gamestate::GameState, card: &Card) -> bool {
    game_state.player_hand.iter().any(|c| c.type_of_card == card.type_of_card)
}

// i want this counter or pick fn implemeted for picking both 2 and 3 cards
// do it

pub fn counter_or_pick_two_or_three(game_state: &mut gamestate::GameState, card: &Card) {
    println!("Do you want to counter or pick?");
    println!("1. Counter");
    println!("2. Pick");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice: u32 = input.trim().parse().expect("Please enter a number");

    match choice {
        1 => {
            card.counter_effect(game_state);
        }
        2 => {
            let cards_to_draw = match card.number {
                Number::Two => 2,
                Number::Three => 3,
                _ => 0,
            };
            for _ in 0..cards_to_draw {
                if let Some(new_card) = game_state.get_top_card() {
                    game_state.player_hand.push(new_card);
                }
            }
        }
        _ => {
            println!("Invalid choice, defaulting to pick.");
            let cards_to_draw = match card.number {
                Number::Two => 2,
                Number::Three => 3,
                _ => 0,
            };
            for _ in 0..cards_to_draw {
                if let Some(new_card) = game_state.get_top_card() {
                    game_state.player_hand.push(new_card);
                }
            }
        }
    }
}



// Not fully implemented, just a placeholder for now
fn question(game_state: &mut gamestate::GameState) {
       println!("Answer the question to avoid picking cards!");

       println!("1. Answer the question");
       println!("2. Pick a card");

       let mut input = String::new(); 
       std::io::stdin().read_line(&mut input).expect("Failed to read line");
       let choice: u32 = input.trim().parse().expect("Please enter a number");
       match choice {
                1 => {
                    println!("Correct answer! No cards drawn.");
                }
                2 => {
                    if let Some(new_card) = game_state.draw_card() {
                        game_state.player_hand.push(new_card);
                    }
                }
                _ => {
                    println!("Invalid choice, defaulting to pick.");
                    if let Some(new_card) = game_state.draw_card() {
                        game_state.player_hand.push(new_card);
                    }
                }
            }
}


fn jump() {
    println!("Jump card played! Skipping computer's turn.");
    // Implement logic to skip computer's turn

    
}