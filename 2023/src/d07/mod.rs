use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
    path::Path,
};

use crate::util::get_lines;

// NOTE: For part 1, J should be ordered between Q and T
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CamelCard {
    J,
    Num(u8),
    T,
    Q,
    K,
    A,
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

    fn get_counts(&self) -> HashMap<&CamelCard, u8> {
        let mut counts: HashMap<&CamelCard, u8> = HashMap::new();

        for card in &self.cards {
            if counts.contains_key(card) {
                *counts.get_mut(card).unwrap() += 1;
            } else {
                counts.insert(card, 1);
            }
        }

        counts
    }

    fn hand_type(&self) -> HandType {
        let counts = self.get_counts();

        match counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let some_val = counts.values().next();
                match some_val.unwrap() {
                    1 | 4 => HandType::FourOfAKind,
                    _ => HandType::FullHouse,
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
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Counts should not have this length"),
        }
    }

    fn hand_type2(&self) -> HandType {
        let counts = self.get_counts();
        let nj = *counts.get(&CamelCard::J).unwrap_or(&(0 as u8));

        if nj == 0 {
            return self.hand_type();
        }

        let mut m: u8 = 0;

        for (k, v) in &counts {
            match k {
                CamelCard::J => continue,
                _ => m = std::cmp::max(m, *v),
            }
        }

        match (m, nj) {
            (5, 0) | (0, 5) | (4, 1) | (3, 2) | (2, 3) | (1, 4) => HandType::FiveOfAKind,
            (3, 1) | (2, 2) | (1, 3) => HandType::FourOfAKind,
            (2, 1) | (1, 2) => match counts.len() {
                3 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            (1, 1) => HandType::OnePair,
            _ => panic!("Unhandled case {}-{}", m, nj),
        }
    }
}

fn make_hands(lines: &Vec<String>) -> Vec<(Hand, i32)> {
    let mut hands: Vec<(Hand, i32)> = vec![];

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        hands.push((Hand::new(words[0]), words[1].parse::<i32>().unwrap()));
    }

    hands
}

fn sort_hands(hands: &mut Vec<(Hand, i32)>) -> &Vec<(Hand, i32)> {
    hands.sort_by(|a, b| {
        let ord = a.0.hand_type().cmp(&b.0.hand_type());
        match ord {
            Ordering::Equal => return a.0.cards.cmp(&b.0.cards),
            _ => return ord,
        }
    });

    hands
}

fn sort_hands2(hands: &mut Vec<(Hand, i32)>) -> &Vec<(Hand, i32)> {
    hands.sort_by(|a, b| {
        let ord = a.0.hand_type2().cmp(&b.0.hand_type2());
        match ord {
            Ordering::Equal => return a.0.cards.cmp(&b.0.cards),
            _ => return ord,
        }
    });

    hands
}

fn p2(lines: Vec<String>) {
    let mut hands = make_hands(&lines);
    let sorted_hands = sort_hands2(&mut hands);

    let mut sum = 0;
    for (i, hand) in sorted_hands.iter().enumerate() {
        sum += (i + 1) * hand.1 as usize;
    }

    println!("Part 2: {}", sum);
}

fn p1(lines: Vec<String>) {
    let mut hands = make_hands(&lines);
    let sorted_hands = sort_hands(&mut hands);

    let mut sum = 0;
    for (i, hand) in sorted_hands.iter().enumerate() {
        sum += (i + 1) * hand.1 as usize;
    }

    println!("Part 1: {}", sum);
}

pub fn d07() {
    let input_file = Path::new("src/d07/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
