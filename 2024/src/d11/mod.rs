use std::collections::HashMap;

use crate::util::get_lines;

fn get_stones(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn stone_count(stone: i64, iters: i32, cache: &mut HashMap<(i64, i32), i64>) -> i64 {
    if cache.contains_key(&(stone, iters)) {
        return cache[&(stone, iters)];
    }

    if iters == 0 {
        return 1;
    }

    if stone == 0 {
        let count = stone_count(1, iters - 1, cache);
        cache.insert((1, iters - 1), count);
        return count;
    } else {
        let digits = stone.to_string();
        let len = digits.len();

        if len % 2 == 0 {
            let left = digits[..len / 2].parse::<i64>().unwrap();
            let right = digits[len / 2..].parse::<i64>().unwrap();
            let count_l = stone_count(left, iters - 1, cache);
            let count_r = stone_count(right, iters - 1, cache);
            cache.insert((left, iters - 1), count_l);
            cache.insert((right, iters - 1), count_r);
            return count_l + count_r;
        } else {
            let count = stone_count(stone * 2024, iters - 1, cache);
            cache.insert((stone * 2024, iters - 1), count);
            return count;
        }
    }
}

fn p02(lines: Vec<String>) {
    let stones = get_stones(&lines[0]);
    let iters = 75;
    let mut cache = HashMap::new();

    let total: i64 = stones
        .iter()
        .map(|&s| stone_count(s, iters, &mut cache))
        .sum();

    println!("{}", total);
}

fn p01(lines: Vec<String>) {
    let stones = get_stones(&lines[0]);
    let iters = 25;
    let mut cache = HashMap::new();

    let total: i64 = stones
        .iter()
        .map(|&s| stone_count(s, iters, &mut cache))
        .sum();

    println!("{}", total);
}

pub fn d11() {
    p01(get_lines("src/d11/in.txt"));
    p02(get_lines("src/d11/in.txt"));
}
