use core::panic;
use std::path::Path;

use crate::util::get_lines;

fn p2(lines: Vec<String>) {
    let mut sum = 0;

    for line in &lines {
        let mut n = line.parse::<i32>().unwrap();

        loop {
            n /= 3;
            n -= 2;

            if n <= 0 {
                break;
            }

            sum += n;
        }
    }

    println!("{}", sum);
}

fn p1(lines: Vec<String>) {
    let mut sum = 0;

    for line in &lines {
        let mut n = line.parse::<i32>().unwrap();
        n /= 3;
        n -= 2;
        sum += n;
    }

    println!("{}", sum);
}

pub fn d01() {
    let input_file = Path::new("src/d01/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
