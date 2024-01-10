use std::{
    collections::{HashSet, VecDeque},
    path::Path,
};

use crate::util::{get_adj_chars, get_lines, lines_to_2d_chars};

fn flood_fill(i: usize, j: usize, grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut basin: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    queue.push_back((i as isize, j as isize));
    basin.insert((i, j));

    while !queue.is_empty() {
        let n = queue.len();
        for _ in 0..n {
            let pos: (isize, isize) = queue.pop_front().unwrap();

            let adjacent_chars: Vec<(char, i8, i8)> =
                get_adj_chars(grid, pos.0 as usize, pos.1 as usize, false)
                    .into_iter()
                    .filter(|&c| c.0 != '9')
                    .collect();

            for adj_char in &adjacent_chars {
                let adj_pos = (pos.0 + adj_char.1 as isize, pos.1 + adj_char.2 as isize);

                if basin.insert((adj_pos.0 as usize, adj_pos.1 as usize)) {
                    queue.push_back(adj_pos);
                }
            }
        }
    }

    basin
}

fn p2(lines: Vec<String>) {
    let mut basins: Vec<HashSet<(usize, usize)>> = vec![];
    let grid = lines_to_2d_chars(&lines);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '9' && !basins.iter().any(|b| b.contains(&(i, j))) {
                basins.push(flood_fill(i, j, &grid));
            }
        }
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    println!("{}", basins.iter().take(3).fold(1, |acc, b| acc * b.len()));
}

fn p1(lines: Vec<String>) {
    let mut sum = 0;

    let grid = lines_to_2d_chars(&lines);

    for (i, line) in grid.iter().enumerate() {
        for (j, n) in line.iter().enumerate() {
            if get_adj_chars(&grid, i, j, false)
                .iter()
                .all(|&cell| cell.0 > *n)
            {
                sum += n.to_digit(10).unwrap() + 1;
            }
        }
    }

    println!("{}", sum);
}

pub fn d09() {
    let input_file = Path::new("src/d09/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
