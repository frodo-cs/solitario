use crate::deck::Deck;
use crate::table::Table;
use crate::rules;

#[derive(Debug)]
pub struct Game {
    table: Table,
    finished: bool,
    history: Vec<(&'static str, usize, usize, bool)>,
    possible: Vec<()>
}

impl Game {
    pub fn new(seed: u64) -> Game {
        let mut game = Game {
            table: Table::new(),
            finished: false,
            history: vec![],
            possible: vec![]
        };

        let mut deck = Deck::new(seed);

        Game::setup_tableu(&mut game, &mut deck);
        Game::setup_stock(&mut game, &mut deck);
        game
    }

    fn setup_tableu(game: &mut Game, deck: &mut Deck){
        for i in 0..7 {
            for j in 0..i+1 {
                game.table.tableau[i].push(deck.cards.pop().unwrap());
                if i == j {
                   game.table.tableau[i][j].flip();
                }
            }
        }
    }

    fn setup_stock(game: &mut Game, deck: &mut Deck){
        while deck.cards.len() > 0 {
            game.table.stock.cards.push(deck.cards.pop().unwrap());
        }
    }

    pub fn draw_card(&mut self){
        if self.table.stock.cards.len() > 0 {
            self.table.stock_to_waste();
            self.history.push(("draw", 0, 0, true));
        } else {
            self.table.waste_to_stock();
            self.history.push(("draw", 0, 0, false));
        }
    }

    pub fn print_table(&mut self){
        println!("{}", self.table);
    }

    pub fn play_card(&mut self, col: usize){
        let mut possibilities: Vec<(&'static str, usize, usize)> = vec![];

        let column =  self.table.tableau[col].last();
        match column {
            Some(c) => {
                // card from tableau to tableau FIX, check the first card that is flipped not the last
                println!("Card: {}", c);
                println!("tableau_check");
                for prob in rules::tableau_check(&self.table, c){
                    let p = (prob.0, col, prob.2);
                    possibilities.push(p);
                }
                println!("foundation_check");
                // card from tableau to foundation
                for prob in rules::foundation_check(&self.table, c){
                    let p = (prob.0, col, prob.2);
                    possibilities.push(p);
                }
            },
            None => ()
        }

        // card from waste to tableau
        if self.table.waste.cards.len() > 0 {
            println!("waste_check");
            let column =  &self.table.tableau[col];
            let card = self.table.waste.cards.last().unwrap();
            if rules::waste_check(&column, card) {
                let p = ("waste", 0, col);
                possibilities.push(p)                
            }         
        }

        if possibilities.len() == 1 {
            match possibilities[0].0 {
                "waste" => {
                    self.waste_card(possibilities[0]);
                },
                "column" => {
                    self.tableau_card(possibilities[0]);         
                },
                "foundation" => {
                    self.foundation_card(possibilities[0]);
                },
                _ => ()
            }
        } else {

        }
    }

    fn waste_card(&mut self, possibilities: (&'static str, usize, usize)){
        self.table.waste_to_tableau(possibilities.2); 
        self.history.push(("waste_to_tableau", possibilities.1, possibilities.2, false))
    }

    fn tableau_card(&mut self, possibilities: (&'static str, usize, usize)){
        let flipped = self.table.tableau_to_tableau(possibilities.1, possibilities.2);
        self.history.push(("tableau_to_tableau", possibilities.1, possibilities.2, flipped))
    }

    fn foundation_card(&mut self, possibilities: (&'static str, usize, usize)){
        let flipped = self.table.tableau_to_foundation(possibilities.1, possibilities.2);
        self.history.push(("tableau_to_foundation", possibilities.1, possibilities.2, flipped))      
    }

    pub fn undo(&mut self) {
        if self.history.len() > 0 {
            let previous = self.history.pop().unwrap();
            // (Move, from col, to col, flag)
            match previous.0 {
                "waste_to_tableau" => {
                    self.table.waste_to_tableau_undo(previous.2)
                },
                "waste_to_stock" => {
                    self.table.waste_to_stock_undo()
                },
                "stock_to_waste" => {
                    self.table.stock_to_waste_undo()
                },
                "tableau_to_tableau" => {
                    self.table.tableau_to_tableau_undo(previous.1, previous.2, previous.3)
                },
                "tableau_to_foundation" => {
                    self.table.tableau_to_foundation_undo(previous.1, previous.2, previous.3)
                },
                "draw" => {
                    if previous.3 {
                        self.table.stock_to_waste_undo()
                    } else {
                        self.table.waste_to_stock_undo()
                    }
                }
                _ => ()
            }
        } else {
            println!("No más movimientos previos");
        }
    }
}
