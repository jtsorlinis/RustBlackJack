pub struct Table {
  pub m_betsize: i32,
  pub m_casinoearnings: f32
}

impl Table {
    pub fn new() -> Table {
        Table {
            m_betsize: 0,
            m_casinoearnings: 0.0
         }
     }
}