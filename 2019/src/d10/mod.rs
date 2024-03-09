use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

const ASTEROID_CHAR: char = '#';
type AsteroidMap = HashSet<Point>;

struct Point {
    x: usize,
    y: usize,
}

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let mut asteroid_map: AsteroidMap = HashSet::new();

    for i in 0..lines.len() {
        for (j, ch) in lines[i].chars().enumerate() {

        }
    }
}

pub fn d10() {
    let input_file = Path::new("src/d10/dbg.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
