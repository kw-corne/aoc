use std::path::Path;

use crate::util::get_lines;

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {}

pub fn d01() {
    let input_file = Path::new("src/d01/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
