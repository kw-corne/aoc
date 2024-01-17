use anyhow::Result;
use std::collections::HashSet;
use std::fmt::Display;
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

pub fn lines_to_2d_ints(lines: &Vec<String>) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

pub fn dump_grid<T: Display>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

pub fn dump_hash_set_points(set: &HashSet<[usize; 2]>) {
    let xylen = set.iter().fold([usize::MIN, usize::MIN], |acc, point| {
        [acc[0].max(point[0]), acc[1].max(point[1])]
    });

    for i in 0..xylen[1] + 1 {
        for j in 0..xylen[0] + 1 {
            let c = match set.contains(&[j, i]) {
                true => "#",
                false => ".",
            };
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

pub fn get_adj_els<T: Copy>(
    grid: &Vec<Vec<T>>,
    i: usize,
    j: usize,
    diag: bool,
) -> Vec<(T, i8, i8)> {
    let mut adjacent_chars: Vec<(T, i8, i8)> = Vec::new();

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
