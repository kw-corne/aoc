use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_lines(p: &str) -> Vec<String> {
    let file = File::open(p).unwrap();
    BufReader::new(file).lines().map_while(Result::ok).collect()
}

pub fn lines_to_2d_chars(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|s| s.chars().collect()).collect()
}

pub fn dump_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

pub fn get_adj_chars(grid: &[&[char]], i: usize, j: usize, diag: bool) -> Vec<(char, i8, i8)> {
    let mut adjacent_chars: Vec<(char, i8, i8)> = Vec::new();

    if i > 0 {
        adjacent_chars.push((grid[i - 1][j], -1, 0));

        if diag {
            if j > 0 {
                adjacent_chars.push((grid[i - 1][j - 1], -1, -1));
            }

            if j < grid[i].len() - 1 {
                adjacent_chars.push((grid[i - 1][j + 1], -1, 1));
            }
        }
    }

    if i < grid.len() - 1 {
        adjacent_chars.push((grid[i + 1][j], 1, 0));

        if diag {
            if j > 0 {
                adjacent_chars.push((grid[i + 1][j - 1], 1, -1));
            }

            if j < grid[i].len() - 1 {
                adjacent_chars.push((grid[i + 1][j + 1], 1, 1));
            }
        }
    }

    if j > 0 {
        adjacent_chars.push((grid[i][j - 1], 0, -1));
    }

    if j < grid[i].len() - 1 {
        adjacent_chars.push((grid[i][j + 1], 0, 1));
    }

    adjacent_chars
}
