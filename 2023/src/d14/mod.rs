use std::{collections::HashMap, path::Path};

use indexmap::{IndexMap, IndexSet};

use crate::util::{get_lines, lines_to_2d_chars};

fn roll_rock_dir(dish: &mut Vec<Vec<char>>, mut i: usize, mut j: usize, dir: char) {
    dish[i][j] = '.';

    match dir {
        'N' => {
            while i > 0 && dish[i - 1][j] == '.' {
                i -= 1;
            }
        }
        'E' => {
            while j < dish[0].len() - 1 && dish[i][j + 1] == '.' {
                j += 1;
            }
        }
        'S' => {
            while i < dish.len() - 1 && dish[i + 1][j] == '.' {
                i += 1;
            }
        }
        'W' => {
            while j > 0 && dish[i][j - 1] == '.' {
                j -= 1;
            }
        }
        _ => panic!("Invalid dir: {}", dir),
    }

    dish[i][j] = 'O';
}

fn roll_dish(dish: &mut Vec<Vec<char>>, dir: char) {
    let row_range: Vec<usize>;
    let col_range: Vec<usize>;

    let r = 0..dish.len();
    let c = 0..dish[0].len();

    match dir {
        'N' | 'W' => {
            row_range = r.collect();
            col_range = c.collect();
        }
        'S' | 'E' => {
            row_range = r.rev().collect();
            col_range = c.rev().collect();
        }
        _ => panic!("Invalid dir: {}", dir),
    }

    for i in row_range {
        for j in col_range.clone() {
            if dish[i][j] == 'O' {
                roll_rock_dir(dish, i, j, dir);
            }
        }
    }
}

fn north_load(dish: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;

    for i in 0..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][j] == 'O' {
                sum += dish.len() - i;
            }
        }
    }

    sum
}

fn p2(lines: Vec<String>) {
    let mut dish = lines_to_2d_chars(&lines);
    let mut cache: IndexSet<Vec<Vec<char>>> = IndexSet::new();
    let mut cycle_len: Option<i32> = None;
    let mut cycle_idx: Option<i32> = None;
    const CYCLES: i32 = 1000000000;

    for i in 0..CYCLES {
        if let Some(idx) = cache.get_index_of(&dish) {
            cycle_idx = Some(idx as i32);
            cycle_len = Some(i - idx as i32);
            break;
        }
        cache.insert(dish.clone());

        for d in ['N', 'W', 'S', 'E'] {
            roll_dish(&mut dish, d);
        }
    }

    let cycles_left = CYCLES - cycle_idx.unwrap();
    let remainder = cycles_left % cycle_len.unwrap();
    let state = cache.get_index((cycle_idx.unwrap() + remainder) as usize);

    println!("Part 2: {}", north_load(state.unwrap()));
}

fn p1(lines: Vec<String>) {
    let mut dish = lines_to_2d_chars(&lines);

    for i in 1..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][j] == 'O' {
                roll_rock_dir(&mut dish, i, j, 'N');
            }
        }
    }

    println!("Part 1: {}", north_load(&dish));
}

pub fn d14() {
    let input_file = Path::new("src/d14/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
