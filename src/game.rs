use crate::deck::Deck;
use crate::table::Table;
use std::error::Error;

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
            game.table.stock.push(deck.cards.pop().unwrap());
        }
    }

    pub fn draw_card(&mut self) ->  Result<i32, i32> {
        let card = self.table.stock.pop();
        if card.is_none() {
            return Err(1) 
        }
        self.table.waste.push(card.unwrap());
        Ok(0)     
    }
}
