use crate::{Card};

/// トランプカードの山札
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for suit in ["h", "d", "c", "s"] {
            for rank in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13] {
                cards.push(Card::new(suit, rank));
            }
        }

        Self { cards: cards }
    }

    pub fn get_cards(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }
}
