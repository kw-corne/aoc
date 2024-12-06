use std::collections::{HashMap, HashSet};

use crate::util::{dump_grid, get_lines};

fn make_map(lines: &[String]) -> Vec<Vec<char>> {
    let mut map = vec![];

    for i in 0..lines.len() {
        map.push(lines[i].chars().collect());
    }

    map
}

fn start_pos(map: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' {
                return (i as i32, j as i32);
            }
        }
    }

    unreachable!()
}

fn in_bounds(pos: (i32, i32), height: i32, width: i32) -> bool {
    pos.0 >= 0 && pos.0 < height && pos.1 >= 0 && pos.1 < width
}

fn p02(lines: Vec<String>) {
    let map = make_map(&lines);
    let mut pos = start_pos(&map);
    let mut visited = HashSet::new();

    let mut dir = (-1, 0);

    loop {
        visited.insert((pos, dir));
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

        if !in_bounds(next_pos, map.len() as i32, map[0].len() as i32) {
            break;
        }

        let on_next_pos = map[next_pos.0 as usize][next_pos.1 as usize];

        match on_next_pos {
            '#' => dir = (dir.1, -dir.0),
            '.' | '^' => pos = next_pos,
            _ => (),
        }
    }

    let mut sum = 0;
    let mut lel = HashSet::new();

    for seen in &visited {
        let dright = (seen.1 .1, -seen.1 .0);
        let next_pos = (seen.0 .0 + dright.0, seen.0 .1 + dright.1);

        if !in_bounds(next_pos, map.len() as i32, map[0].len() as i32) {
            break;
        }

        if visited.contains(&(next_pos, dright)) {
            lel.insert(next_pos);
        }
    }

    dbg!(&lel);
    println!("{}", lel.len());
}

fn p01(lines: Vec<String>) {
    let map = make_map(&lines);
    let mut pos = start_pos(&map);
    let mut visited = HashSet::new();

    let mut dir = (-1, 0);

    loop {
        visited.insert(pos);
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

        if !in_bounds(next_pos, map.len() as i32, map[0].len() as i32) {
            break;
        }

        let on_next_pos = map[next_pos.0 as usize][next_pos.1 as usize];

        match on_next_pos {
            '#' => dir = (dir.1, -dir.0),
            '.' | '^' => pos = next_pos,
            _ => (),
        }
    }

    println!("{}", visited.len());
}

pub fn d06() {
    p01(get_lines("src/d06/in.txt"));
    p02(get_lines("src/d06/dbg.txt"));
}
