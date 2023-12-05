use std::path::Path;

use crate::util::get_lines;

fn stackify(lines: &Vec<String>) -> (Vec<Vec<char>>, usize) {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut i = 0;

    while lines[i] != "" {
        i += 1;
    }

    let n_stacks = lines[i - 1]
        .chars()
        .nth_back(1)
        .unwrap()
        .to_digit(10)
        .unwrap();

    for _ in 0..n_stacks {
        stacks.push(vec![]);
    }

    let mut r1 = (i as i32) - 2;
    let inst_line = i + 1;

    while r1 >= 0 {
        let chars: Vec<char> = lines[r1 as usize].chars().collect();

        for i in 0..n_stacks {
            let c = chars[1 + (4 * i) as usize];

            match c {
                ' ' => (),
                _ => stacks[i as usize].push(c),
            }
        }

        r1 -= 1;
    }

    (stacks, inst_line)
}

fn instr(s: &str) -> Vec<usize> {
    s.split_whitespace()
        .skip(1)
        .step_by(2)
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn p2(lines: Vec<String>) {
    let (mut stacks, s) = stackify(&lines);

    for i in s..lines.len() {
        let inst = instr(&lines[i]);

        let mut crates: Vec<char> = vec![];
        for _ in 0..inst[0] {
            if let Some(e) = stacks[inst[1] - 1].pop() {
                crates.push(e);
            }
        }

        for c in crates.iter().rev() {
            stacks[inst[2] - 1].push(*c);
        }
    }

    for stack in &stacks {
        if let Some(l) = stack.last() {
            print!("{}", l);
        }
    }

    println!();
}

fn p1(lines: Vec<String>) {
    let (mut stacks, s) = stackify(&lines);

    for i in s..lines.len() {
        let inst = instr(&lines[i]);

        for _ in 0..inst[0] {
            if let Some(e) = stacks[inst[1] - 1].pop() {
                stacks[inst[2] - 1].push(e);
            }
        }
    }

    for stack in &stacks {
        if let Some(l) = stack.last() {
            print!("{}", l);
        }
    }

    println!();
}

pub fn d05() {
    let input_file = Path::new("src/d05/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
