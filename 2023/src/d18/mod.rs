use std::path::Path;

use crate::util::{dump_grid, get_lines};

fn p2(lines: Vec<String>) {
    let mut points: Vec<(i64, i64)> = vec![];
    let mut curr = (0, 0);
    points.push(curr.clone());
    let mut border_len = 0;

    for line in &lines {
        let instr: Vec<&str> = line.split(' ').collect();
        let hex = instr[2];
        let n = i64::from_str_radix(&hex[2..7], 16).unwrap();
        let dir = hex.chars().nth_back(1).unwrap();

        // 0R 1D 2L 3U
        match dir {
            '0' => curr.1 += n,
            '2' => curr.1 -= n,
            '1' => curr.0 += n,
            '3' => curr.0 -= n,
            _ => panic!(),
        }
        border_len += n;
        points.push(curr.clone());
    }

    // shoelace algorithm
    let mut left_sum = 0;
    let mut right_sum = 0;
    let n_points = points.len() as i32;

    for i in 0..n_points {
        let j = (i + 1) % n_points;
        left_sum += points[i as usize].0 * points[j as usize].1;
        right_sum += points[j as usize].0 * points[i as usize].1;
    }

    let area = (left_sum - right_sum).abs() / 2;
    let i = area - border_len / 2 + 1; // picks theorem
    println!("Part 2: {}", i + border_len);
}

fn p1(lines: Vec<String>) {
    let mut points: Vec<(i32, i32)> = vec![];
    let mut curr = (0, 0);
    points.push(curr.clone());
    let mut border_len = 0;

    for line in &lines {
        let instr: Vec<&str> = line.split(' ').collect();
        let dir = instr[0].chars().nth(0).unwrap();
        let n = instr[1].parse::<i32>().unwrap();

        match dir {
            'U' => curr.0 -= n,
            'D' => curr.0 += n,
            'L' => curr.1 -= n,
            'R' => curr.1 += n,
            _ => panic!(),
        }
        border_len += n;
        points.push(curr.clone());
    }

    // shoelace algorithm
    let mut left_sum = 0;
    let mut right_sum = 0;
    let n_points = points.len() as i32;

    for i in 0..n_points {
        let j = (i + 1) % n_points;
        left_sum += points[i as usize].0 * points[j as usize].1;
        right_sum += points[j as usize].0 * points[i as usize].1;
    }

    let area = (left_sum - right_sum).abs() / 2;
    let i = area - border_len / 2 + 1; // picks theorem
    println!("Part 1: {}", i + border_len);
}

pub fn d18() {
    let input_file = Path::new("src/d18/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
