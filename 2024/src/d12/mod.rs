use std::collections::{HashMap, HashSet};

use crate::util::{get_lines, in_bounds, lines_to_2d_chars};

#[derive(Debug, PartialEq, Eq)]
struct Region {
    c: char,
    points: HashSet<(i32, i32)>,
}

impl Region {
    fn area(&self) -> i32 {
        self.points.len() as i32
    }

    fn perimeter(&self) -> i32 {
        let mut perim = 0;
        let mut counted = HashSet::new();

        for p in &self.points {
            if counted.contains(&p) {
                continue;
            }
            counted.insert(p);

            perim += 4;

            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let adj_pos = (p.0 + dir.0, p.1 + dir.1);

                if self.points.contains(&(adj_pos)) {
                    perim -= 1;
                }
            }
        }

        perim
    }

    fn sides(&self) -> i32 {
        let mut corners = 0;

        for p in &self.points {
            let above = (p.0 - 1, p.1);
            let below = (p.0 + 1, p.1);
            let right = (p.0, p.1 + 1);
            let left = (p.0, p.1 - 1);

            let has_above = self.points.contains(&above);
            let has_below = self.points.contains(&below);
            let has_right = self.points.contains(&right);
            let has_left = self.points.contains(&left);

            // outer top right corner
            if !has_above && !has_right {
                corners += 1;
            }

            // outer bot right right
            if !has_below && !has_right {
                corners += 1;
            }

            // outer bot left corner
            if !has_below && !has_left {
                corners += 1;
            }

            // outer top left corner
            if !has_above && !has_left {
                corners += 1;
            }

            // inner bot left corner
            if has_left && has_below {
                let diag_left_below = (p.0 + 1, p.1 - 1);
                if !self.points.contains(&diag_left_below) {
                    corners += 1;
                }
            }

            // inner bot right corner
            if has_right && has_below {
                let diag_right_below = (p.0 + 1, p.1 + 1);
                if !self.points.contains(&diag_right_below) {
                    corners += 1;
                }
            }

            // inner top right corner
            if has_right && has_above {
                let diag_right_above = (p.0 - 1, p.1 + 1);
                if !self.points.contains(&diag_right_above) {
                    corners += 1;
                }
            }

            // inner top left corner
            if has_left && has_above {
                let diag_left_above = (p.0 - 1, p.1 - 1);
                if !self.points.contains(&diag_left_above) {
                    corners += 1;
                }
            }
        }

        corners
    }

    pub fn has_point(&self, p: (i32, i32)) -> bool {
        self.points.contains(&p)
    }
}

fn make_region(grid: &[Vec<char>], pos: (usize, usize)) -> Region {
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    let char = grid[pos.0][pos.1];

    let width = grid[0].len();
    let height = grid.len();

    let mut stack = vec![(pos.0 as i32, pos.1 as i32)];

    while let Some(p) = stack.pop() {
        points.insert(p);

        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next_pos = (p.0 + dir.0, p.1 + dir.1);

            if in_bounds(next_pos, width as i32, height as i32) && !points.contains(&next_pos) {
                let next_char = grid[next_pos.0 as usize][next_pos.1 as usize];
                if next_char == char {
                    stack.push(next_pos);
                }
            }
        }
    }

    Region { c: char, points }
}

fn p02(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);
    let mut regions: Vec<Region> = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if regions.iter().any(|r| r.has_point((i as i32, j as i32))) {
                continue;
            }

            regions.push(make_region(&grid, (i, j)));
        }
    }

    let mut sum = 0;
    for r in regions {
        sum += r.area() * r.sides();
    }
    println!("{}", sum);
}

fn p01(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);
    let mut regions: Vec<Region> = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if regions.iter().any(|r| r.has_point((i as i32, j as i32))) {
                continue;
            }

            regions.push(make_region(&grid, (i, j)));
        }
    }

    let mut sum = 0;
    for r in regions {
        sum += r.area() * r.perimeter();
    }
    println!("{}", sum);
}

pub fn d12() {
    p01(get_lines("src/d12/in.txt"));
    p02(get_lines("src/d12/in.txt"));
}
