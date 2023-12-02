use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn read_lines(p: &Path) -> Result<Lines<BufReader<File>>> {
    let file = File::open(p)?;
    Ok(BufReader::new(file).lines())
}
