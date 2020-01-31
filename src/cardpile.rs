use crate::card::Card;
use crate::deck::Deck;
use crate::rand;
use std::time;
use std::convert::TryInto;


pub struct CardPile {
    pub m_decks: i32,
    pub m_cards: Vec<Card>,
    pub m_original_cards: Vec<Card>
}

impl CardPile {
    pub fn new(decks: i32) -> CardPile {
        CardPile {
            m_decks: decks,
            m_cards: CardPile::generate_cardpile(decks),
            m_original_cards: CardPile::generate_cardpile(decks)
        }
    }

    fn generate_cardpile(decks: i32) -> Vec<Card> {
        let mut vec: Vec<Card> = Vec::new();
        for _ in 0..decks {
            let mut temp = Deck::new();
            vec.append(&mut temp.m_cards);
        }
        return vec;
    }

    pub fn print(&self) -> String {
        self.m_cards[0].print();
        let mut output = String::default();
        for card in self.m_cards.iter() {
            output += card.print();
        }
        return output;
    }

    pub fn shuffle(&mut self) {
        //TODO: Move seed to only happen once
        let now = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).expect("");
        let mut rng = rand::Rand::new(now.as_secs().try_into().unwrap());
        rng.shuffle(&mut self.m_cards);
    }

    pub fn refresh(&mut self) {
        //TODO: Implement cardpile as reference to cards for speed
        self.m_cards = self.m_original_cards.clone();
        self.shuffle();
    }
    
}

