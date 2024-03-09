use core::panic;
use std::path::Path;

use crate::util::get_lines;

fn p2(lines: Vec<String>) {
    let mut height = 0;

    for (i, c) in lines[0].chars().enumerate() {
        match c {
            '(' => height += 1,
            ')' => height -= 1,
            _ => panic!(),
        }

        if height == -1 {
            println!("{}", i + 1);
            break;
        }
    }
}

fn p1(lines: Vec<String>) {
    let mut height = 0;

    for c in lines[0].chars() {
        match c {
            '(' => height += 1,
            ')' => height -= 1,
            _ => panic!(),
        }
    }

    println!("{}", height)
}

pub fn d01() {
    let input_file = Path::new("src/d01/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
