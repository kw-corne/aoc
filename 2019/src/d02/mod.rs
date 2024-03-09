use core::panic;
use std::path::Path;

use crate::util::get_lines;

fn p2(lines: Vec<String>) {
    for i in 0..99 {
        for j in 0..99 {
            let mut nums: Vec<i32> = lines[0]
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            nums[1] = i;
            nums[2] = j;

            let mut i = 0;

            while nums[i] != 99 {
                let opcode = nums[i];
                let n1 = nums[nums[i + 1] as usize];
                let n2 = nums[nums[i + 2] as usize];
                let idx = nums[i + 3] as usize;

                nums[idx] = match opcode {
                    1 => n1 + n2,
                    2 => n1 * n2,
                    _ => panic!(),
                };

                i += 4
            }

            if nums[0] == 19690720 {
                println!("{}", 100 * nums[1] + nums[2]);
                break;
            }
        }
    }
}

fn p1(lines: Vec<String>) {
    let mut nums: Vec<i32> = lines[0]
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    nums[1] = 12;
    nums[2] = 2;

    let mut i = 0;

    while nums[i] != 99 {
        let opcode = nums[i];
        let n1 = nums[nums[i + 1] as usize];
        let n2 = nums[nums[i + 2] as usize];
        let idx = nums[i + 3] as usize;

        nums[idx] = match opcode {
            1 => n1 + n2,
            2 => n1 * n2,
            _ => panic!(),
        };

        i += 4
    }

    println!("{}", nums[0]);
}

pub fn d02() {
    let input_file = Path::new("src/d02/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
