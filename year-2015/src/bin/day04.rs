use common::anyhow::anyhow;
use common::{read_single_input_line, Day, Result, Year};

fn find_number_for_md5(input: &str, starting_with: &str) -> Option<usize> {
    (0..).find(|number| {
        let hash = md5::compute(format!("{input}{number}"));
        let hash_in_hex = format!("{hash:x}"); // probably could be done better without allocating?

        hash_in_hex.starts_with(starting_with)
    })
}

fn main() -> Result<()> {
    let input = read_single_input_line(Year(2015), Day(4))?;
    let with_five_zeroes = find_number_for_md5(&input, "00000")
        .ok_or_else(|| anyhow!("Failed to get number for part 1"))?;
    let with_six_zeroes = find_number_for_md5(&input, "000000")
        .ok_or_else(|| anyhow!("Failed to get number for part 2"))?;

    println!("The number for part 1 is: {with_five_zeroes}");
    println!("The number for part 2 is: {with_six_zeroes}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // If your secret key is abcdef, the answer is 609043,
    // because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...),
    // and it is the lowest such number to do so.
    #[test]
    fn first_example_from_task_description() {
        let input = "abcdef";
        let expected = Some(609043);

        assert_eq!(expected, find_number_for_md5(input, "00000"));
    }
}
