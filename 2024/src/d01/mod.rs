use std::collections::HashMap;

use crate::util::get_lines;

fn p01(lines: Vec<String>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<i32>().unwrap());
        right.push(r.parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let sum = left
        .iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r));

    println!("{}", sum);
}

fn p02(lines: Vec<String>) {
    let mut map = HashMap::new();

    let mut left = vec![];

    for line in lines {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<i32>().unwrap());

        let num = r.parse::<i32>().unwrap();
        *map.entry(num).or_insert(0) += 1;
    }

    let sum = left
        .iter()
        .fold(0, |acc, x| acc + (x * map.get(x).cloned().unwrap_or(0)));

    println!("{}", sum);
}

pub fn d01() {
    p01(get_lines("src/d01/in.txt"));
    p02(get_lines("src/d01/in.txt"));
}
