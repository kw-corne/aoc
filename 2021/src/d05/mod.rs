use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::util::get_lines;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(s: &str) -> Self {
        let split: Vec<i32> = s.split(',').map(|n| n.parse::<i32>().unwrap()).collect();

        Self {
            x: split[0],
            y: split[1],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Line(Point, Point);

impl Line {
    fn new(line: &str) -> Self {
        let split: Vec<&str> = line.split_whitespace().collect();

        Self {
            0: Point::new(split[0]),
            1: Point::new(split[2]),
        }
    }

    fn is_diagonal(&self) -> bool {
        !(self.0.x == self.1.x || self.0.y == self.1.y)
    }

    fn points(&self) -> HashSet<Point> {
        let dx = match self.0.x.cmp(&self.1.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let dy = match self.0.y.cmp(&self.1.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let step = (dx, dy);
        let mut pts: HashSet<Point> = HashSet::new();
        let mut curr = self.0.clone();
        pts.insert(curr.clone());

        while curr != self.1 {
            curr.x += step.0;
            curr.y += step.1;
            pts.insert(curr.clone());
        }

        pts
    }
}

fn p2(lines: Vec<String>) {
    let mut point_count: HashMap<Point, i32> = HashMap::new();

    for input_line in &lines {
        let line = Line::new(input_line);

        for pt in line.points() {
            *point_count.entry(pt).or_insert(0) += 1;
        }
    }

    println!("{}", point_count.values().filter(|v| **v >= 2).count());
}

fn p1(lines: Vec<String>) {
    let mut point_count: HashMap<Point, i32> = HashMap::new();

    for input_line in &lines {
        let line = Line::new(input_line);
        if line.is_diagonal() {
            continue;
        }

        for pt in line.points() {
            *point_count.entry(pt).or_insert(0) += 1;
        }
    }

    println!("{}", point_count.values().filter(|v| **v >= 2).count());
}

pub fn d05() {
    let input_file = Path::new("src/d05/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
