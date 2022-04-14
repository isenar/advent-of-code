pub use anyhow;
pub use anyhow::Result;

use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Lines};

pub type InputLines = Lines<BufReader<File>>;

pub struct Day(pub u8);

pub struct Year(pub u32);

pub fn read_input_lines(year: Year, day: Day) -> Result<InputLines> {
    let path = input_path(year, day);
    let file = File::open(path)?;

    Ok(BufReader::new(file).lines())
}

pub fn read_single_input_line(year: Year, day: Day) -> Result<String> {
    let path = input_path(year, day);
    let input = read_to_string(path)?;

    Ok(input)
}

fn input_path(year: Year, day: Day) -> String {
    format!("input/{}/day{:02}", year.0, day.0)
}
