use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub type InputLines = Lines<BufReader<File>>;

pub fn read_lines(p: &Path) -> Result<InputLines> {
    let file = File::open(p)?;
    Ok(BufReader::new(file).lines())
}

pub fn get_lines(p: &Path) -> Vec<String> {
    let file = File::open(p).unwrap();
    BufReader::new(file).lines().into_iter().flatten().collect()
}

pub fn lines_to_2d_chars(lines: &Vec<String>) -> Vec<Vec<char>> {
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

pub fn slice_col<T>(grid: &Vec<Vec<T>>, col: usize) -> Vec<&T> {
    let mut s: Vec<&T> = vec![];

    for i in 0..grid[0].len() {
        s.push(&grid[i][col]);
    }

    s
}

pub fn get_adj_chars(grid: &Vec<Vec<char>>, i: usize, j: usize, diag: bool) -> Vec<(char, i8, i8)> {
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
