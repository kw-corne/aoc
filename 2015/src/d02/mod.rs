use core::panic;
use std::path::Path;

use crate::util::get_lines;

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let mut sum = 0;

    for line in &lines {
        let split: Vec<u32> = line.split('x').map(|x| x.parse::<u32>().unwrap()).collect();

        let l = split[0];
        let w = split[1];
        let h = split[2];

        let lw = l * w;
        let wh = w * h;
        let hl = h * l;
        let m = lw.min(wh).min(hl);

        sum += (2 * lw) + (2 * wh) + (2 * hl) + m;
    }

    println!("{}", sum);
}

pub fn d02() {
    let input_file = Path::new("src/d02/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
