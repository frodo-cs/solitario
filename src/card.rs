#[derive(Debug)]
pub struct Card {
    rank: String,
    suit: String,
    face_down: bool
}

impl Card {
    pub fn new(rank: String, suit: String) -> Card {
        Card {
            rank: rank,
            suit: suit,
            face_down: true
        }
    }

    pub fn flip(&mut self) {
        self.face_down = !self.face_down;
    }
}