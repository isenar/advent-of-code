use common::{read_single_input_line, Day, Result, Year};

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
    let input = read_single_input_line(Year(2015), Day(1))?;
    let floor = calculate_floor(&input);
    let basement_pos = calculate_basement_position(&input);

    println!("Floor: {floor}");
    println!("Basement: {basement_pos}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_floor() {
        let input = ")))(()((()(()(";
        let floor = calculate_floor(input);
        let expected_floor = 2;

        assert_eq!(expected_floor, floor);
    }

    #[test]
    fn calculates_basement_position() {
        let input = "(())()())(()()())))(()()(";
        let basement_position = calculate_basement_position(input);
        let expected_position = 9;

        assert_eq!(expected_position, basement_position);
    }
}
