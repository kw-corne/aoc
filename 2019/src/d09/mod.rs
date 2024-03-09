use core::panic;
use std::path::Path;

use crate::util::get_lines;

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let instructions: Vec<i32> = lines[0]
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    dbg!(instructions);
}

pub fn d09() {
    let input_file = Path::new("src/d09/dbg.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
