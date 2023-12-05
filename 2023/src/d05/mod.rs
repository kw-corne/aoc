use std::{
    cmp::min,
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
    path::Path,
};

use crate::util::get_lines;

const MAP_NAMES: [&str; 7] = ["s2s", "s2f", "f2w", "w2l", "l2t", "t2h", "h2l"];

fn line_instr(s: &str) -> (i64, i64, i64) {
    let v: Vec<i64> = s
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    if let [a, b, c] = v.as_slice() {
        return (*a, *b, *c);
    } else {
        panic!();
    }
}

fn get_maps() -> HashMap<String, HashMap<(i64, i64), (i64, i64)>> {
    let mut m: HashMap<String, HashMap<(i64, i64), (i64, i64)>> = HashMap::new();

    for s in MAP_NAMES {
        m.insert(s.to_string(), HashMap::new());
    }

    m
}

fn get_seeds(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let seeds = get_seeds(&lines[0]);
    let mut maps = get_maps();
    let mut i = 3;
    let mut map_i = 0;

    while i < lines.len() {
        let map_name = MAP_NAMES[map_i];
        let m = maps.get_mut(map_name).unwrap();

        while lines[i] != "" {
            let (k, v, len) = line_instr(&lines[i]);

            let range_left = (v, v + (len - 1));
            let range_right = (k, k + (len - 1));

            m.insert(range_left, range_right);

            i += 1;
            if i >= lines.len() {
                break;
            }
        }

        i += 2;
        map_i += 1;
    }

    let mut min_seed = i64::MAX;

    for seed in seeds {
        let mut val = seed;

        for m in MAP_NAMES {
            for (k, v) in &maps[m] {
                if val >= k.0 && val <= k.1 {
                    val = v.0 + (val - k.0);
                    break;
                }
            }
        }

        min_seed = min(min_seed, val);
    }

    println!("Part 1: {}", min_seed);
}

pub fn d05() {
    let input_file = Path::new("src/d05/in.txt");

    p1(get_lines(input_file));
    // p2(read_lines(input_file).expect("Couldnt read lines of file"));
}
