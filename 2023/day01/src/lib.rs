pub mod part1;
pub mod part2;

use std::fs::File;
use std::io::{self, BufReader};

pub fn read_input(file_path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(file_path)?;

    Ok(BufReader::new(file))
}
