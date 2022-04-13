use common::anyhow::anyhow;
use common::{read_input_lines, Day, Result, Year};

fn delta(floor_symbol: char) -> i64 {
    match floor_symbol {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn calculate_floor(input: &str) -> i64 {
    input.chars().fold(0, |acc, floor| acc + delta(floor))
}

fn calculate_basement_position(input: &str) -> usize {
    let mut current_floor = 0;
    for (position, floor) in input.chars().enumerate() {
        if current_floor == -1 {
            return position;
        }

        current_floor += delta(floor);
    }

    0
}

fn main() -> Result<()> {
    let input = read_input_lines(Year(2015), Day(1))?
        .next()
        .ok_or_else(|| anyhow!("Failed to parse input"))??;
    let floor = calculate_floor(&input);
    let basement_pos = calculate_basement_position(&input);

    println!("Floor: {floor}");
    println!("Basement: {basement_pos}");

    Ok(())
}
