use std::collections::{HashMap, HashSet};

use crate::util::get_lines;

fn all_solutions(s: &str, towels: &HashSet<&str>, cache: &mut HashMap<String, i64>) -> i64 {
    if s.is_empty() {
        return 1;
    }

    if let Some(v) = cache.get(s) {
        return *v;
    }

    let mut solutions = 0;

    for t in towels {
        if let Some(subs) = s.strip_prefix(t) {
            solutions += all_solutions(subs, towels, cache);
        }
    }

    cache.insert(s.into(), solutions);
    solutions
}

fn p02(lines: Vec<String>) {
    let towels = lines[0].split(", ").collect::<HashSet<_>>();
    let mut sum = 0;
    let mut cache = HashMap::new();

    for line in &lines[2..] {
        sum += all_solutions(line, &towels, &mut cache);
    }

    println!("{}", sum);
}

fn is_solution(s: &str, towels: &HashSet<&str>, cache: &mut HashMap<String, bool>) -> bool {
    if s.is_empty() {
        return true;
    }

    if let Some(v) = cache.get(s) {
        return *v;
    }

    for t in towels {
        if let Some(subs) = s.strip_prefix(t) {
            if is_solution(subs, towels, cache) {
                cache.insert(subs.to_string(), true);
                return true;
            }
        }
    }

    cache.insert(s.into(), false);
    false
}

fn p01(lines: Vec<String>) {
    let towels = lines[0].split(", ").collect::<HashSet<_>>();
    let mut count = 0;
    let mut cache = HashMap::new();

    for line in &lines[2..] {
        if is_solution(line, &towels, &mut cache) {
            count += 1;
        }
    }

    println!("{}", count);
}

pub fn d19() {
    p01(get_lines("src/d19/in.txt"));
    p02(get_lines("src/d19/in.txt"));
}
