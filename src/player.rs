use crate::table::Table;
use crate::card::Card;

static MAXSPLITS: i32 = 10;

pub struct Player<'a> {
    pub m_value: i32,
    pub m_earnings: f32,
    pub m_aces: i32,
    pub m_issoft: bool,
    pub m_splitcount: i32,
    pub m_isdone: bool,
    pub m_splitfrom: Option<&'a Player<'a>>,
    pub m_betmult: f32,
    pub m_hasnatural: bool,
    pub m_table: &'a mut Table,
    pub m_initialbet: i32,
    pub m_originalbet: i32,
    pub m_hand: Vec<Card>,
    pub m_playernum: &'a str
}

impl<'a> Player<'a> {
    pub fn new(table: &'a mut Table, betsize: i32) -> Player<'a> {
       Player {
            m_value: 0,
            m_earnings: 0.0,
            m_aces: 0,
            m_issoft: false,
            m_splitcount: 0,
            m_isdone: false,
            m_splitfrom: None,
            m_betmult: 1.0,
            m_hasnatural: false,
            m_table: table,
            m_initialbet: betsize,
            m_originalbet: betsize,
            m_hand: Vec::new(),
            m_playernum: ""
        }
    }

    fn getinitialbet(table: Table) -> i32 {
        return table.m_betsize;
    }

    pub fn doubleBet(&mut self) {
        self.m_betmult = 2.0;
    }

    pub fn resetHand(&mut self) {
        self.m_hand.clear();
        self.m_value = 0;
        self.m_aces = 0;
        self.m_issoft = false;
        self.m_splitcount = 0;
        self.m_isdone = false;
        self.m_betmult = 1.0;
        self.m_hasnatural = false;
        self.m_initialbet = self.m_originalbet
    }

    pub fn canSplit(&self) -> &str {
        if self.m_hand.len() == 2 && self.m_hand[0].m_rank == self.m_hand[1].m_rank && self.m_splitcount < MAXSPLITS {
            return self.m_hand[0].m_rank;
        } else {
            return "";
        }
    }

    pub fn win(&'a mut self, mult: f32) {
        if self.m_splitfrom.is_some() {
            self.m_splitfrom.unwrap().win(mult);
        } else {
            self.m_earnings += self.m_initialbet as f32 * self.m_betmult * mult;
            self.m_table.m_casinoearnings -= self.m_initialbet as f32 * self.m_betmult * mult;
        }
    }
}