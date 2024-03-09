use std::{collections::HashSet, path::Path};

use crate::util::{get_adj_chars, get_lines, lines_to_2d_chars};

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);

    let end_pos = (grid.len() - 1, grid[0].len() - 2);
    let mut stack: Vec<(usize, usize)> = vec![(0, 1)];
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    while !stack.is_empty() {
        let pos = stack.pop().unwrap();

        if seen.contains(&pos) {
            continue;
        }
        seen.insert(pos);

        if pos == end_pos {
            println!("reached end");
            break;
        }

        for (c, i, j) in get_adj_chars(&grid, pos.0, pos.1, false) {
            let new_i = (pos.0 as i32) + (i as i32);
            let new_j = (pos.1 as i32) + (j as i32);
            let new_pos = (new_i as usize, new_j as usize);

            let push_it = {
                match c {
                    '#' => false,
                    '.' => true,
                    '>' | '<' | '^' | 'v' => match (c, (i, j)) {
                        ('>', (0, 1)) => true,
                        ('<', (0, -1)) => true,
                        ('^', (-1, 0)) => true,
                        ('v', (1, 0)) => true,
                        _ => false,
                    },
                    _ => panic!(),
                }
            };

            if push_it {
                stack.push(new_pos);
            }
        }
    }
}

pub fn d23() {
    let input_file = Path::new("src/d23/dbg.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
