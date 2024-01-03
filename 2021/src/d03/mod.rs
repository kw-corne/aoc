use std::cmp::Ordering;
use std::path::Path;

use crate::util::get_lines;

fn get_mcb(bits: &Vec<String>) -> Vec<i32> {
    let n_bits = bits[0].len();
    let mut mcb = vec![0; n_bits];

    for line in bits {
        for (i, bit) in line.chars().enumerate() {
            match bit {
                '0' => mcb[i] += 1,
                '1' => mcb[i] -= 1,
                _ => panic!(),
            }
        }
    }

    mcb
}

fn rating(bits: &mut Vec<String>, keep_bit: char) -> i32 {
    fn keep_bits(bits: &mut Vec<String>, keep: char, i: usize) {
        bits.retain(|b| b.chars().nth(i).unwrap() == keep);
    }

    let keep_inv = {
        match keep_bit {
            '0' => '1',
            '1' => '0',
            _ => panic!(),
        }
    };

    for i in 0..bits[0].len() {
        match get_mcb(&bits)[i].cmp(&0) {
            Ordering::Less => {
                keep_bits(bits, keep_bit, i);
            }
            Ordering::Greater => {
                keep_bits(bits, keep_inv, i);
            }
            Ordering::Equal => {
                keep_bits(bits, keep_bit, i);
            }
        }

        if bits.len() == 1 {
            return i32::from_str_radix(&bits[0], 2).unwrap();
        }
    }

    panic!()
}

fn p2(lines: Vec<String>) {
    let ox_gen = rating(&mut lines.clone(), '1');
    let co_scrub = rating(&mut lines.clone(), '0');
    println!("{}", ox_gen * co_scrub);
}

fn p1(lines: Vec<String>) {
    let mcb = get_mcb(&lines);

    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, b) in mcb.iter().rev().enumerate() {
        let inc = {
            if *b > 0 {
                &mut gamma
            } else {
                &mut epsilon
            }
        };

        *inc += 2_i32.pow(i as u32);
    }

    println!("{}", gamma * epsilon);
}
pub fn d03() {
    let input_file = Path::new("src/d03/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
