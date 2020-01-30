mod deck;
mod card;
mod rand;
mod cardpile;

use cardpile::CardPile;

fn main() {
    
    let mut _c = CardPile {..CardPile::new(8)};
    _c.shuffle();
    println!("{}", _c.print());
}