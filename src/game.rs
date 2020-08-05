use crate::deck::Deck;
use crate::table::Table;

#[derive(Debug)]
pub struct Game {
    table: Table,
    finished: bool
}

impl Game {
    pub fn new(seed: u64) -> Game {
        let mut game = Game {
            table: Table::new(),
            finished: false
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

    fn run(&mut self) {
        while(!self.finished){
            
        }
    }

    pub fn test(&mut self){
        println!("{}", self.table);
    }
}
