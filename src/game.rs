use crate::deck::Deck;
use crate::table::Table;

#[derive(Debug)]
pub struct Game {
    pub table: Table
}

impl Game {
    pub fn new(seed: u64) -> Game {
        let mut game = Game {
            table: Table::new()
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

    pub fn test(&mut self){
        let col1 = 2;
        let col2 = 1;

        println!("{}", self.table);

        self.table.stock_to_waste();
        self.table.stock_to_waste();
        self.table.stock_to_waste();
        self.table.stock_to_waste();
        self.table.stock_to_waste();
        self.table.stock_to_waste();

        println!("{}", self.table);

        self.table.waste_to_tableau(col1);
        self.table.waste_to_tableau(col2);
        self.table.waste_to_tableau(col1);
        self.table.tableau_to_tableau(col1, col2);
        // self.table.waste_to_stock();
        self.table.tableau_to_foundation(col1, col2);
        self.table.tableau_to_foundation(col2, col1);

        println!("{}", self.table);
    }
}
