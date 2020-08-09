use std::io;
use crate::deck::Deck;
use crate::table::Table;
use crate::rules;

#[derive(Debug)]
pub struct Game {
    table: Table,
    finished: bool,
    history: Vec<(&'static str, usize, usize, usize, usize, bool)>,
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
            self.history.push(("draw", 0, 0, 0, 0, true));
        } else {
            self.table.waste_to_stock();
            self.history.push(("draw", 0, 0, 0, 0, false));
        }
    }

    pub fn print_table(&mut self){
        println!("{}", self.table);
    }

    pub fn play_card(&mut self, col: usize){
        let mut possibilities: Vec<(&'static str, usize, usize, usize)> = vec![]; // (where, col origin, row origin, col destination)

        // card from tableau to tableau
        for prob in rules::tableau_check(&self.table, col){
            possibilities.push(prob);
        }

        // card from tableau to foundation
        match self.table.tableau[col].last() {
            Some(c) => {        
                for prob in rules::foundation_check(&self.table, c){
                    possibilities.push((prob.0, col, prob.2, prob.3));
                }
            },
            None => ()
        }

        // card from waste to tableau
        if self.table.waste.cards.len() > 0 {
            let column =  &self.table.tableau[col];
            let card = self.table.waste.cards.last().unwrap();
            if rules::waste_check(&column, card) {
                possibilities.push(("pila a columna", 0, 0, col))                
            }         
        }

        // card from waste to foundation
        match self.table.waste.cards.last() {
            Some(c) => {        
                for prob in rules::foundation_check(&self.table, c){
                    possibilities.push(("pila a base", prob.1, prob.2, prob.3));
                }
            },
            None => ()
        }


        if possibilities.len() == 1 {
            self.selection(possibilities[0]);
        } else if possibilities.len() > 1 {
            let mut input = String::new();
            let none = String::from((possibilities.len()+1).to_string());
            while input.as_str().trim() != none {

                for i in 0..possibilities.len() {
                    println!("{}. {} {} a {}", i+1, possibilities[i].0, possibilities[i].1+1, possibilities[i].3+1)
                }
                println!("{}. Ninguna", none);
                input = read_input();
                match input.as_str().trim() {
                    a => match a.parse::<usize>() {
                        Ok(ok) => {
                            if ok > 0 && ok <= possibilities.len(){
                                self.selection(possibilities[ok-1]);
                                input = String::from((possibilities.len()+1).to_string());
                            } else {
                                println!("No es una selección válida")
                            }
                        },                      
                        Err(_) => println!("No es una selección válida")
                    }
                }
            }
        }
    }

    fn selection(&mut self, possibilities: (&'static str, usize, usize, usize)){
        match possibilities.0 {
            "pila a columna" => {
                self.waste_card(possibilities);
            },
            "columna a columna" => {
                self.tableau_card(possibilities);         
            },
            "columna a base" => {
                self.foundation_card(possibilities);
            },
            "pila a base" => {
                self.base_card(possibilities);
            },
            _ => ()
        }      
    }

    fn waste_card(&mut self, possibilities: (&'static str, usize, usize, usize)){
        self.table.waste_to_tableau(possibilities.3); 
        self.history.push(("waste_to_tableau", possibilities.1, possibilities.2, possibilities.3,  0, false))
    }

    fn tableau_card(&mut self, possibilities: (&'static str, usize, usize, usize)){
        let flipped = self.table.tableau_to_tableau(possibilities.1, possibilities.2, possibilities.3);
        self.history.push(("tableau_to_tableau", possibilities.1, possibilities.2, possibilities.3, flipped.0, flipped.1))
    }

    fn foundation_card(&mut self, possibilities: (&'static str, usize, usize, usize)){
        let flipped = self.table.tableau_to_foundation(possibilities.1, possibilities.3);
        self.history.push(("tableau_to_foundation", possibilities.1, possibilities.2, possibilities.3,  0, flipped))      
    }

    fn base_card(&mut self, possibilities: (&'static str, usize, usize, usize)){
        self.table.waste_to_foundation(possibilities.3);
        self.history.push(("waste_to_foundation", possibilities.1, possibilities.2, possibilities.3,  0, false))      
    }

    pub fn undo(&mut self) {
        if self.history.len() > 0 {
            let previous = self.history.pop().unwrap();
            // (Move, col origin, row origin, col destination, dif, flag)
            println!("{:?}", previous);
            match previous.0 {
                "waste_to_tableau" => {
                    self.table.waste_to_tableau_undo(previous.3)
                },
                "waste_to_stock" => {
                    self.table.waste_to_stock_undo()
                },
                "waste_to_foundation" => {
                    self.table.waste_to_foundation_undo(previous.3)
                },               
                "stock_to_waste" => {
                    self.table.stock_to_waste_undo()
                },
                "tableau_to_tableau" => {
                    self.table.tableau_to_tableau_undo(previous.1, previous.3, previous.4, previous.5)
                },
                "tableau_to_foundation" => {
                    self.table.tableau_to_foundation_undo(previous.1, previous.3, previous.5)
                },
                "draw" => {
                    if previous.5 {
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

    pub fn check_done(&mut self) -> bool {
        self.table.foundation_full()
    }
}

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            input
        }
        Err(_) => {
            String::from("Error")
        }
    }
}

