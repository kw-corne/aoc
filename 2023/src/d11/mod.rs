use std::path::Path;

use crate::util::get_lines;

fn make_universe(lines: &Vec<String>) -> (Vec<Vec<char>>, Vec<(i32, i32)>) {
    let mut universe: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    for i in 0..universe.len() {
        if universe[i].iter().all(|c| *c == '.') {
            empty_rows.push(i);
        }
    }

    for j in 0..universe[0].len() {
        let mut is_empty = true;

        for i in 0..universe.len() {
            if universe[i][j] != '.' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            empty_cols.push(j);
        }
    }

    for (i, row) in empty_rows.iter().enumerate() {
        universe.insert(row + i, vec!['.'; universe[0].len()]);
    }

    for (i, col) in empty_cols.iter().enumerate() {
        for j in 0..universe.len() {
            universe[j].insert(*col + i, '.');
        }
    }

    let mut galaxies: Vec<(i32, i32)> = vec![];
    for i in 0..universe.len() {
        for j in 0..universe[i].len() {
            if universe[i][j] == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    (universe, galaxies)
}

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let (universe, galaxies) = make_universe(&lines);

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += dist(galaxies[i], galaxies[j]);
        }
    }

    println!("Part 1: {}", sum);
}

pub fn d11() {
    let input_file = Path::new("src/d11/in.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
