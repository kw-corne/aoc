use std::{collections::HashMap, path::Path};

use crate::util::get_lines;

fn p2(lines: Vec<String>) {
    let mut fish_day = [0; 9];

    for n in lines[0].split(',') {
        fish_day[n.parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..256 {
        fish_day.rotate_left(1);
        fish_day[6] += fish_day[8];
    }

    println!("{}", fish_day.iter().sum::<i64>());
}

fn p1(lines: Vec<String>) {
    let mut fish: Vec<i32> = lines[0]
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    for _ in 0..80 {
        for i in 0..fish.len() {
            match fish[i] {
                0 => {
                    fish[i] = 6;
                    fish.push(8);
                }
                _ => fish[i] -= 1,
            }
        }
    }

    println!("{}", fish.len());
}

pub fn d06() {
    let input_file = Path::new("src/d06/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
