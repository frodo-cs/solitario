use std::collections::HashMap;
use crate::table::Table;
use crate::card::Card;

pub fn tableau_check(table: &Table, card: &Card) -> Vec<(&'static str, usize, usize)> {
    let mut possibilities: Vec<(&'static str, usize, usize)> = vec![]; // (where, origin, destination)
   
    for i in 0..7 {
        match table.tableau[i].last() {
            Some(c) => {
                if color_check(card, c) && rank_check(card, c) {
                    possibilities.push(("column", 0, i))
                }
            }
            None => {
                if open_check(card, "K") {
                    possibilities.push(("column", 0, i))
                }
            }
        }
    }

    possibilities
}

pub fn foundation_check(table: &Table, card: &Card) -> Vec<(&'static str, usize, usize)> {
    let mut possibilities: Vec<(&'static str, usize, usize)> = vec![]; // (where, origin, destination)

    for i in 0..4 {
        match table.foundation[i].cards.last() {
            Some(c) => {
                if suit_check(card, c) && rank_check(c, card) {
                    possibilities.push(("foundation", 0, i));
                }
            }
            None => {
                if open_check(card, "A") {
                    possibilities.push(("foundation", 0, i))
                }
            }
        }
    }

    possibilities
}

fn color_check(origin: &Card, destination: &Card) -> bool {
    origin.color != destination.color
}

fn rank_check(origin: &Card, destination: &Card) -> bool {
    let r = Rank::default();
    r.rank[origin.rank] < r.rank[destination.rank]
}

fn open_check(origin: &Card, value: &'static str) -> bool {
    origin.rank == value
}

fn suit_check(origin: &Card, destination: &Card) -> bool {
    origin.suit == destination.suit
}

pub struct Rank {
    rank: HashMap<&'static str, usize>,
}

impl Default for Rank {
    fn default () -> Rank {
        let rank = ["A", "2", "3","4","5","6","7","8","9","Z","J","Q","K"];
        let mut r = Rank{ rank: HashMap::new() };

        for i in 0..13 {
            r.rank.insert(rank[i], i);
        }

        r
    }
}

