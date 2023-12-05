use std::{
    cmp::min,
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
    path::Path,
    thread::{self, ScopedJoinHandle},
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

fn get_seeds2(s: &str) -> Vec<i64> {
    let ranges: Vec<(i64, i64)> = get_seeds(s).chunks_exact(2).map(|c| (c[0], c[1])).collect();
    let mut v: Vec<i64> = vec![];

    for r in ranges {
        let mut a: Vec<i64> = (r.0..r.0 + r.1).collect();
        v.append(&mut a);
    }

    v
}

fn p2(lines: Vec<String>) {
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

    let ranges: Vec<(i64, i64)> = get_seeds(&lines[0])
        .chunks_exact(2)
        .map(|c| (c[0], c[1]))
        .collect();

    let maps2 = &maps;
    let mut mins: Vec<i64> = vec![];

    // NOTE: Efficient solution is updating the seed intervals, but dont have the time
    thread::scope(|s| {
        let mut ts: Vec<ScopedJoinHandle<'_, i64>> = vec![];
        for r in ranges {
            ts.push(s.spawn(move || {
                let mut min_seed = i64::MAX;
                for seed in r.0..r.0 + r.1 {
                    let mut val = seed;

                    for m in MAP_NAMES {
                        for (k, v) in &maps2[m] {
                            if val >= k.0 && val <= k.1 {
                                val = v.0 + (val - k.0);
                                break;
                            }
                        }
                    }
                    min_seed = min(min_seed, val);
                }
                min_seed
            }));
        }

        for t in ts {
            mins.push(t.join().unwrap());
        }
    });

    println!("Part 2: {}", mins.iter().min().unwrap());
}

fn p1(lines: Vec<String>) {
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

    let seeds = get_seeds(&lines[0]);
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
    p2(get_lines(input_file));
}
