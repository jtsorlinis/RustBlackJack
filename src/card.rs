pub struct Card {
    pub m_rank: String,
    pub m_suit: String
}

impl Card {
    pub fn print(&self) {
        println!("{} of {}", self.m_rank, self.m_suit);
    }
}