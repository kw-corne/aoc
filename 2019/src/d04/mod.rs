use core::panic;
use std::path::Path;

use crate::util::get_lines;

fn check_pw(mut pw: i32) -> bool {
    let mut double_found = false;
    let mut prev_digit: Option<i32> = None;
    let mut i = 0;

    while pw > 0 {
        let digit = pw % 10;
        pw /= 10;

        if let Some(p_digit) = prev_digit {
            if p_digit < digit {
                return false;
            }

            if p_digit == digit {
                double_found = true;
            }
        }

        prev_digit = Some(digit);
        i += 1;
    }

    double_found
}

fn char_at(s: &str, i: usize) -> i32 {
    s.chars().nth(i).unwrap().to_digit(10).unwrap() as i32
}

fn check_pw2(mut pw: i32) -> bool {
    let spw = pw.to_string();

    for i in 0..spw.len() {
        let curr = char_at(&spw, i);
        let next = char_at(&spw, i + 1);
    }

    false
}

fn p2(lines: Vec<String>) {
    let nums = lines[0].split_once('-').unwrap();
    let lb = nums.0.parse::<i32>().unwrap();
    let ub = nums.1.parse::<i32>().unwrap();

    let mut total = 0;

    for i in lb..=ub {
        if check_pw2(i) {
            total += 1;
        }
    }

    println!("{}", total);
    println!("{}", ub - lb);
}

fn p1(lines: Vec<String>) {
    let nums = lines[0].split_once('-').unwrap();
    let lb = nums.0.parse::<i32>().unwrap();
    let ub = nums.1.parse::<i32>().unwrap();

    let mut total = 0;

    for i in lb..=ub {
        if check_pw(i) {
            total += 1;
        }
    }

    println!("{}", total);
    println!("{}", ub - lb);
}

pub fn d04() {
    let input_file = Path::new("src/d04/in.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
