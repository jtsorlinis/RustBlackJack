use crate::card::Card;
use crate::deck::Deck;
use std::time;
use std::convert::TryInto;

pub struct CardPile {
    pub m_decks: i32,
    pub m_cards: Vec<*mut Card>,
    pub m_original_cards: Vec<Card>,
    pub seed: u32
}

impl CardPile {
    pub fn new(decks: i32) -> CardPile {
        let mut cp = CardPile {
            m_decks: decks,
            m_original_cards: CardPile::generate_cardpile(decks),
            m_cards: Vec::new(),
            seed: time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).expect("").as_secs().try_into().unwrap()
        };
        
        cp.refresh();

        return cp;
    }

    fn xorshift(&mut self) -> u32 {
        self.seed ^= self.seed << 13;
	    self.seed ^= self.seed >> 17;
	    self.seed ^= self.seed << 5;
	    return self.seed
    }

    fn generate_cardpile(decks: i32) -> Vec<Card> {
        let mut vec: Vec<Card> = Vec::new();
        for _ in 0..decks {
            let mut temp = Deck::new();
            vec.append(&mut temp.m_cards);
        }
        return vec;
    }

    // pub fn print(&self) -> String {
    //     self.m_cards[0].print();
    //     let mut output = String::default();
    //     for card in self.m_cards.iter() {
    //         output += card.print();
    //     }
    //     return output;
    // }

    pub fn shuffle(&mut self) {
        for i in (0..self.m_cards.len()).rev() {
            let j = (self.xorshift() % (i+1) as u32) as usize;
            self.m_cards.swap(i, j);
        }
    }

    pub fn refresh(&mut self) {
        self.m_cards.clear();
        for card in &mut self.m_original_cards {
            self.m_cards.push(card);
        }
    }
    
}

