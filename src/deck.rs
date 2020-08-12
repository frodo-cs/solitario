use rand::{seq::SliceRandom, SeedableRng};
use rand::rngs::StdRng;

use crate::card::Card;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub seed: u64
}

impl Deck {
    pub fn new(seed: u64) -> Deck {
        Deck {
            cards: vec![],
            seed: seed
        }
    }

    pub fn gen_deck(&mut self) {
        let mut deck = vec![];

        let cards = ["2Cr", "3Cr","4Cr","5Cr","6Cr","7Cr","8Cr","9Cr","ZCr","JCr","QCr","KCr","ACr",
        "2Tn", "3Tn","4Tn","5Tn","6Tn","7Tn","8Tn","9Tn","ZTn","JTn","QTn","KTn","ATn",
        "2En", "3En","4En","5En","6En","7En","8En","9En","ZEn","JEn","QEn","KEn","AEn",
        "2Dr", "3Dr","4Dr","5Dr","6Dr","7Dr","8Dr","9Dr","ZDr","JDr","QDr","KDr","ADr"];

        for card in cards.iter() {
            let rank = &card[0..1];
            let suit = &card[1..2];
            let color = &card[2..]; 
            deck.push(Card::new(rank, suit, color));
        }

        self.cards = Deck::shuffle(self.seed, deck)
    }

    fn shuffle(seed: u64, mut deck: Vec<Card>) -> Vec<Card> {      
        let mut s = StdRng::seed_from_u64(seed);
        deck.shuffle(&mut s);
        deck
    }
}