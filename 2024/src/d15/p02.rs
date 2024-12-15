use crate::util::dump_grid;
use std::collections::HashSet;
use std::fs::canonicalize;
use std::process::exit;

fn make_grid(lines: &[String]) -> Vec<Vec<char>> {
    let mut grid = vec![];

    for i in 0..lines.len() {
        grid.push(vec![]);
        for ch in lines[i].chars() {
            match ch {
                '#' => {
                    grid[i].push('#');
                    grid[i].push('#');
                }
                'O' => {
                    grid[i].push('[');
                    grid[i].push(']');
                }
                '.' => {
                    grid[i].push('.');
                    grid[i].push('.');
                }
                '@' => {
                    grid[i].push('@');
                    grid[i].push('.');
                }
                _ => panic!(),
            }
        }
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

fn can_box_move(grid: &Vec<Vec<char>>, box_pos: (i32, i32), dir: (i32, i32)) -> bool {
    let mut pos = box_pos;

    // Check horizontal
    if dir.1 != 0 {
        let mut next_ch = grid[pos.0 as usize][pos.1 as usize];
        while next_ch != '.' && next_ch != '#' {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            next_ch = grid[pos.0 as usize][pos.1 as usize];
        }

        return next_ch == '.';
    }

    // Check vertical
    let bx = get_box_from_one_pos(grid, box_pos);

    let next_left = (bx.0 .0 + dir.0, bx.0 .1);
    let next_right = (bx.1 .0 + dir.0, bx.1 .1);
    let next_ch_left = grid[next_left.0 as usize][next_left.1 as usize];
    let next_ch_right = grid[next_right.0 as usize][next_right.1 as usize];

    if next_ch_left == '#' || next_ch_right == '#' {
        return false;
    }

    match (next_ch_left, next_ch_right) {
        ('[', ']') => can_box_move(grid, next_left, dir),
        (']', '[') => can_box_move(grid, next_left, dir) && can_box_move(grid, next_right, dir),
        (']', _) => can_box_move(grid, next_left, dir),
        (_, '[') => can_box_move(grid, next_right, dir),
        _ => true,
    }
}

fn get_boxes_to_move_v(
    grid: &Vec<Vec<char>>,
    boxes: &mut HashSet<((i32, i32), (i32, i32))>,
    box_pos: (i32, i32),
    dir: (i32, i32),
) {
    let bx = get_box_from_one_pos(grid, box_pos);
    boxes.insert(bx);

    let next_left = (bx.0 .0 + dir.0, bx.0 .1);
    let next_right = (bx.1 .0 + dir.0, bx.1 .1);
    let next_ch_left = grid[next_left.0 as usize][next_left.1 as usize];
    let next_ch_right = grid[next_right.0 as usize][next_right.1 as usize];

    if next_ch_left == '#' || next_ch_right == '#' {
        return;
    }

    match (next_ch_left, next_ch_right) {
        ('[', ']') => get_boxes_to_move_v(grid, boxes, next_left, dir),
        (']', '[') => {
            get_boxes_to_move_v(grid, boxes, next_left, dir);
            get_boxes_to_move_v(grid, boxes, next_right, dir);
        }
        (']', _) => get_boxes_to_move_v(grid, boxes, next_left, dir),
        (_, '[') => get_boxes_to_move_v(grid, boxes, next_right, dir),
        _ => (),
    }
}

fn move_vert(grid: &mut Vec<Vec<char>>, mover_pos: (i32, i32), dir: (i32, i32)) {
    let mut boxes = HashSet::new();
    let bbb = (mover_pos.0 + dir.0, mover_pos.1 + dir.1);

    get_boxes_to_move_v(grid, &mut boxes, bbb, dir);

    let mut boxes2 = boxes.iter().collect::<Vec<_>>();
    boxes2.sort();

    if dir.0 == 1 {
        boxes2.reverse();
    }

    for b in boxes2 {
        let new_left = (b.0 .0 + dir.0, b.0 .1 + dir.1);
        let new_right = (b.1 .0 + dir.0, b.1 .1 + dir.1);

        grid[new_left.0 as usize][new_left.1 as usize] = '[';
        grid[b.0 .0 as usize][b.0 .1 as usize] = '.';

        grid[new_right.0 as usize][new_right.1 as usize] = ']';

        grid[b.1 .0 as usize][b.1 .1 as usize] = '.';
    }

    let atpos = (mover_pos.0 + dir.0, mover_pos.1 + dir.1);
    grid[mover_pos.0 as usize][mover_pos.1 as usize] = '.';
    grid[atpos.0 as usize][atpos.1 as usize] = '@';
}

fn move_horiz(grid: &mut Vec<Vec<char>>, mover_pos: (i32, i32), dir: (i32, i32)) {
    let mut pos = (mover_pos.0 + dir.0, mover_pos.1 + dir.1);
    let mut next_ch = grid[pos.0 as usize][pos.1 as usize];
    let mut boxes_to_push = 0;

    while next_ch == '[' || next_ch == ']' {
        boxes_to_push += 1;
        pos = (pos.0 + (dir.0 * 2), pos.1 + (dir.1 * 2));
        next_ch = grid[pos.0 as usize][pos.1 as usize];
    }

    if boxes_to_push == 0 || next_ch != '.' {
        return;
    }

    pos = mover_pos;
    grid[pos.0 as usize][pos.1 as usize] = '.';
    pos = (pos.0 + dir.0, pos.1 + dir.1);
    let atpos = pos;
    pos = (pos.0 + dir.0, pos.1 + dir.1);

    for _ in 0..boxes_to_push {
        move_box_h(grid, pos, dir);
        pos = (pos.0 + dir.0 * 2, pos.1 + dir.1 * 2);
    }

    grid[atpos.0 as usize][atpos.1 as usize] = '@';
}

fn get_box_from_one_pos(grid: &Vec<Vec<char>>, box_pos: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let box_left;
    let box_right;
    if grid[box_pos.0 as usize][box_pos.1 as usize] == '[' {
        box_left = box_pos;
        box_right = (box_pos.0, box_pos.1 + 1);
    } else {
        box_right = box_pos;
        box_left = (box_pos.0, box_pos.1 - 1);
    }
    (box_left, box_right)
}

fn move_box_h(grid: &mut Vec<Vec<char>>, box_pos: (i32, i32), dir: (i32, i32)) {
    let (box_left, box_right) = get_box_from_one_pos(grid, box_pos);

    let left_pos = (box_left.0 + dir.0, box_left.1 + dir.1);
    let right_pos = (box_right.0 + dir.0, box_right.1 + dir.1);

    if grid[box_left.0 as usize][box_left.1 as usize] == '[' {
        grid[box_left.0 as usize][box_left.1 as usize] = '.';
    }
    grid[left_pos.0 as usize][left_pos.1 as usize] = '[';

    if grid[box_right.0 as usize][box_right.1 as usize] == ']' {
        grid[box_right.0 as usize][box_right.1 as usize] = '.';
    }

    grid[right_pos.0 as usize][right_pos.1 as usize] = ']';
}

fn move_boxes(grid: &mut Vec<Vec<char>>, mover_pos: (i32, i32), dir: (i32, i32)) {
    let pos = (mover_pos.0 + dir.0, mover_pos.1 + dir.1);

    if can_box_move(grid, pos, dir) {
        if dir.1 != 0 {
            move_horiz(grid, mover_pos, dir);
        } else {
            move_vert(grid, mover_pos, dir);
        }
    }
}

fn gps_sum(grid: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '[' {
                sum += 100 * i as i64 + j as i64;
            }
        }
    }

    sum
}

pub fn p02(lines: Vec<String>) {
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
            ']' | '[' => {
                move_boxes(&mut grid, pos, m);
                pos = start_pos(&grid);
            }
            '#' => (),
            _ => panic!(),
        }
    }

    println!("{}", gps_sum(&grid));
}
