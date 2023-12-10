use std::path::Path;

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

fn p2(lines: Vec<String>) {}

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
                }
            }
        }

        let c = maze[p.0][p.1];

        match (c, prev) {
            ('|', _) => (),
            ('-', _) => (),
            ('L', (1, 0)) => dir = (0, 1),   // from up
            ('L', (0, -1)) => dir = (-1, 0), // from right
            ('J', (1, 0)) => dir = (0, -1),  // from up
            ('J', (0, 1)) => dir = (-1, 0),  // from left
            ('7', (-1, 0)) => dir = (0, -1), // from down
            ('7', (0, 1)) => dir = (1, 0),   // from left
            ('F', (-1, 0)) => dir = (0, 1),  // from down
            ('F', (0, -1)) => dir = (1, 0),  // from right
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
