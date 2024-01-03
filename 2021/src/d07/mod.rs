use std::path::Path;

use crate::util::get_lines;

fn p2(lines: Vec<String>) {
    let positions: Vec<i32> = lines[0]
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mean = ((positions.iter().sum::<i32>() as f32) / positions.len() as f32).round() as i32;

    fn fuel_to(from: i32, to: i32) -> i32 {
        let n = (to - from).abs();
        n * (n + 1) / 2
    }

    // one of these three?
    println!(
        "{}",
        positions
            .iter()
            .fold(0, |acc, pos| acc + fuel_to(*pos, mean))
    );
    println!(
        "{}",
        positions
            .iter()
            .fold(0, |acc, pos| acc + fuel_to(*pos, mean - 1))
    );
    println!(
        "{}",
        positions
            .iter()
            .fold(0, |acc, pos| acc + fuel_to(*pos, mean + 1))
    );
}

fn p1(lines: Vec<String>) {
    let mut positions: Vec<i32> = lines[0]
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    positions.sort();
    let middle = positions[positions.len() / 2];

    println!(
        "{}",
        positions
            .iter()
            .fold(0, |acc, pos| acc + (middle - pos).abs())
    );
}

pub fn d07() {
    let input_file = Path::new("src/d07/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
