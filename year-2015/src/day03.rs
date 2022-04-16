use common::{read_single_input_line, Day, Result, Year};

use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn move_pos(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
        }
    }
}

fn houses_visited(input: &str) -> usize {
    let mut house_positions = HashSet::new();
    let mut santas_position = Position::default();

    house_positions.insert(santas_position);

    for symbol in input.chars() {
        let direction = Direction::from(symbol);
        santas_position.move_pos(direction);

        house_positions.insert(santas_position);
    }

    house_positions.len()
}

fn houses_visited_with_robo_santa(input: &str) -> usize {
    let mut house_positions = HashSet::new();
    let mut santas_position = Position::default();
    let mut robos_position = Position::default();

    house_positions.insert(santas_position);

    for (n, symbol) in input.chars().enumerate() {
        let direction = Direction::from(symbol);

        let new_position = if n % 2 == 0 {
            santas_position.move_pos(direction);
            santas_position
        } else {
            robos_position.move_pos(direction);
            robos_position
        };

        house_positions.insert(new_position);
    }

    house_positions.len()
}

fn main() -> Result<()> {
    let input = read_single_input_line(Year(2015), Day(3))?;
    let houses_visited = houses_visited(&input);
    let houses_visited_with_robo = houses_visited_with_robo_santa(&input);

    println!("Houses visited: {houses_visited}");
    println!("Houses visited with Robo-Santa: {houses_visited_with_robo}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // The tests are taken from the examples in the description
    // Part 1

    // > delivers presents to 2 houses: one at the starting location, and one to the east.
    #[test]
    fn simple() {
        let input = ">";
        let houses_visited = houses_visited(input);

        assert_eq!(2, houses_visited);
    }

    // ^>v< delivers presents to 4 houses in a square,
    // including twice to the house at his starting/ending location.
    #[test]
    fn houses_in_a_square() {
        let input = "^>v<";
        let houses_visited = houses_visited(input);

        assert_eq!(4, houses_visited);
    }

    // ^v^v^v^v^v delivers a bunch of presents to
    // some very lucky children at only 2 houses.
    #[test]
    fn up_and_down() {
        let input = "^v^v^v^v^v";
        let houses_visited = houses_visited(input);

        assert_eq!(2, houses_visited);
    }

    // Part 2

    // ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
    #[test]
    fn robo_santa() {
        let input = "^v";
        let houses_visited = houses_visited_with_robo_santa(input);

        assert_eq!(3, houses_visited);
    }

    // ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa
    // end up back where they started.
    #[test]
    fn both_back_and_forth() {
        let input = "^>v<";
        let houses_visited = houses_visited_with_robo_santa(input);

        assert_eq!(3, houses_visited);
    }

    // ^v^v^v^v^v now delivers presents to 11 houses, with Santa going
    // one direction and Robo-Santa going the other
    #[test]
    fn both_going_away() {
        let input = "^v^v^v^v^v";
        let houses_visited = houses_visited_with_robo_santa(input);

        assert_eq!(11, houses_visited);
    }
}
