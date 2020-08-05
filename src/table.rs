use std::fmt;
use crate::card::Card;

#[derive(Debug)]
pub struct Table {
    pub tableau: [Vec<Card>; 7],
    pub foundation: [Foundation; 4],
    pub stock: Pile,
    pub waste: Pile,
}

impl Table {
    pub fn new() -> Table {
        Table {
            tableau: [vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            foundation: [ Foundation::default(), Foundation::default(), Foundation::default(), Foundation::default() ],
            stock: Pile { cards: vec![]},
            waste: Pile { cards: vec![]},
        }
    }

    pub fn waste_to_tableau(&mut self, col: usize) {
        let card = self.waste.cards.pop().unwrap();
        self.tableau[col].push(card);   
    }

    pub fn waste_to_stock(&mut self) {
        let mut card : Card;
        while !self.waste.cards.is_empty() {
            card = self.waste.cards.pop().unwrap();
            card.flip();
            self.stock.cards.push(card);
        }

        if !self.stock.cards.is_empty(){
            self.stock_to_waste();
        }      
    }

    pub fn stock_to_waste(&mut self) {
        let mut card = self.stock.cards.pop().unwrap();
        card.flip();
        self.waste.cards.push(card);
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
        self.foundation[rank].cards.push(card);

        if self.tableau[col].len() > 0 {
            let mut card2 = self.tableau[col].pop().unwrap();
            card2.flip();
            self.tableau[col].push(card2);
        }
    }
}

#[derive(Debug)]
pub struct Pile {
    pub cards: Vec<Card>
}

#[derive(Debug)]
pub struct Foundation {
    pub cards: Vec<Card>
}

impl Default for Foundation {
    fn default () -> Foundation {
        Foundation{ cards: vec![] }
    }
}

// TODO
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t  \t{}\t{}\t{}\t{}\n", self.stock, self.waste, self.foundation[0], self.foundation[1], self.foundation[2], self.foundation[3]);
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}", self.stock, self.stock, self.stock, self.stock, self.stock, self.stock, self.stock)
    }
}

impl fmt::Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.cards.is_empty() {
            write!(f, "___")
        } else {
            write!(f, "{}", self.cards.last().unwrap())
        }     
    }
}

impl fmt::Display for Foundation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.cards.is_empty() {
            write!(f, "{}", self.cards.last().unwrap())
        } else {
            write!(f, "___")
        }  
    }
}


