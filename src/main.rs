mod deck;
mod card;
mod rand;
mod cardpile;
mod player;
mod table;
mod dealer;

use cardpile::CardPile;
use player::Player;
use table::Table;

fn main() {
    let mut table = Table {..Table::new(5,8,10,40,true)};

    let mut player = Player{m_playernum: "1", ..Player::new(&mut table, 10)};
    println!("{}", player.print());
    
    // let mut _c = CardPile {..CardPile::new(8)};
    // _c.shuffle();
    // println!("{}", _c.print());
}