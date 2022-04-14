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
    // let mut house_positions = HashSet::new();
    // let mut santas_position = Position::default();
    //
    // house_positions.insert(santas_position);
    //
    // for symbol in input.chars() {
    //     let direction = Direction::from(symbol);
    //     santas_position.move_pos(direction);
    //
    //     house_positions.insert(santas_position);
    // }
    //
    // house_positions.len()

    let houses_visited: HashSet<_> = input
        .chars()
        .into_iter()
        .scan(Position::default(), |current_position, symbol| {
            let direction = Direction::from(symbol);
            current_position.move_pos(direction);

            Some(current_position.clone())
        })
        .chain(std::iter::once(Position::default())) // include the initial position as well
        .collect();

    houses_visited.len()
}

fn houses_visited_with_robo_santa(input: &str) -> usize {
    3
}

fn main() -> Result<()> {
    let input = read_single_input_line(Year(2015), Day(3))?;
    let houses_visited = houses_visited(&input);

    println!("Houses visited: {houses_visited}");

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
}
