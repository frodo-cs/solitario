use std::fmt;

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

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.face_down {
            write!(f, "XXX")
        } else {
            write!(f, "{}{}", self.rank, self.suit)
        }
    }
}