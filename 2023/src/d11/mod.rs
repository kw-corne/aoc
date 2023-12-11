use std::path::Path;

use crate::util::get_lines;

fn get_empty_rows_and_cols(universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
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

    (empty_rows, empty_cols)
}

fn get_galaxies(universe: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut galaxies: Vec<(i32, i32)> = vec![];
    for i in 0..universe.len() {
        for j in 0..universe[i].len() {
            if universe[i][j] == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    galaxies
}

fn dist(a: (i32, i32), b: (i32, i32)) -> i64 {
    (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()
}

fn dist_sum(universe: &Vec<Vec<char>>, space: i32) -> i64 {
    let (empty_rows, empty_cols) = get_empty_rows_and_cols(&universe);
    let mut galaxies = get_galaxies(&universe);

    fn bigger_than_n(n: i32, nums: &Vec<usize>) -> i32 {
        let mut c = 0;

        for num in nums {
            if n > *num as i32 {
                c += 1;
            }
        }

        c
    }

    for i in 0..galaxies.len() {
        let nr = bigger_than_n(galaxies[i].0, &empty_rows);
        let nc = bigger_than_n(galaxies[i].1, &empty_cols);

        galaxies[i].0 = galaxies[i].0 - nr + (space * nr);
        galaxies[i].1 = galaxies[i].1 - nc + (space * nc);
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += dist(galaxies[i], galaxies[j]);
        }
    }
    sum
}

fn p2(lines: Vec<String>) {
    let universe: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    println!("Part 2: {}", dist_sum(&universe, 1000000));
}

fn p1(lines: Vec<String>) {
    let universe: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    println!("Part 1: {}", dist_sum(&universe, 2));
}

pub fn d11() {
    let input_file = Path::new("src/d11/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
