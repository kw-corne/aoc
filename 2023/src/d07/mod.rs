use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
    path::Path,
};

use crate::util::get_lines;

const HAND_SIZE: usize = 5;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CamelCard {
    Num(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl CamelCard {
    const fn variants() -> u8 {
        13
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<CamelCard>,
}

impl Hand {
    fn new(hand: &str) -> Self {
        let mut cards: Vec<CamelCard> = vec![];

        for c in hand.chars() {
            match c {
                'A' => cards.push(CamelCard::A),
                'K' => cards.push(CamelCard::K),
                'Q' => cards.push(CamelCard::Q),
                'J' => cards.push(CamelCard::J),
                'T' => cards.push(CamelCard::T),
                '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' => {
                    let n = c.to_digit(10).unwrap();
                    assert!(n <= 9 && n >= 2);

                    cards.push(CamelCard::Num(n as u8));
                }
                _ => panic!("Unknown hand char {c}"),
            }
        }

        Hand { cards }
    }

    fn hand_type(&self) -> HandType {
        let mut counts: HashMap<&CamelCard, u8> = HashMap::new();

        for card in &self.cards {
            if counts.contains_key(card) {
                *counts.get_mut(card).unwrap() += 1;
            } else {
                counts.insert(card, 1);
            }
        }

        match counts.len() {
            1 => return HandType::FiveOfAKind,
            2 => {
                let some_val = counts.values().next();
                match some_val.unwrap() {
                    1 | 4 => return HandType::FourOfAKind,
                    _ => return HandType::FullHouse,
                }
            }
            3 => {
                for v in counts.values() {
                    if *v == 3 {
                        return HandType::ThreeOfAKind;
                    }
                }
                return HandType::TwoPair;
            }
            4 => return HandType::OnePair,
            5 => return HandType::HighCard,
            _ => panic!("Counts should not have this length"),
        }
    }
}

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let mut hands: Vec<(Hand, i32)> = vec![];

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        hands.push((Hand::new(words[0]), words[1].parse::<i32>().unwrap()));
    }

    hands.sort_by(|a, b| {
        let ord = a.0.hand_type().cmp(&b.0.hand_type());
        match ord {
            Ordering::Equal => return a.0.cards.cmp(&b.0.cards),
            _ => return ord,
        }
    });

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i + 1) * hand.1 as usize;
    }

    println!("Part 1: {}", sum);
}

pub fn d07() {
    let input_file = Path::new("src/d07/in.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
