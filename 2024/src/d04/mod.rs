use std::collections::HashMap;

use crate::util::{dump_grid, get_adj_chars, get_lines, lines_to_2d_chars};

fn is_xmas_cross(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let adj_chars = get_adj_chars(grid, i, j, true);

    let ul = adj_chars.iter().find(|x| x.1 == -1 && x.2 == -1);
    let ur = adj_chars.iter().find(|x| x.1 == -1 && x.2 == 1);
    let dl = adj_chars.iter().find(|x| x.1 == 1 && x.2 == -1);
    let dr = adj_chars.iter().find(|x| x.1 == 1 && x.2 == 1);

    match (ul, ur, dl, dr) {
        (Some(ull), Some(urr), Some(dll), Some(drr)) => {
            if ull.0 != 'M' && ull.0 != 'S' {
                return false;
            }
            if urr.0 != 'M' && urr.0 != 'S' {
                return false;
            }
            if dll.0 != 'M' && dll.0 != 'S' {
                return false;
            }
            if drr.0 != 'M' && drr.0 != 'S' {
                return false;
            }

            // if (ull.0 != drr.0) && (dll.0 != urr.0) {}
            return (ull.0 != drr.0) && (dll.0 != urr.0);
        }
        _ => false,
    }
}

fn p02(lines: Vec<String>) {
    let mut sum = 0;
    let char_grid = lines_to_2d_chars(&lines);

    for i in 0..char_grid.len() {
        for j in 0..char_grid[i].len() {
            if char_grid[i][j] == 'A' {
                if is_xmas_cross(&char_grid, i, j) {
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);
}

fn next_in_dir(grid: &Vec<Vec<char>>, i: usize, j: usize, didj: (i8, i8)) -> Option<char> {
    let max_i = grid.len() as i32;
    let max_j = grid[0].len() as i32;

    let i2 = (i as i32) + didj.0 as i32;
    let j2 = (j as i32) + didj.1 as i32;

    if i2 >= 0 && i2 < max_i && j2 >= 0 && j2 < max_j {
        return Some(grid[i2 as usize][j2 as usize]);
    }

    None
}

fn xmas_count(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    let adj_chars = get_adj_chars(grid, i, j, true);

    for (ch, ii, jj) in adj_chars {
        if ch == 'M' {
            let ch_a = next_in_dir(grid, i, j, (ii * 2, jj * 2));
            let ch_s = next_in_dir(grid, i, j, (ii * 3, jj * 3));

            match (ch_a, ch_s) {
                (Some(a), Some(s)) if a == 'A' && s == 'S' => count += 1,
                _ => (),
            }
        }
    }

    count
}

fn p01(lines: Vec<String>) {
    let mut sum = 0;
    let char_grid = lines_to_2d_chars(&lines);

    for i in 0..char_grid.len() {
        for j in 0..char_grid[i].len() {
            if char_grid[i][j] == 'X' {
                sum += xmas_count(&char_grid, i, j);
            }
        }
    }

    println!("{}", sum);
}

pub fn d04() {
    p01(get_lines("src/d04/in.txt"));
    p02(get_lines("src/d04/in.txt"));
}
