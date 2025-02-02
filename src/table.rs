#![allow(unused_must_use)]
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

    pub fn foundation_full(&mut self) -> bool {
        for foundation in self.foundation.iter() {
            if foundation.cards.len() != 13 {
                return false
            }
        }
        true
    }

    pub fn waste_to_tableau(&mut self, col: usize) {
        let card = self.waste.cards.pop().unwrap();
        self.tableau[col].push(card);   
    }

    pub fn waste_to_tableau_undo(&mut self, col: usize){
        let card = self.tableau[col].pop().unwrap();
        self.waste.cards.push(card);
    }

    pub fn waste_to_foundation(&mut self, rank: usize){
        let card = self.waste.cards.pop().unwrap();
        self.foundation[rank].cards.push(card);
    }

    pub fn waste_to_foundation_undo(&mut self, rank: usize){
        let card = self.foundation[rank].cards.pop().unwrap();
        self.waste.cards.push(card);
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

    pub fn waste_to_stock_undo(&mut self) {
        let mut card : Card;
        while !self.stock.cards.is_empty() {
            card = self.stock.cards.pop().unwrap();
            card.flip();
            self.waste.cards.push(card);
        }      
    }

    pub fn stock_to_waste(&mut self) {
        let mut card = self.stock.cards.pop().unwrap();
        card.flip();
        self.waste.cards.push(card);
    }

    pub fn stock_to_waste_undo(&mut self) {
        let mut card = self.waste.cards.pop().unwrap();
        card.flip();
        self.stock.cards.push(card);
    }

    pub fn tableau_to_tableau(&mut self, col1: usize, row: usize, col2: usize) -> (usize, bool) {
        let mut length = self.tableau[col1].len();
        let mut cards: Vec<Card> = vec![];
        let dif = length - row;

        while length > row {   
            let card = self.tableau[col1].pop();
            match card {
                Some(c) => cards.push(c),
                None => ()
            }
            length = length-1;
        }

        cards.reverse();
        self.tableau[col2].append(&mut cards);

        let card2 = self.tableau[col1].last();
        if self.tableau[col1].len() > 0 && card2.unwrap().facing_down() {
            let mut card3 = self.tableau[col1].pop().unwrap();
            card3.flip();
            self.tableau[col1].push(card3);

            return (dif, true)
        }
        (dif, false)
    }

    pub fn tableau_to_tableau_undo(&mut self, col1: usize, col2: usize, dif: usize, flipped: bool) {     
        if flipped {
            let mut card2 = self.tableau[col1].pop().unwrap();
            card2.flip();
            self.tableau[col1].push(card2);
        }

        let mut diff = dif;
        let mut cards: Vec<Card> = vec![];
        
        while diff > 0 {
            let card = self.tableau[col2].pop();
            match card {
                Some(c) => cards.push(c),
                None => ()
            }
            diff = diff-1;
        }

        cards.reverse();

        self.tableau[col1].append(&mut cards);
    }

    pub fn tableau_to_foundation(&mut self, col: usize, rank: usize) -> bool {
        let card = self.tableau[col].pop().unwrap();
        self.foundation[rank].cards.push(card);

        let card3 = self.tableau[col].last();
        if self.tableau[col].len() > 0 && card3.unwrap().facing_down() {
            let mut card2 = self.tableau[col].pop().unwrap();
            card2.flip();
            self.tableau[col].push(card2);
            return true
        }
        false
    }

    pub fn tableau_to_foundation_undo(&mut self, col: usize, rank: usize, flipped: bool) {
        let card = self.foundation[rank].cards.pop().unwrap();

        if flipped {
            let mut card2 = self.tableau[col].pop().unwrap();
            card2.flip();
            self.tableau[col].push(card2);
        }

        self.tableau[col].push(card);
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

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}\t{}\t  \t{}\t{}\t{}\t{}\n\n", self.stock, self.waste, self.foundation[0], self.foundation[1], self.foundation[2], self.foundation[3]);
        for i in 0..largest(self) {
            for j in 0..7 {
                if i < self.tableau[j].len() {
                    write!(f, "{}\t", self.tableau[j][i]);
                } else {
                    write!(f, "  \t");
                }       
            }
            write!(f, "\n");
        }
        write!(f, "")
    }
}

fn largest(t : &Table) -> usize {
    let mut size: usize = 0;
    for c in t.tableau.iter() {
        if c.len() > size { size = c.len() }
    }
    size
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

pub fn table_log(t : &Table) -> String {
    let mut s: String = String::new(); 
    let stock = match t.stock.cards.last() {
        Some(e) => e.to_log(),
        None => "___".to_string()
    };

    s.push_str(format!("\n{}\t{}\t  \t{}\t{}\t{}\t{}\n\n", stock, t.waste, t.foundation[0], t.foundation[1], t.foundation[2], t.foundation[3]).as_str());
    for i in 0..largest(t) {
        for j in 0..7 {
            if i < t.tableau[j].len() {
                s.push_str(format!("{}\t", t.tableau[j][i].to_log()).as_str());
            } else {
                s.push_str("  \t");
            }       
        }
        s.push_str("\n");
    }
    s
}


