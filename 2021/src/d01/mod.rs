use std::path::Path;

use crate::util::get_lines;

fn p2(lines: Vec<String>) {
    let mut total = 0;

    for i in 3..lines.len() {
        let prev = lines[i - 3].parse::<i32>().unwrap();
        let curr = lines[i].parse::<i32>().unwrap();

        if curr > prev {
            total += 1;
        }
    }

    println!("{}", total);
}

fn p1(lines: Vec<String>) {
    let mut total = 0;

    for i in 1..lines.len() {
        let prev = lines[i - 1].parse::<i32>().unwrap();
        let curr = lines[i].parse::<i32>().unwrap();

        if curr > prev {
            total += 1;
        }
    }

    println!("{}", total);
}

pub fn d01() {
    let input_file = Path::new("src/d01/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
