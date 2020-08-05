use crate::card::Card;
use crate::game::Game;

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

    pub fn waste_to_tableau(&mut self, col: usize) {
        let card = self.waste.pop().unwrap();
        self.tableau[col].push(card);   
    }

    pub fn waste_to_stock(&mut self) {
        let mut card : Card;
        while !self.waste.is_empty() {
            card = self.waste.pop().unwrap();
            card.flip();
            self.stock.push(card);
        }

        if !self.stock.is_empty(){
            self.stock_to_waste();
        }      
    }

    pub fn stock_to_waste(&mut self) {
        let mut card = self.stock.pop().unwrap();
        card.flip();
        self.waste.push(card);
    }

    pub fn tableau_to_tableau(&mut self, col1: usize, col2: usize) {
        let card = self.tableau[col1].pop().unwrap(); 
        self.tableau[col2].push(card);

        if self.tableau[col1].len() > 0 {
            let mut card3 = self.tableau[col1].pop().unwrap();
            card3.flip();
            self.tableau[col1].push(card3);
        }
    }

    pub fn tableau_to_foundation(&mut self, col: usize, rank: usize) {
        let card = self.tableau[col].pop().unwrap();
        self.foundation[rank].push(card);

        if self.tableau[col].len() > 0 {
            let mut card2 = self.tableau[col].pop().unwrap();
            card2.flip();
            self.tableau[col].push(card2);
        }
    }
}