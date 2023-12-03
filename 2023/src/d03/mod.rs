use std::{
    collections::{hash_map::Entry, HashMap},
    path::Path,
};

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

fn get_adj_chars(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<char> {
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

    adjacent_chars
}

type CellInfo = (char, usize, usize);
fn get_adj_chars2(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<CellInfo> {
    let mut adjacent_chars: Vec<CellInfo> = Vec::new();

    if i > 0 {
        adjacent_chars.push((grid[i - 1][j], i - 1, j));

        if j > 0 {
            adjacent_chars.push((grid[i - 1][j - 1], i - 1, j - 1));
        }

        if j < grid[i].len() - 1 {
            adjacent_chars.push((grid[i - 1][j + 1], i - 1, j + 1));
        }
    }

    if i < grid.len() - 1 {
        adjacent_chars.push((grid[i + 1][j], i + 1, j));

        if j > 0 {
            adjacent_chars.push((grid[i + 1][j - 1], i + 1, j - 1));
        }

        if j < grid[i].len() - 1 {
            adjacent_chars.push((grid[i + 1][j + 1], i + 1, j + 1));
        }
    }

    if j > 0 {
        adjacent_chars.push((grid[i][j - 1], i, j - 1));
    }

    if j < grid[i].len() - 1 {
        adjacent_chars.push((grid[i][j + 1], i, j + 1));
    }

    adjacent_chars
}

fn has_adj_symbol(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let adj_chars = get_adj_chars(grid, i, j);

    for c in adj_chars {
        if !c.is_numeric() && c != '.' {
            return true;
        }
    }

    false
}

fn has_adj_ast(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(usize, usize)> {
    let adj_chars = get_adj_chars2(grid, i, j);

    for c in adj_chars {
        if c.0 == '*' {
            return Some((c.1, c.2));
        }
    }

    None
}

fn p2(lines: InputLines) {
    let grid = make_grid(lines);
    let mut sum = 0;
    let mut ast_c: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..grid.len() {
        let mut j = 0;

        while j < grid[i].len() {
            if !grid[i][j].is_numeric() {
                j += 1;
                continue;
            }

            let mut num: u32 = grid[i][j].to_digit(10).unwrap();
            let mut has_ast = has_adj_ast(&grid, i, j);

            j += 1;

            while j < grid[i].len() && grid[i][j].is_numeric() {
                num *= 10;
                num += grid[i][j].to_digit(10).unwrap();

                if has_ast == None {
                    has_ast = has_adj_ast(&grid, i, j);
                }

                j += 1;
            }

            if let Some(t) = has_ast {
                let vc = ast_c.entry(t);
                match vc {
                    Entry::Occupied(mut o) => {
                        let v = o.get_mut();
                        v.push(num);
                    }
                    Entry::Vacant(v) => {
                        v.insert(vec![num]);
                    }
                }
            }
        }
    }

    for nums in ast_c.values() {
        if nums.len() != 2 {
            continue;
        }

        sum += nums[0] * nums[1];
    }

    println!("Part 2: {sum}");
}

fn p1(lines: InputLines) {
    let grid = make_grid(lines);
    let mut sum = 0;

    for i in 0..grid.len() {
        let mut j = 0;

        while j < grid[i].len() {
            if !grid[i][j].is_numeric() {
                j += 1;
                continue;
            }
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
        }
    }

    println!("Part 1: {sum}");
}

pub fn d03() {
    let input_file = Path::new("src/d03/in.txt");

    p1(read_lines(input_file).expect("Couldnt read lines of file"));
    p2(read_lines(input_file).expect("Couldnt read lines of file"));
}
