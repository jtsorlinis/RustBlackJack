use crate::card::Card;
use crate::deck::Deck;
use std::convert::TryInto;
use std::time;

pub struct CardPile {
    pub m_decks: i32,
    pub m_cards: Vec<*mut Card>,
    pub m_original_cards: Vec<*mut Card>,
    pub state: u64,
}

impl CardPile {
    pub fn new(decks: i32) -> CardPile {
        let c = CardPile::generate_cardpile(decks);
        let mut cp = CardPile {
            m_decks: decks,
            m_original_cards: c.clone(),
            m_cards: c,
            state: time::SystemTime::now()
                .duration_since(time::SystemTime::UNIX_EPOCH)
                .expect("")
                .as_secs()
                .try_into()
                .unwrap(),
        };

        cp.refresh();

        return cp;
    }

    // From https://www.pcg-random.org/download.html#minimal-c-implementation
    fn pcg_32(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        self.state = oldstate.wrapping_mul(6364136223846793005).wrapping_add(1);
        let xorshifted: u32 = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot: u32 = (oldstate >> 59) as u32;
        return xorshifted >> rot | (xorshifted << (rot.wrapping_neg() & 31));
    }

    // use nearly divisionless technique found here https://github.com/lemire/FastShuffleExperiments
    fn pcg_32_range(&mut self, s: u32) -> u32 {
        let mut x = self.pcg_32();
        let mut m = x as u64 * s as u64;
        let mut l = m as u32;
        if l < s {
            let t = s.wrapping_neg() % s;
            while l < t {
                x = self.pcg_32();
                m = x as u64 * s as u64;
                l = m as u32;
            }
        }
        return (m >> 32) as u32;
    }

    fn generate_cardpile(decks: i32) -> Vec<*mut Card> {
        let mut vec: Vec<*mut Card> = Vec::new();
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
            let j = self.pcg_32_range(i as u32) as usize;
            self.m_cards.swap(i, j);
        }
    }

    pub fn refresh(&mut self) {
        self.m_cards = self.m_original_cards.clone();
    }
}
