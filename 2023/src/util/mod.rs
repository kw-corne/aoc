use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub type InputLines = Lines<BufReader<File>>;

pub fn read_lines(p: &Path) -> Result<InputLines> {
    let file = File::open(p)?;
    Ok(BufReader::new(file).lines())
}
