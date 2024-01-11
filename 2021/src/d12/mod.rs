use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::util::get_lines;

fn get_caves(lines: &Vec<String>) -> HashMap<String, Vec<&str>> {
    let mut caves: HashMap<String, Vec<&str>> = HashMap::new();

    for line in lines {
        let (l, r) = line.split_once('-').unwrap();
        caves.entry(l.to_string()).or_default().push(&r);
        caves.entry(r.to_string()).or_default().push(&l);
    }

    caves
}

fn num_paths2<'a>(
    cave: &'a str,
    caves: &HashMap<String, Vec<&'a str>>,
    mut visited: HashSet<&'a str>,
    lc_twice: bool,
) -> i32 {
    if cave == "end" {
        return 1;
    }

    let mut n_paths = 0;

    for next_cave in caves.get(cave).unwrap() {
        if *next_cave == "start" || (lc_twice && visited.contains(next_cave)) {
            continue;
        }

        if cave.chars().all(|c| c.is_lowercase()) {
            visited.insert(cave);
        }

        if visited.contains(next_cave) {
            n_paths += num_paths2(next_cave, caves, visited.clone(), true);
        } else {
            n_paths += num_paths2(next_cave, caves, visited.clone(), lc_twice);
        }
    }

    n_paths
}

fn num_paths1<'a>(
    cave: &'a str,
    caves: &HashMap<String, Vec<&'a str>>,
    mut visited: HashSet<&'a str>,
) -> i32 {
    if cave == "end" {
        return 1;
    }

    let mut n_paths = 0;

    for next_cave in caves.get(cave).unwrap() {
        if visited.contains(*next_cave) {
            continue;
        }

        if cave.chars().all(|c| c.is_lowercase()) {
            visited.insert(cave);
        }

        n_paths += num_paths1(next_cave, caves, visited.clone());
    }

    n_paths
}

fn p2(lines: Vec<String>) {
    let caves = get_caves(&lines);
    println!("{}", num_paths2("start", &caves, HashSet::new(), false));
}

fn p1(lines: Vec<String>) {
    let caves = get_caves(&lines);
    println!("{}", num_paths1("start", &caves, HashSet::new()));
}

pub fn d12() {
    let input_file = Path::new("src/d12/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
