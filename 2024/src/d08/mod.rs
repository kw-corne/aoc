use std::collections::{HashMap, HashSet};

use crate::util::{dump_grid, get_lines, lines_to_2d_chars};

fn in_bounds(p: (i32, i32), width: i32, height: i32) -> bool {
    p.0 >= 0 && p.0 < height && p.1 >= 0 && p.1 < width
}

fn p2(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);

    let mut antennas = HashMap::new();
    let mut antis = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let ch = grid[i][j];
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_insert(vec![])
                    .push((i as i32, j as i32));
            }
        }
    }

    let height = grid.len();
    let width = grid[0].len();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            antis.insert(positions[i]);

            let pi = positions[i];
            for j in i + 1..positions.len() {
                let pj = positions[j];

                let dxdy1 = (pi.0 - pj.0, pi.1 - pj.1);
                let mut anti1 = (pi.0 + dxdy1.0, pi.1 + dxdy1.1);

                while in_bounds(anti1, width as i32, height as i32) {
                    antis.insert(anti1);
                    anti1 = (anti1.0 + dxdy1.0, anti1.1 + dxdy1.1);
                }

                let dxdy2 = (pj.0 - pi.0, pj.1 - pi.1);
                let mut anti2 = (pj.0 + dxdy2.0, pj.1 + dxdy2.1);

                while in_bounds(anti2, width as i32, height as i32) {
                    antis.insert(anti2);
                    anti2 = (anti2.0 + dxdy2.0, anti2.1 + dxdy2.1);
                }
            }
        }
    }

    println!("{}", antis.len());
}

fn p1(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);

    let mut antennas = HashMap::new();
    let mut antis = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let ch = grid[i][j];
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_insert(vec![])
                    .push((i as i32, j as i32));
            }
        }
    }

    let height = grid.len();
    let width = grid[0].len();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            let pi = positions[i];
            for j in i + 1..positions.len() {
                let pj = positions[j];

                let dxdy1 = (pi.0 - pj.0, pi.1 - pj.1);
                let anti1 = (pi.0 + dxdy1.0, pi.1 + dxdy1.1);

                if in_bounds(anti1, width as i32, height as i32) {
                    antis.insert(anti1);
                }

                let dxdy2 = (pj.0 - pi.0, pj.1 - pi.1);
                let anti2 = (pj.0 + dxdy2.0, pj.1 + dxdy2.1);

                if in_bounds(anti2, width as i32, height as i32) {
                    antis.insert(anti2);
                }
            }
        }
    }

    println!("{}", antis.len());
}

pub fn d08() {
    p1(get_lines("src/d08/in.txt"));
    p2(get_lines("src/d08/in.txt"));
}
