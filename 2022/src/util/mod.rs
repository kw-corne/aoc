use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_lines(p: &Path) -> Vec<String> {
    let file = File::open(p).unwrap();
    BufReader::new(file).lines().into_iter().flatten().collect()
}
