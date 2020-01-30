mod card;

fn main() {
    use card::Card;
    let _c = Card{m_rank: "S".to_string(),m_suit: "Hearts".to_string()};
    _c.print();
}