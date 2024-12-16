use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use crate::util::{get_lines, lines_to_2d_chars, pos_in_grid};

#[derive(Debug, Eq, Clone, Copy, PartialEq, Hash)]
struct State {
    pos: (isize, isize),
    dir: (isize, isize),
    score: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

fn p02(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);
    let start_pos = pos_in_grid(&grid, 'S');
    let end_pos = pos_in_grid(&grid, 'E');

    let mut path = HashMap::new();
    let mut best_end_score = i32::MAX;
    let mut best_score_per_state = HashMap::new();
    let mut end_pos_states = HashSet::new();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        pos: start_pos,
        dir: (0, 1),
        score: 0,
    });

    while let Some(state) = heap.pop() {
        let best_score_this_pos = *best_score_per_state
            .get(&(state.pos, state.dir))
            .unwrap_or(&i32::MAX);

        if state.score > best_score_this_pos {
            continue;
        }

        if state.pos == end_pos {
            if state.score > best_end_score {
                break;
            }
            end_pos_states.insert((state.pos, state.dir));
            best_end_score = state.score;
        }

        let possible_next_states = [
            State {
                pos: (state.pos.0 + state.dir.0, state.pos.1 + state.dir.1),
                dir: state.dir,
                score: state.score + 1,
            },
            State {
                pos: state.pos,
                dir: (state.dir.1, -state.dir.0),
                score: state.score + 1000,
            },
            State {
                pos: state.pos,
                dir: (-state.dir.1, state.dir.0),
                score: state.score + 1000,
            },
        ];

        for next_state in possible_next_states {
            if grid[next_state.pos.0 as usize][next_state.pos.1 as usize] == '#' {
                continue;
            }

            let next_posdir = (next_state.pos, next_state.dir);
            let posdir = (state.pos, state.dir);

            let best_score_next_pos = *best_score_per_state.get(&next_posdir).unwrap_or(&i32::MAX);

            if next_state.score > best_score_next_pos {
                continue;
            }
            if next_state.score < best_score_next_pos {
                best_score_per_state.insert(next_posdir, next_state.score);
                path.insert(next_posdir, HashSet::new());
            }

            path.get_mut(&next_posdir).unwrap().insert(posdir);
            heap.push(next_state);
        }
    }

    let mut states = end_pos_states.iter().collect::<VecDeque<_>>();
    let mut on_fastest_path = HashSet::new();

    while let Some(posdir) = states.pop_front() {
        let path_for_current = path.get(&posdir).unwrap();

        for state in path_for_current {
            if on_fastest_path.contains(&state) {
                continue;
            }
            on_fastest_path.insert(state);
            states.push_back(state);
        }
    }

    let unique_pos = on_fastest_path
        .iter()
        .map(|x| (x.0))
        .collect::<HashSet<_>>()
        .len()
        + 1; // end pos
    println!("{}", unique_pos);
}

fn p01(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);
    let start_pos = pos_in_grid(&grid, 'S');
    let end_pos = pos_in_grid(&grid, 'E');

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        pos: start_pos,
        dir: (0, 1),
        score: 0,
    });
    let mut seen = HashSet::new();

    while let Some(state) = heap.pop() {
        if state.pos == end_pos {
            println!("{}", state.score);
            break;
        }

        if seen.contains(&(state.pos, state.dir)) {
            continue;
        }
        seen.insert((state.pos, state.dir));

        let next_in_curr_dir = (state.pos.0 + state.dir.0, state.pos.1 + state.dir.1);
        if grid[next_in_curr_dir.0 as usize][next_in_curr_dir.1 as usize] != '#' {
            heap.push(State {
                pos: next_in_curr_dir,
                dir: state.dir,
                score: state.score + 1,
            });
        }

        // clockwise turn
        heap.push(State {
            pos: state.pos,
            dir: (state.dir.1, -state.dir.0),
            score: state.score + 1000,
        });

        // counter clockwise turn
        heap.push(State {
            pos: state.pos,
            dir: (-state.dir.1, state.dir.0),
            score: state.score + 1000,
        });
    }
}

pub fn d16() {
    p01(get_lines("src/d16/in.txt"));
    p02(get_lines("src/d16/in.txt"));
}
