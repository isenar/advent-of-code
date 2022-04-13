pub use anyhow;
pub use anyhow::Result;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub type InputLines = Lines<BufReader<File>>;

pub struct Day(pub u8);

pub struct Year(pub u32);

pub fn read_input_lines(year: Year, day: Day) -> Result<InputLines> {
    let path = format!("input/{}/day{:02}", year.0, day.0);
    let file = File::open(path)?;

    Ok(BufReader::new(file).lines())
}
