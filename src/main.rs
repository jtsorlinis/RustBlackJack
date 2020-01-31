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
    //TODO: implement main
    let mut table1 = Table {..Table::new(5,8,10,40,true)};
    table1.start_round();
    
    // let mut _c = CardPile {..CardPile::new(8)};
    // _c.shuffle();
    // println!("{}", _c.print());
}