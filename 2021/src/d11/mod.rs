use std::{collections::VecDeque, path::Path};

use crate::util::{dump_grid, get_adj_els, get_lines, lines_to_2d_ints};

fn all_zero(grid: &Vec<Vec<i32>>) -> bool {
    grid.iter().all(|row| row.iter().all(|n| *n == 0))
}

fn p2(lines: Vec<String>) {
    let mut grid = lines_to_2d_ints(&lines);
    let mut flash_queue: VecDeque<(usize, usize)> = VecDeque::new();

    for ii in 0..1000 {
        if all_zero(&grid) {
            println!("{}", ii);
            break;
        }

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                grid[i][j] += 1;

                if grid[i][j] >= 10 {
                    flash_queue.push_back((i, j));
                    grid[i][j] = 0;
                }
            }
        }

        while !flash_queue.is_empty() {
            let n = flash_queue.len();

            for _ in 0..n {
                let pos = flash_queue.pop_front().unwrap();
                for adj in get_adj_els(&grid, pos.0, pos.1, true) {
                    let i = pos.0.checked_add_signed(adj.1 as isize).unwrap();
                    let j = pos.1.checked_add_signed(adj.2 as isize).unwrap();

                    if grid[i][j] == 0 {
                        continue; // already flashed this iter
                    }

                    grid[i][j] += 1;

                    if grid[i][j] == 10 {
                        flash_queue.push_back((i, j));
                        grid[i][j] = 0;
                    }
                }
            }
        }
    }
}

fn p1(lines: Vec<String>) {
    let mut grid = lines_to_2d_ints(&lines);
    let mut flash_queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut n_flashes = 0;

    for _ in 0..100 {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                grid[i][j] += 1;

                if grid[i][j] >= 10 {
                    flash_queue.push_back((i, j));
                    grid[i][j] = 0;
                }
            }
        }

        while !flash_queue.is_empty() {
            let n = flash_queue.len();

            for _ in 0..n {
                let pos = flash_queue.pop_front().unwrap();
                n_flashes += 1;
                for adj in get_adj_els(&grid, pos.0, pos.1, true) {
                    let i = pos.0.checked_add_signed(adj.1 as isize).unwrap();
                    let j = pos.1.checked_add_signed(adj.2 as isize).unwrap();

                    if grid[i][j] == 0 {
                        continue; // already flashed this iter
                    }

                    grid[i][j] += 1;

                    if grid[i][j] == 10 {
                        flash_queue.push_back((i, j));
                        grid[i][j] = 0;
                    }
                }
            }
        }
    }

    println!("{}", n_flashes);
}

pub fn d11() {
    let input_file = Path::new("src/d11/in.txt");

    // p1(get_lines(input_file));
    p2(get_lines(input_file));
}
