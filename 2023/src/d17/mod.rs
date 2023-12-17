use std::path::Path;

use crate::util::{get_lines, lines_to_2d_chars};

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    hl: i32,
    pos: (i32, i32),
    dir: (i32, i32),
    n: i32, // times moved in this dir
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.hl.cmp(&self.hl)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn in_grid(pos: (i32, i32), bounds: (i32, i32)) -> bool {
    (pos.0 >= 0 && pos.0 < bounds.0) && (pos.1 >= 0 && pos.1 < bounds.1)
}

fn p2(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut cache: HashSet<State> = HashSet::new();
    let bounds = (grid.len() as i32, grid[0].len() as i32);
    let end_pos = (bounds.0 - 1, bounds.1 - 1);

    heap.push(State {
        hl: 0,
        pos: (0, 0),
        dir: (0, 0),
        n: 0,
    });

    while let Some(State { hl, pos, dir, n }) = heap.pop() {
        if pos == end_pos {
            println!("Reached end: {} {:?} {:?} {}", hl, pos, dir, n);
            break;
        }

        if cache.contains(&State { hl: 0, pos, dir, n }) {
            continue;
        }

        cache.insert(State { hl: 0, pos, dir, n });

        if n < 10 && dir != (0, 0) {
            let np = (pos.0 + dir.0, pos.1 + dir.1);
            if in_grid(np, bounds) {
                let next_hl = grid[np.0 as usize][np.1 as usize].to_digit(10).unwrap() as i32;
                heap.push(State {
                    hl: hl + next_hl,
                    pos: np,
                    dir,
                    n: n + 1,
                })
            }
        }

        if n >= 4 || dir == (0, 0) {
            for ndir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if ndir != dir && ndir != (-dir.0, -dir.1) {
                    let np = (pos.0 + ndir.0, pos.1 + ndir.1);
                    if in_grid(np, bounds) {
                        let next_hl =
                            grid[np.0 as usize][np.1 as usize].to_digit(10).unwrap() as i32;
                        heap.push(State {
                            hl: hl + next_hl,
                            pos: np,
                            dir: ndir,
                            n: 1,
                        })
                    }
                }
            }
        }
    }
}

fn p1(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut cache: HashSet<State> = HashSet::new();
    let bounds = (grid.len() as i32, grid[0].len() as i32);
    let end_pos = (bounds.0 - 1, bounds.1 - 1);

    heap.push(State {
        hl: 0,
        pos: (0, 0),
        dir: (0, 0),
        n: 0,
    });

    while let Some(State { hl, pos, dir, n }) = heap.pop() {
        if pos == end_pos {
            println!("Reached end: {} {:?} {:?} {}", hl, pos, dir, n);
            break;
        }

        if cache.contains(&State { hl: 0, pos, dir, n }) {
            continue;
        }

        cache.insert(State { hl: 0, pos, dir, n });

        if n < 3 && dir != (0, 0) {
            let np = (pos.0 + dir.0, pos.1 + dir.1);
            if in_grid(np, bounds) {
                let next_hl = grid[np.0 as usize][np.1 as usize].to_digit(10).unwrap() as i32;
                heap.push(State {
                    hl: hl + next_hl,
                    pos: np,
                    dir,
                    n: n + 1,
                })
            }
        }

        for ndir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if ndir != dir && ndir != (-dir.0, -dir.1) {
                let np = (pos.0 + ndir.0, pos.1 + ndir.1);
                if in_grid(np, bounds) {
                    let next_hl = grid[np.0 as usize][np.1 as usize].to_digit(10).unwrap() as i32;
                    heap.push(State {
                        hl: hl + next_hl,
                        pos: np,
                        dir: ndir,
                        n: 1,
                    })
                }
            }
        }
    }
}

pub fn d17() {
    let input_file = Path::new("src/d17/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
