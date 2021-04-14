pub struct Card {
    pub m_rank: &'static str,
    pub m_suit: &'static str,
    pub m_facedown: bool,
    pub m_count: i32,
    pub m_value: i32,
    pub m_isace: bool,
}

impl Card {
    pub fn new(rank: &'static str, suit: &'static str) -> Card {
        Card {
            m_rank: rank,
            m_suit: suit,
            m_facedown: false,
            m_count: Card::count(rank),
            m_value: Card::evaluate(rank),
            m_isace: Card::isace(rank),
        }
    }

    pub fn print(&self) -> &str {
        if self.m_facedown {
            return "X";
        } else {
            return self.m_rank;
        }
    }

    fn isace(rank: &str) -> bool {
        return rank == "A";
    }

    fn evaluate(rank: &str) -> i32 {
        if rank == "J" || rank == "Q" || rank == "K" {
            return 10;
        } else if rank == "A" {
            return 11;
        } else {
            return rank.parse::<i32>().unwrap();
        }
    }

    fn count(rank: &str) -> i32 {
        if rank == "10" || rank == "J" || rank == "Q" || rank == "K" || rank == "A" {
            return -1;
        } else if rank == "7" || rank == "8" || rank == "9" {
            return 0;
        } else {
            return 1;
        }
    }
}
