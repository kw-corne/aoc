use std::{collections::HashSet, path::Path};

use crate::util::{get_adj_chars, get_lines};

type Maze = Vec<Vec<char>>;

fn mazeify(lines: &Vec<String>) -> Maze {
    let maze: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    maze
}

fn get_start_pos(maze: &Maze) -> (usize, usize) {
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No start pos?");
}

fn new_pos(curr: (usize, usize), dir: (i8, i8)) -> (usize, usize) {
    fn add(u: usize, i: i8) -> usize {
        if i.is_negative() {
            u - i.wrapping_abs() as u32 as usize
        } else {
            u + i as usize
        }
    }

    (add(curr.0, dir.0), add(curr.1, dir.1))
}

fn p2(lines: Vec<String>) {
    let mut maze = mazeify(&lines);
    let start_pos = get_start_pos(&maze);
    let mut loop_pos: HashSet<(usize, usize)> = HashSet::new();
    loop_pos.insert(start_pos);

    let mut p: (usize, usize) = (start_pos.0 as usize, start_pos.1 as usize);
    let mut steps = 0;

    let mut dir = (0, 0);
    let mut prev = dir;

    let mut s_out_dir = (0, 0);

    loop {
        if steps == 0 {
            let adj = get_adj_chars(&maze, start_pos.0 as usize, start_pos.1 as usize, false);
            for (c, i, j) in adj {
                if c == '|' || c == '-' {
                    dir = (i, j);
                    s_out_dir = dir;
                    p = new_pos(p, dir);
                    loop_pos.insert(p);
                    steps += 1;
                    break;
                }
            }
        }

        let c = maze[p.0][p.1];

        match (c, prev) {
            ('|', _) => (),
            ('-', _) => (),
            ('L', (1, 0)) => dir = (0, 1),
            ('L', (0, -1)) => dir = (-1, 0),
            ('J', (1, 0)) => dir = (0, -1),
            ('J', (0, 1)) => dir = (-1, 0),
            ('7', (-1, 0)) => dir = (0, -1),
            ('7', (0, 1)) => dir = (1, 0),
            ('F', (-1, 0)) => dir = (0, 1),
            ('F', (0, -1)) => dir = (1, 0),
            ('S', _) => {
                let r: char;
                match dir {
                    (0, -1) => match s_out_dir {
                        (0, -1) => r = '-',
                        (-1, 0) => r = 'L',
                        (1, 0) => r = 'F',
                        _ => unreachable!(),
                    }, // L
                    (0, 1) => match s_out_dir {
                        (-1, 0) => r = 'J',
                        (1, 0) => r = '7',
                        (0, 1) => r = '-',
                        _ => unreachable!(),
                    }, // R
                    (-1, 0) => match s_out_dir {
                        (-1, 0) => r = '|',
                        (0, 1) => r = 'F',
                        (0, -1) => r = '7',
                        _ => unreachable!(),
                    }, // U
                    (1, 0) => match s_out_dir {
                        (1, 0) => r = '|',
                        (0, 1) => r = 'L',
                        (0, -1) => r = 'J',
                        _ => unreachable!(),
                    }, // D
                    _ => unreachable!(),
                }
                maze[p.0][p.1] = r;
                break;
            }
            _ => panic!("Bad char {} at: {}-{}", c, p.0, p.1),
        }

        p = new_pos(p, dir);
        loop_pos.insert(p);
        steps += 1;
        prev = dir;
    }

    let mut in_loop = false;
    let mut total = 0;

    let mut i = 0;

    while i < maze.len() {
        let mut j = 0;
        while j < maze[i].len() {
            let c = maze[i][j];

            let has = loop_pos.contains(&(i, j));

            match (c, has) {
                (_, false) => {
                    if in_loop {
                        total += 1
                    }
                }
                ('|', true) => in_loop = !in_loop,
                ('L', true) => {
                    j += 1;
                    while maze[i][j] == '-' {
                        j += 1;
                    }

                    if maze[i][j] == '7' {
                        in_loop = !in_loop;
                    }
                }
                ('F', true) => {
                    j += 1;
                    while maze[i][j] == '-' {
                        j += 1;
                    }

                    if maze[i][j] == 'J' {
                        in_loop = !in_loop;
                    }
                }
                _ => (),
            }
            j += 1;
        }
        i += 1;
    }

    println!("Part 2: {}", total);
}

fn p1(lines: Vec<String>) {
    let maze = mazeify(&lines);
    let start_pos = get_start_pos(&maze);

    let mut p: (usize, usize) = (start_pos.0 as usize, start_pos.1 as usize);
    let mut steps = 0;

    let mut dir = (0, 0);
    let mut prev = dir;

    loop {
        if steps == 0 {
            let adj = get_adj_chars(&maze, start_pos.0 as usize, start_pos.1 as usize, false);
            for (c, i, j) in adj {
                if c == '|' || c == '-' {
                    dir = (i, j);
                    p = new_pos(p, dir);
                    steps += 1;
                    break;
                }
            }
        }

        let c = maze[p.0][p.1];

        match (c, prev) {
            ('|', _) => (),
            ('-', _) => (),
            ('L', (1, 0)) => dir = (0, 1),
            ('L', (0, -1)) => dir = (-1, 0),
            ('J', (1, 0)) => dir = (0, -1),
            ('J', (0, 1)) => dir = (-1, 0),
            ('7', (-1, 0)) => dir = (0, -1),
            ('7', (0, 1)) => dir = (1, 0),
            ('F', (-1, 0)) => dir = (0, 1),
            ('F', (0, -1)) => dir = (1, 0),
            ('S', _) => break,
            _ => panic!("Bad char {} at: {}-{}", c, p.0, p.1),
        }

        p = new_pos(p, dir);
        steps += 1;
        prev = dir;
    }

    println!("Part 1: {}", steps / 2);
}

pub fn d10() {
    let input_file = Path::new("src/d10/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
