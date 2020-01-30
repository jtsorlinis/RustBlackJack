use crate::player::Player;
use crate::cardpile::CardPile;
use crate::dealer::Dealer;

pub struct Table {
  pub m_verbose: bool,
  pub m_betsize: i32,
  pub m_players: Vec<Player>,
  pub m_numofdecks: i32,
  pub m_cardpile: CardPile,
  pub m_mincards : i32,
  pub m_dealer: Dealer,
  pub m_currentplayer: i32,
  pub m_casinoearnings: f32,
  pub m_runningcount: i32,
  pub m_truecount: f32,
  pub m_numplayers: i32
}

impl Table {
    pub fn new(numplayers: i32, numdecks: i32, betsize: i32, mincards: i32, verbose: bool) -> Table {
        Table {
            m_verbose: verbose,
            m_betsize: betsize,
            m_players: Table::fill(numplayers, betsize),
            m_numofdecks: numdecks,
            m_cardpile: CardPile::new(numdecks),
            m_mincards: mincards,
            m_dealer: Dealer::new(),
            m_currentplayer: 0,
            m_casinoearnings: 0.0,
            m_runningcount: 0,
            m_truecount: 0.0,
            m_numplayers: numplayers
         }
     }

     fn fill(numplayers: i32, betsize: i32) -> Vec<Player> {
       let mut temp: Vec<Player> = Vec::new();
       for i in 0..numplayers {
          temp.push(Player::new(&(i+1).to_string(), betsize));
       }
        return temp;
     }

     pub fn print(&self) {
       for player in self.m_players.iter() {
          println!("{}", player.print());
       }
       println!();
     }
}