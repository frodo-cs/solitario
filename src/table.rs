use crate::card::Card;

#[derive(Debug)]
pub struct Table {
    pub tableau: [Vec<Card>; 7],
    pub foundation: [Vec<Card>; 4],
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub c: i32
}

impl Table {
    pub fn new() -> Table {
        Table {
            tableau: [vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            foundation: [vec![], vec![], vec![], vec![]],
            stock: vec![],
            waste: vec![],
            c: 7
        }
    }
}