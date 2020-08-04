#[derive(Debug)]
pub struct Card {
    rank: String,
    suit: String,
    flipped: bool
}

impl Card {
    pub fn new(rank: String, suit: String) -> Card {
        Card {
            rank: rank,
            suit: suit,
            flipped: true
        }
    }

    pub fn flip(&mut self) {
        self.flipped = !self.flipped;
    }
}