use std::path::Path;

use crate::util::get_lines;

fn get_blocks(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut blocks: Vec<Vec<String>> = vec![vec![]];

    for line in lines {
        match line.as_str() {
            "" => blocks.push(vec![]),
            _ => blocks.last_mut().unwrap().push(line.clone()),
        }
    }

    blocks
}

fn transpose(block: &Vec<String>) -> Vec<String> {
    let mut transposed: Vec<String> = vec!["".to_string(); block[0].len()];

    for i in 0..block.len() {
        for (k, c) in block[i].chars().enumerate() {
            transposed[k].push(c);
        }
    }

    transposed
}

fn count_diff(a: &Vec<String>, b: &Vec<String>) -> i32 {
    let mut d = 0;

    // assuming equal lengths for rows and strings
    for (s1, s2) in a.iter().zip(b.iter()) {
        for (ac, bc) in s1.chars().zip(s2.chars()) {
            if ac != bc {
                d += 1;
            }
        }
    }

    d
}

fn mirror_row(block: &Vec<String>, diff: i32) -> i32 {
    for i in 1..block.len() {
        let mut a: Vec<String> = block[..i].to_vec();
        a.reverse();

        let mut b: Vec<String> = block[i..].to_vec();

        if a.len() > b.len() {
            a = a[..b.len()].to_vec();
        }

        if b.len() > a.len() {
            b = b[..a.len()].to_vec();
        }

        if count_diff(&a, &b) == diff {
            return i as i32;
        }
    }

    0
}

fn p2(lines: Vec<String>) {
    let blocks = get_blocks(&lines);
    let mut sum = 0;

    for mut block in blocks {
        sum += mirror_row(&block, 1) * 100;
        block = transpose(&block);
        sum += mirror_row(&block, 1);
    }

    println!("Part 2: {}", sum);
}

fn p1(lines: Vec<String>) {
    let blocks = get_blocks(&lines);
    let mut sum = 0;

    for mut block in blocks {
        sum += mirror_row(&block, 0) * 100;
        block = transpose(&block);
        sum += mirror_row(&block, 0);
    }

    println!("Part 1: {}", sum);
}

pub fn d13() {
    let input_file = Path::new("src/d13/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
