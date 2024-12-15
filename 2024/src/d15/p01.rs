use std::{collections::HashMap, process::exit};

use crate::util::{dump_grid, get_lines};

fn make_grid(lines: &[String]) -> Vec<Vec<char>> {
    let mut grid = vec![];

    for i in 0..lines.len() {
        grid.push(lines[i].chars().collect());
    }

    grid
}

fn get_moves(lines: &[String]) -> Vec<(i32, i32)> {
    let mut moves = vec![];

    for line in lines {
        for ch in line.chars() {
            match ch {
                '<' => moves.push((0, -1)),
                'v' => moves.push((1, 0)),
                '>' => moves.push((0, 1)),
                '^' => moves.push((-1, 0)),
                _ => panic!(),
            }
        }
    }

    moves
}

fn get_grid_and_moves(lines: &[String]) -> (Vec<Vec<char>>, Vec<(i32, i32)>) {
    let grid_lines = lines
        .iter()
        .take_while(|x| !x.is_empty())
        .cloned()
        .collect::<Vec<_>>();

    let move_lines = &lines[grid_lines.len() + 1..];

    let grid = make_grid(&grid_lines);
    let moves = get_moves(move_lines);
    (grid, moves)
}

fn start_pos(grid: &[Vec<char>]) -> (i32, i32) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                return (i as i32, j as i32);
            }
        }
    }

    panic!()
}

fn move_boxes(grid: &mut [Vec<char>], mover_pos: (i32, i32), dir: (i32, i32)) {
    let mut pos = (mover_pos.0 + dir.0, mover_pos.1 + dir.1);
    let mut next_ch = grid[pos.0 as usize][pos.1 as usize];
    let mut boxes_to_push = 0;

    while next_ch == 'O' {
        boxes_to_push += 1;
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        next_ch = grid[pos.0 as usize][pos.1 as usize];
    }

    if boxes_to_push == 0 || next_ch != '.' {
        return;
    }

    pos = mover_pos;
    grid[pos.0 as usize][pos.1 as usize] = '.';
    pos = (pos.0 + dir.0, pos.1 + dir.1);
    grid[pos.0 as usize][pos.1 as usize] = '@';
    pos = (pos.0 + dir.0, pos.1 + dir.1);

    for _ in 0..boxes_to_push {
        grid[pos.0 as usize][pos.1 as usize] = 'O';
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
}

fn gps_sum(grid: &[Vec<char>]) -> i64 {
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                sum += 100 * i as i64 + j as i64
            }
        }
    }

    sum
}

pub fn p01(lines: Vec<String>) {
    let (mut grid, moves) = get_grid_and_moves(&lines);
    let mut pos = start_pos(&grid);

    for m in moves {
        let next_pos = (pos.0 + m.0, pos.1 + m.1);

        match grid[next_pos.0 as usize][next_pos.1 as usize] {
            '.' => {
                grid[pos.0 as usize][pos.1 as usize] = '.';
                pos = next_pos;
                grid[pos.0 as usize][pos.1 as usize] = '@';
            }
            'O' => {
                move_boxes(&mut grid, pos, m);
                pos = start_pos(&grid);
            }
            '#' => (),
            _ => panic!(),
        }
    }

    println!("{}", gps_sum(&grid));
}
