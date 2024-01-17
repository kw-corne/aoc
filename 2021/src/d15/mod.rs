use std::cmp::Ordering;
use std::collections::HashSet;
use std::{collections::BinaryHeap, path::Path};

use crate::util::{get_adj_els, get_lines, lines_to_2d_chars, lines_to_2d_ints};

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    risk: u32,
    pos: (isize, isize),
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

fn p2(lines: Vec<String>) {
    let grid = lines_to_2d_ints(&lines);

    let og_i = grid.len() as isize;
    let og_j = grid[0].len() as isize;
    let max_i = og_i * 5;
    let max_j = og_j * 5;

    let start_pos = (0, 0);
    let end_pos = (max_i - 1, max_j - 1);

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut seen: HashSet<(isize, isize)> = HashSet::new();

    fn in_grid(pos: &(isize, isize), ui: isize, uj: isize) -> bool {
        (pos.0 >= 0 && pos.0 < ui) && (pos.1 >= 0 && pos.1 < uj)
    }

    heap.push(State {
        risk: 0,
        pos: start_pos,
    });

    while let Some(state) = heap.pop() {
        if state.pos == end_pos {
            println!("{}", state.risk);
            break;
        }

        if seen.contains(&state.pos) {
            continue;
        }
        seen.insert(state.pos);

        for dir in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let next_pos = (state.pos.0 + dir.0, state.pos.1 + dir.1);
            if !in_grid(&next_pos, max_i, max_j) {
                continue;
            }

            let i_base = next_pos.0 % og_i;
            let j_base = next_pos.1 % og_j;
            let mut next_risk = grid[i_base as usize][j_base as usize] as isize
                + ((next_pos.0 / og_i) + (next_pos.1 / og_j));

            if next_risk >= 10 {
                next_risk %= 9;
            }

            heap.push(State {
                risk: state.risk + next_risk as u32,
                pos: next_pos,
            })
        }
    }
}

fn p1(lines: Vec<String>) {
    let grid = lines_to_2d_ints(&lines);
    let start_pos = (0, 0);
    let end_pos = (grid.len() as isize - 1, grid[0].len() as isize - 1);

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut seen: HashSet<(isize, isize)> = HashSet::new();

    heap.push(State {
        risk: 0,
        pos: start_pos,
    });

    while let Some(state) = heap.pop() {
        if state.pos == end_pos {
            println!("{}", state.risk);
            break;
        }

        if seen.contains(&state.pos) {
            continue;
        }
        seen.insert(state.pos);

        let adjacent_elements =
            get_adj_els(&grid, state.pos.0 as usize, state.pos.1 as usize, false);

        for adj_el in &adjacent_elements {
            let next_i = state.pos.0 + adj_el.1 as isize;
            let next_j = state.pos.1 + adj_el.2 as isize;
            let next_risk = grid[next_i as usize][next_j as usize];

            heap.push(State {
                risk: state.risk + next_risk as u32,
                pos: (next_i, next_j),
            });
        }
    }
}

pub fn d15() {
    let input_file = Path::new("src/d15/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
