use std::{
    fs::File,
    io::{BufReader, Lines},
    path::Path,
};

use crate::util::read_lines;

type InputLines = Lines<BufReader<File>>;

fn num_from_line(line: &str) -> i32 {
    let first_num = line
        .chars()
        .find(|c| c.is_numeric())
        .expect("Couldnt find first num");

    let last_num = line
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .expect("Couldnt find last num");

    format!("{}{}", first_num, last_num)
        .parse::<i32>()
        .expect("Cant convert nums to 1 int")
}

fn p2(lines: InputLines) {
    let nums = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = &[
        "o1e", "t2o", "t3e", "f4r", "f5e", "s6x", "s7n", "e8t", "n9e",
    ];

    let mut sum = 0;
    for res_line in lines {
        let mut line: String = res_line.expect("Failed to read line");

        for (n, d) in nums.iter().zip(digits.iter()) {
            line = line.replace(n, d);
        }

        sum += num_from_line(&line);
    }

    println!("Part 2: {sum}");
}

fn p1(lines: InputLines) {
    let mut sum = 0;

    for res_line in lines {
        let line: String = res_line.expect("Failed to read line");
        sum += num_from_line(&line);
    }

    println!("Part 1: {sum}");
}

pub fn d01() {
    let input_file = Path::new("src/d01/in.txt");

    p1(read_lines(input_file).expect("Couldnt read lines of file"));
    p2(read_lines(input_file).expect("Couldnt read lines of file"));
}
