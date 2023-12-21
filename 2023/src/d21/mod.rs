use std::{collections::HashSet, path::Path};

use crate::util::{dump_grid, get_adj_chars, get_lines, lines_to_2d_chars};

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let mut grid = lines_to_2d_chars(&lines);
    let start_pos = ((grid.len() / 2) as i32, (grid[0].len() / 2) as i32);
    grid[start_pos.0 as usize][start_pos.1 as usize] = '.';
    println!("{} {}", grid.len(), grid[0].len());

    let mut points: Vec<(i32, i32)> = vec![start_pos];
    let n_steps = 64;

    for i in 0..n_steps {
        let n_points = points.len();
        let mut dots: HashSet<(i32, i32)> = HashSet::new();
        let mut circles: HashSet<(i32, i32)> = HashSet::new();

        for _ in 0..n_points {
            let point = points.pop().unwrap();
            let adj_chars = get_adj_chars(&grid, point.0 as usize, point.1 as usize, false);
            dots.insert(point.clone());

            for (ch, i, j) in adj_chars {
                if ch == '.' {
                    let new_i = point.0 + i as i32;
                    let new_j = point.1 + j as i32;
                    circles.insert((new_i as i32, new_j as i32));
                }
            }
        }

        for p in &dots {
            grid[p.0 as usize][p.1 as usize] = '.';
        }

        for p in &circles {
            grid[p.0 as usize][p.1 as usize] = 'O';
            points.push(p.clone());
        }

        if i == n_steps - 1 {
            println!("Part 1: {} {}", circles.len(), i);
        }
    }
}

pub fn d21() {
    let input_file = Path::new("src/d21/in.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
