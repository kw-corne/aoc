use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    process::exit,
};

use crate::util::{dump_grid, get_lines, in_bounds};

#[derive(Debug, Eq, Clone, Copy, PartialEq, Hash)]
struct State {
    pos: (isize, isize),
    steps: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.steps.cmp(&self.steps))
    }
}

fn make_grid(lines: &Vec<String>, grid_size: usize, n_bytes: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; grid_size]; grid_size];

    for line in &lines[..n_bytes] {
        let (xs, ys) = line.split_once(',').unwrap();
        let x = xs.parse::<usize>().unwrap();
        let y = ys.parse::<usize>().unwrap();
        grid[y][x] = '#';
    }

    grid
}

fn p02(lines: Vec<String>, grid_size: usize) {
    for i in 0..lines.len() {
        let grid = make_grid(&lines, grid_size, i);
        let start_pos = (0, 0);
        let end_pos = ((grid_size - 1) as isize, (grid_size - 1) as isize);

        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        let mut seen = HashSet::new();

        heap.push(State {
            pos: start_pos,
            steps: 0,
        });

        let mut reached_end = false;

        while let Some(state) = heap.pop() {
            if state.pos == end_pos {
                reached_end = true;
                break;
            }

            if seen.contains(&(state.pos)) {
                continue;
            }
            seen.insert(state.pos);

            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_pos = (state.pos.0 + dir.0, state.pos.1 + dir.1);
                if !in_bounds(
                    (next_pos.0 as i32, next_pos.1 as i32),
                    grid_size as i32,
                    grid_size as i32,
                ) {
                    continue;
                }
                let next_char = grid[next_pos.0 as usize][next_pos.1 as usize];
                if next_char == '#' {
                    continue;
                }

                heap.push(State {
                    pos: next_pos,
                    steps: state.steps + 1,
                });
            }
        }

        if !reached_end {
            println!("{}", lines[i - 1]);
            exit(0);
        }
    }
}

fn p01(lines: Vec<String>, grid_size: usize, n_bytes: usize) {
    let grid = make_grid(&lines, grid_size, n_bytes);
    let start_pos = (0, 0);
    let end_pos = ((grid_size - 1) as isize, (grid_size - 1) as isize);

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut seen = HashSet::new();

    heap.push(State {
        pos: start_pos,
        steps: 0,
    });

    while let Some(state) = heap.pop() {
        if state.pos == end_pos {
            println!("{}", state.steps);
            break;
        }

        if seen.contains(&(state.pos)) {
            continue;
        }
        seen.insert(state.pos);

        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next_pos = (state.pos.0 + dir.0, state.pos.1 + dir.1);
            if !in_bounds(
                (next_pos.0 as i32, next_pos.1 as i32),
                grid_size as i32,
                grid_size as i32,
            ) {
                continue;
            }
            let next_char = grid[next_pos.0 as usize][next_pos.1 as usize];
            if next_char == '#' {
                continue;
            }

            heap.push(State {
                pos: next_pos,
                steps: state.steps + 1,
            });
        }
    }
}

pub fn d18() {
    p01(get_lines("src/d18/in.txt"), 71, 1024);
    p02(get_lines("src/d18/in.txt"), 71);
}
