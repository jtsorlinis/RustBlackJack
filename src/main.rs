mod deck;
mod card;
mod rand;
mod cardpile;
mod player;
mod table;

use cardpile::CardPile;
use player::Player;
use table::Table;

fn main() {
    let mut table = Table {..Table::new()};

    let mut player = Player{m_playernum: "3", ..Player::new(&mut table, 10)};
    println!("{}", player.m_playernum);
    
    // let mut _c = CardPile {..CardPile::new(8)};
    // _c.shuffle();
    // println!("{}", _c.print());
}