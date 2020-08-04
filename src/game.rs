use crate::deck::Deck;
use crate::table::Table;
use crate::card::Card;

pub fn start(){
    let _ranks = ["2", "3","4","5","6","7","8","9","Z","J","Q","K","A"]; 
    let _suits = ["Cr", "Tn", "En", "Dr"];

    let deck = Deck::new(0);
    let mut table = Table::new();

    let mut card = Card::new("2".to_string(), "Cr".to_string());
    println!("{:?}", card);
    card.flip();
    println!("{:?}", card);

}

fn set_up(mut deck: Deck, mut cards: [Vec<Card>; 7]) {
    for card in deck.cards.iter() {

    }
}