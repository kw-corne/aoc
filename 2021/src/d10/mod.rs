use std::{collections::HashMap, path::Path};

use crate::util::get_lines;

fn p2(lines: Vec<String>) {
    let points = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut scores: Vec<i64> = vec![];

    for line in &lines {
        let mut stack: Vec<char> = vec![];
        let mut is_valid = true;

        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '<' => stack.push('>'),
                ')' | '}' | ']' | '>' => {
                    if stack.pop().unwrap() != c {
                        is_valid = false;
                        break;
                    }
                }
                _ => panic!("Unhandled char: {}", c),
            }
        }

        if is_valid {
            let mut score = 0;
            while let Some(c) = stack.pop() {
                score *= 5;
                score += points.get(&c).unwrap();
            }
            scores.push(score);
        }
    }

    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}

fn p1(lines: Vec<String>) {
    let points = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut sum = 0;

    for line in &lines {
        let mut stack: Vec<char> = vec![];

        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '<' => stack.push('>'),
                ')' | '}' | ']' | '>' => {
                    if stack.pop().unwrap() != c {
                        sum += points.get(&c).unwrap();
                    }
                }
                _ => panic!("Unhandled char: {}", c),
            }
        }
    }

    println!("{}", sum);
}

pub fn d10() {
    let input_file = Path::new("src/d10/in.txt");

    // p1(get_lines(input_file));
    p2(get_lines(input_file));
}
