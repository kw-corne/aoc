use std::{collections::HashSet, io, path::Path};

use crate::util::{read_lines, InputLines};

fn p2(lines: InputLines) {
    let mut lines2: Vec<Result<String, io::Error>> = lines.collect();
    let mut copies = vec![1; lines2.len()];

    for (j, line) in lines2.iter_mut().enumerate() {
        let rline = line.as_ref().expect("Expected valid line");
        let words: Vec<&str> = rline.split_whitespace().skip(2).collect();
        let mut win_nums: HashSet<i32> = HashSet::new();
        let mut n_wins = 0;
        let mut i = 0;

        while words[i] != "|" {
            win_nums.insert(words[i].parse::<i32>().unwrap());
            i += 1;
        }

        i += 1;

        while i < words.len() {
            let n = words[i].parse::<i32>().unwrap();

            if win_nums.contains(&n) {
                n_wins += 1;
            }

            i += 1;
        }

        for k in j + 1..=j + n_wins {
            copies[k] += copies[j];
        }
    }

    println!("Part 2: {}", copies.iter().sum::<i32>());
}

fn p1(lines: InputLines) {
    let mut sum = 0;

    for line in lines {
        let rline = line.expect("Expected valid line");
        let words: Vec<&str> = rline.split_whitespace().skip(2).collect();
        let mut win_nums: HashSet<i32> = HashSet::new();
        let mut row_sum = 0;
        let mut i = 0;

        while words[i] != "|" {
            win_nums.insert(words[i].parse::<i32>().unwrap());
            i += 1;
        }

        i += 1;

        while i < words.len() {
            let n = words[i].parse::<i32>().unwrap();

            if win_nums.contains(&n) {
                match row_sum {
                    0 => row_sum = 1,
                    _ => row_sum *= 2,
                }
            }

            i += 1;
        }

        sum += row_sum;
    }

    println!("Part 1: {sum}");
}

pub fn d04() {
    let input_file = Path::new("src/d04/in.txt");

    p1(read_lines(input_file).expect("Couldnt read lines of file"));
    p2(read_lines(input_file).expect("Couldnt read lines of file"));
}
