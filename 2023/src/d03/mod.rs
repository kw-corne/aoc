use std::path::Path;

use crate::util::{read_lines, InputLines};

fn make_grid(lines: InputLines) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in lines {
        let rline = line.expect("Couldnt read line");
        grid.push(vec![]);

        for c in rline.chars() {
            grid.last_mut().unwrap().push(c);
        }
    }

    grid
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn has_adj_symbol(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let mut adjacent_chars = Vec::new();

    if i > 0 {
        adjacent_chars.push(grid[i - 1][j]);

        if j > 0 {
            adjacent_chars.push(grid[i - 1][j - 1]);
        }

        if j < grid[i].len() - 1 {
            adjacent_chars.push(grid[i - 1][j + 1]);
        }
    }

    if i < grid.len() - 1 {
        adjacent_chars.push(grid[i + 1][j]);

        if j > 0 {
            adjacent_chars.push(grid[i + 1][j - 1]);
        }

        if j < grid[i].len() - 1 {
            adjacent_chars.push(grid[i + 1][j + 1]);
        }
    }

    if j > 0 {
        adjacent_chars.push(grid[i][j - 1]);
    }

    if j < grid[i].len() - 1 {
        adjacent_chars.push(grid[i][j + 1]);
    }

    for c in adjacent_chars {
        if is_symbol(c) {
            return true;
        }
    }

    return false;
}

fn p2(lines: InputLines) {}

fn p1(lines: InputLines) {
    let grid = make_grid(lines);
    let mut sum = 0;

    for i in 0..grid.len() {
        let mut j = 0;

        while j < grid[i].len() {
            if grid[i][j].is_numeric() {
                let mut num: u32 = grid[i][j].to_digit(10).unwrap();
                let mut seen_symbol = has_adj_symbol(&grid, i, j);
                j += 1;

                while j < grid[i].len() && grid[i][j].is_numeric() {
                    num *= 10;
                    num += grid[i][j].to_digit(10).unwrap();

                    if !seen_symbol {
                        seen_symbol = has_adj_symbol(&grid, i, j);
                    }
                    j += 1;
                }

                if seen_symbol {
                    sum += num;
                }
            } else {
                j += 1;
            }
        }
    }

    println!("Part 1: {sum}");
}

pub fn d03() {
    let input_file = Path::new("src/d03/in.txt");

    p1(read_lines(input_file).expect("Couldnt read lines of file"));
    // p2(read_lines(input_file).expect("Couldnt read lines of file"));
}
