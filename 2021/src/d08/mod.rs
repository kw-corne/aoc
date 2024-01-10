use std::{collections::HashSet, path::Path};

use crate::util::get_lines;

fn overlap1(word: &str, chars1: &HashSet<char>) -> i32 {
    chars1.iter().filter(|&c| word.contains(*c)).count() as i32
}

fn overlap4(word: &str, chars4: &HashSet<char>) -> i32 {
    chars4.iter().filter(|&c| word.contains(*c)).count() as i32
}

fn p2(lines: Vec<String>) {
    let mut sum = 0;

    for line in &lines {
        let (input, output) = line.split_once('|').unwrap();

        let mut chars1: HashSet<char> = HashSet::new();
        let mut chars4: HashSet<char> = HashSet::new();

        for word in input.split_whitespace() {
            match word.len() {
                2 => chars1.extend(word.chars()),
                4 => chars4.extend(word.chars()),
                _ => (),
            }
        }

        sum += output.split_whitespace().fold(0, |acc, word| {
            acc * 10
                + match (word.len(), overlap1(word, &chars1), overlap4(word, &chars4)) {
                    (2, _, _) => 1,
                    (3, _, _) => 7,
                    (4, _, _) => 4,
                    (7, _, _) => 8,
                    (5, _, 2) => 2,
                    (5, 1, 3) => 5,
                    (5, 2, 3) => 3,
                    (6, _, 4) => 9,
                    (6, 1, 3) => 6,
                    (6, 2, 3) => 0,
                    _ => panic!(),
                }
        });
    }

    println!("{}", sum);
}

fn p1(lines: Vec<String>) {
    let mut total = 0;
    for line in &lines {
        let output = line.split_once('|').unwrap().1;

        for word in output.split_whitespace() {
            match word.len() {
                2 | 3 | 4 | 7 => total += 1,
                _ => (),
            }
        }
    }

    println!("{}", total);
}

pub fn d08() {
    let input_file = Path::new("src/d08/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
