use std::{collections::HashMap, path::Path};

use crate::util::get_lines;

fn sep(line: &str) -> (String, Vec<i64>) {
    let words: Vec<&str> = line.split_whitespace().collect();

    let springs = words[0].to_string();
    let conds: Vec<i64> = words[1]
        .split(',')
        .map(|c| c.parse::<i64>().unwrap())
        .collect();

    (springs, conds)
}

fn counts(s: &str, conds: &[i64], cache: &mut HashMap<(String, Vec<i64>), i64>) -> i64 {
    if s.len() == 0 {
        match conds.is_empty() {
            true => return 1,
            false => return 0,
        }
    }

    if conds.is_empty() {
        match s.contains("#") {
            true => return 0,
            false => return 1,
        }
    }

    let key = (s.to_string(), conds.to_vec());

    if let Some(v) = cache.get(&key) {
        return *v;
    }

    let mut count = 0;
    let f = s.chars().nth(0).unwrap();

    match f {
        '.' | '?' => count += counts(&s[1..], conds, cache),
        _ => (),
    }

    match f {
        '#' | '?' => {
            let n = conds[0] as usize;
            if n <= s.len()
                && !&s[..n].contains('.')
                && (n == s.len() || s.chars().nth(n).unwrap() != '#')
            {
                if n + 1 > s.len() {
                    count += counts("", &conds[1..], cache);
                } else {
                    count += counts(&s[n + 1..], &conds[1..], cache)
                }
            }
        }
        _ => (),
    }

    cache.insert(key, count);
    count
}

fn p2(lines: Vec<String>) {
    let mut sum = 0;
    let rep_count = 5;

    for line in &lines {
        let (springs, mut conditions) = sep(line);
        let mut springs2: String = "".to_string();

        for i in 0..rep_count {
            springs2.push_str(&springs.clone());

            if i < rep_count - 1 {
                springs2.push('?');
            }
        }

        conditions = conditions.repeat(5);

        let mut cache: HashMap<(String, Vec<i64>), i64> = HashMap::new();
        sum += counts(&springs2, conditions.as_slice(), &mut cache);
    }

    println!("Part 2: {}", sum);
}

fn p1(lines: Vec<String>) {
    let mut sum = 0;

    for line in &lines {
        let (springs, conditions) = sep(line);
        let mut cache: HashMap<(String, Vec<i64>), i64> = HashMap::new();
        sum += counts(&springs, conditions.as_slice(), &mut cache);
    }

    println!("Part 1: {}", sum);
}

pub fn d12() {
    let input_file = Path::new("src/d12/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
