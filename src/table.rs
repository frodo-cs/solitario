use crate::card::Card;

pub struct Table {
    pub table: [Vec<Card>; 7],
    pub suits: [Vec<Card>; 4],
    pub draw: Vec<Card>,
    pub discard: Vec<Card>,
    pub c: i32
}

impl Table {
    pub fn new() -> Table {
        Table {
            table: [vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            suits: [vec![], vec![], vec![], vec![]],
            draw: vec![],
            discard: vec![],
            c: 7
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.discard);
    }
}