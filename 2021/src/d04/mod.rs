use std::path::Path;

use crate::util::{get_lines, slice_col};

const CARD_SHAPE: usize = 5;

#[derive(Debug)]
struct BingoCard {
    card: Vec<(i32, bool)>,
    has_won: bool,
}

impl BingoCard {
    fn new(lines: &[String]) -> Self {
        let card: Vec<(i32, bool)> = lines
            .iter()
            .flat_map(|row| {
                row.split_whitespace()
                    .map(|n| (n.parse::<i32>().unwrap(), false))
            })
            .collect();

        Self {
            card,
            has_won: false,
        }
    }

    fn unmarked_sum(&self) -> i32 {
        self.card
            .iter()
            .filter(|e| !e.1)
            .fold(0, |acc, e| acc + e.0)
    }

    fn mark(&mut self, num: i32) -> bool {
        for row in self.card.chunks_mut(CARD_SHAPE) {
            let mut won = true;
            for entry in row {
                if entry.0 == num {
                    entry.1 = true;
                }
                if entry.1 == false {
                    won = false;
                }
            }

            if won {
                return true;
            }
        }

        for col in 0..CARD_SHAPE {
            let mut won = true;
            for entry in self.card.iter_mut().skip(col).step_by(CARD_SHAPE) {
                if entry.0 == num {
                    entry.1 = true;
                }
                if entry.1 == false {
                    won = false;
                }
            }
            if won {
                return true;
            }
        }

        false
    }
}

fn p2(lines: Vec<String>) {
    let mut cards: Vec<BingoCard> = vec![];
    let mut j = 2;
    for i in 2..lines.len() {
        if i == lines.len() - 1 || lines[i + 1].is_empty() {
            cards.push(BingoCard::new(&lines[j..=i]));
            j = i + 1;
        }
    }

    let nums: Vec<i32> = lines[0]
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    for num in nums {
        for card in cards.iter_mut() {
            if card.has_won {
                continue;
            }
            let won = card.mark(num);
            if won {
                // last number printed is the answer
                println!("{}", card.unmarked_sum() * num);
                card.has_won = true;
            }
        }
    }
}

fn p1(lines: Vec<String>) {
    let mut cards: Vec<BingoCard> = vec![];
    let mut j = 2;
    for i in 2..lines.len() {
        if i == lines.len() - 1 || lines[i + 1].is_empty() {
            cards.push(BingoCard::new(&lines[j..=i]));
            j = i + 1;
        }
    }

    let nums: Vec<i32> = lines[0]
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    'outer: for num in nums {
        for card in cards.iter_mut() {
            let won = card.mark(num);
            if won {
                println!("{}", card.unmarked_sum() * num);
                break 'outer;
            }
        }
    }
}

pub fn d04() {
    let input_file = Path::new("src/d04/in.txt");

    // p1(get_lines(input_file));
    p2(get_lines(input_file));
}
