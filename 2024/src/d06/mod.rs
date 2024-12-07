use std::collections::{HashMap, HashSet};

use crate::util::{dump_grid, get_lines};

fn make_map(lines: &[String]) -> Vec<Vec<char>> {
    let mut map = vec![];

    for i in 0..lines.len() {
        map.push(lines[i].chars().collect());
    }

    map
}

fn start_pos(map: &[Vec<char>]) -> (i32, i32) {
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

fn go_until_oob_or_cache(
    mut pos: (i32, i32),
    mut dir: (i32, i32),
    map: &[Vec<char>],
) -> (bool, Vec<((i32, i32), (i32, i32))>) {
    let mut visited = vec![];

    loop {
        if visited.contains(&(pos, dir)) {
            return (true, visited);
        }
        visited.push((pos, dir));

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

    (false, visited)
}

fn p02(lines: Vec<String>) {
    let mut map = make_map(&lines);
    let pos = start_pos(&map);
    let dir = (-1, 0);
    let mut count = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '#' {
                map[i][j] = '#';

                let looped = go_until_oob_or_cache(pos, dir, &map).0;
                if looped {
                    count += 1;
                }

                map[i][j] = '.';
            }
        }
    }
    println!("{}", count);
}

fn p01(lines: Vec<String>) {
    let map = make_map(&lines);
    let pos = start_pos(&map);
    let dir = (-1, 0);

    let visited = go_until_oob_or_cache(pos, dir, &map)
        .1
        .iter()
        .map(|x| x.0)
        .collect::<HashSet<_>>()
        .len();

    println!("{}", visited);
}

pub fn d06() {
    p01(get_lines("src/d06/in.txt"));
    p02(get_lines("src/d06/in.txt"));
}
