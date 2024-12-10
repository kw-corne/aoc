use std::collections::HashSet;

use crate::util::get_lines;

fn make_grid(lines: &[String]) -> Vec<Vec<u32>> {
    let mut grid = vec![];

    for i in 0..lines.len() {
        let chars: Vec<u32> = lines[i].chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(chars);
    }

    grid
}

fn in_bounds(pos: (i32, i32), width: i32, height: i32) -> bool {
    pos.0 >= 0 && pos.0 < height && pos.1 >= 0 && pos.1 < width
}

fn reachable_9s(grid: &[Vec<u32>], pos: (i32, i32)) -> Vec<(i32, i32)> {
    let width = grid[0].len();
    let height = grid.len();

    let mut nines_seen = vec![];
    let mut stack = vec![pos];

    while let Some(p) = stack.pop() {
        let num = grid[p.0 as usize][p.1 as usize];
        if num == 9 {
            nines_seen.push(p);
        }

        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next_pos = (p.0 + dir.0, p.1 + dir.1);
            if !in_bounds(next_pos, width as i32, height as i32) {
                continue;
            }

            let next_num = grid[next_pos.0 as usize][next_pos.1 as usize];
            if next_num == num + 1 {
                stack.push(next_pos);
            }
        }
    }

    nines_seen
}

fn p01(lines: Vec<String>) {
    let mut score = 0;
    let grid = make_grid(&lines);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                score += reachable_9s(&grid, (i as i32, j as i32))
                    .iter()
                    .collect::<HashSet<_>>()
                    .len();
            }
        }
    }

    println!("{}", score);
}

fn p02(lines: Vec<String>) {
    let mut score = 0;
    let grid = make_grid(&lines);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                score += reachable_9s(&grid, (i as i32, j as i32)).len();
            }
        }
    }

    println!("{}", score);
}

pub fn d10() {
    p01(get_lines("src/d10/in.txt"));
    p02(get_lines("src/d10/in.txt"));
}
