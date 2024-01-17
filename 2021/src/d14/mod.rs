use std::{cell::RefCell, collections::HashMap, path::Path};

use crate::util::get_lines;

type Rules = HashMap<[char; 2], char>;

fn nchar(s: &str, i: usize) -> char {
    s.chars().nth(i).unwrap()
}

fn char_counts(s: &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();

    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    counts
}

fn char_counts_from_pair_counts(
    pair_counts: &HashMap<[char; 2], i64>,
    first_char: char,
    last_char: char,
) -> HashMap<char, i64> {
    let mut raw_counts = HashMap::new();

    for (pair, count) in pair_counts {
        *raw_counts.entry(pair[0]).or_insert(0) += count;
        *raw_counts.entry(pair[1]).or_insert(0) += count;
    }

    let mut real_counts = HashMap::new();
    for (ch, mut count) in raw_counts {
        if ch == first_char || ch == last_char {
            count += 1;
        }

        real_counts.insert(ch, count / 2);
    }

    real_counts
}

fn get_insertions(insertion_rules: &[String]) -> Rules {
    let mut rules: Rules = HashMap::new();

    for line in insertion_rules {
        let (l, r) = line.split_once(" -> ").unwrap();
        rules.insert([nchar(l, 0), nchar(l, 1)], nchar(r, 0));
    }

    rules
}

fn p2(lines: Vec<String>) {
    let ply = lines[0].clone();
    let insertions = get_insertions(&lines[2..]);
    let mut pair_counts: HashMap<[char; 2], i64> = HashMap::new();

    for pair in insertions.keys() {
        pair_counts.insert(pair.clone(), 0);
    }

    let mut i = 0;
    while i < ply.len() - 1 {
        let pair = [nchar(&ply, i), nchar(&ply, i + 1)];
        pair_counts.entry(pair).and_modify(|c| *c += 1);
        i += 1;
    }

    let first_char = nchar(&ply, 0);
    let last_char = nchar(&ply, ply.len() - 1);

    for _ in 0..40 {
        let mut inc_kv: Vec<([char; 2], i64)> = vec![];
        let mut dec_kv: Vec<([char; 2], i64)> = vec![];

        for (pair, count) in &pair_counts {
            if *count == 0 {
                continue;
            }

            let new_char = insertions.get(pair).unwrap();

            inc_kv.push(([pair[0], *new_char], *count));
            inc_kv.push(([*new_char, pair[1]], *count));
            dec_kv.push((*pair, *count));
        }

        for (pair, n) in inc_kv {
            pair_counts.entry(pair).and_modify(|c| *c += n);
        }
        for (pair, n) in dec_kv {
            pair_counts.entry(pair).and_modify(|c| *c -= n);
        }
    }

    let counts = char_counts_from_pair_counts(&pair_counts, first_char, last_char);
    println!(
        "{}",
        counts.values().max().unwrap() - counts.values().min().unwrap()
    );
}

fn p1(lines: Vec<String>) {
    let mut ply = lines[0].clone();
    let insertions = get_insertions(&lines[2..]);

    for _ in 0..10 {
        let mut to_insert: Vec<(usize, char)> = vec![];

        let mut i = 0;
        while i < ply.len() - 1 {
            let pair = [nchar(&ply, i), nchar(&ply, i + 1)];

            if let Some(insert_char) = insertions.get(&pair) {
                to_insert.push((i + 1 + to_insert.len(), *insert_char));
            }

            i += 1;
        }

        for (idx, ch) in to_insert {
            ply.insert(idx, ch);
        }
    }

    let counts = char_counts(&ply);
    println!(
        "{}",
        counts.values().max().unwrap() - counts.values().min().unwrap()
    );
}

pub fn d14() {
    let input_file = Path::new("src/d14/in.txt");

    // p1(get_lines(input_file));
    p2(get_lines(input_file));
}
