use std::path::Path;

use crate::util::get_lines;

fn p2(lines: Vec<String>) {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let instr = line.split_once(' ').unwrap();
        let dir = instr.0.chars().nth(0).unwrap();
        let n = instr.1.parse::<i32>().unwrap();

        match dir {
            'f' => {
                horiz += n;
                depth += aim * n;
            }
            'd' => aim += n,
            'u' => aim -= n,
            _ => panic!(),
        }
    }

    println!("{}", depth * horiz);
}

fn p1(lines: Vec<String>) {
    let mut horiz = 0;
    let mut depth = 0;

    for line in lines {
        let instr = line.split_once(' ').unwrap();
        let dir = instr.0.chars().nth(0).unwrap();
        let n = instr.1.parse::<i32>().unwrap();

        match dir {
            'f' => horiz += n,
            'd' => depth += n,
            'u' => depth -= n,
            _ => panic!(),
        }
    }

    println!("{}", depth * horiz);
}

pub fn d02() {
    let input_file = Path::new("src/d02/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
