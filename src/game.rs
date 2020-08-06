use crate::deck::Deck;
use crate::table::Table;

#[derive(Debug)]
pub struct Game {
    table: Table,
    finished: bool,
    history: Vec<(&'static str, usize, usize, bool)>
}

impl Game {
    pub fn new(seed: u64) -> Game {
        let mut game = Game {
            table: Table::new(),
            finished: false,
            history: vec![]
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

    pub fn play(&mut self, col: usize){

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
