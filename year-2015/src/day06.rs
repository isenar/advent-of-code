use common::anyhow::{anyhow, bail, Error};
use common::{read_input_lines, Day, InputLines, Result, Year};

use std::str::FromStr;

#[derive(Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split
            .next()
            .ok_or_else(|| anyhow!("Failed to get x coordinate from '{s}'"))?
            .parse()?;
        let y = split
            .next()
            .ok_or_else(|| anyhow!("Failed to get y coordinate from '{s}'"))?
            .parse()?;

        if let Some(unexpected) = split.next() {
            bail!("Unexpected additional parameter: {unexpected}");
        }

        Ok(Self { x, y })
    }
}

#[derive(Debug)]
struct Command {
    start: Point,
    end: Point,
    action: Action,
}

impl Command {
    pub fn parse(string: String) -> Result<Self> {
        let split: Vec<_> = string.split(' ').collect();

        let (action, start, end) = match *split.as_slice() {
            ["turn", "on", point_x, "through", point_y] => (Action::TurnOn, point_x, point_y),
            ["turn", "off", point_x, "through", point_y] => (Action::TurnOff, point_x, point_y),
            ["toggle", point_x, "through", point_y] => (Action::Toggle, point_x, point_y),
            _ => bail!("Unknown pattern: {split:?})"),
        };

        let start = Point::from_str(start)?;
        let end = Point::from_str(end)?;

        Ok(Self { start, end, action })
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Light {
    On,
    Off,
}

impl Light {
    pub fn toggle(&mut self) {
        *self = match self {
            Light::On => Light::Off,
            Light::Off => Light::On,
        };
    }
}

#[derive(Debug)]
struct Grid {
    lights: Vec<Vec<Light>>,
}

impl Grid {
    const SIZE: usize = 1000;

    pub fn new() -> Self {
        Self {
            lights: vec![vec![Light::Off; Self::SIZE]; Self::SIZE],
        }
    }

    #[cfg(test)]
    pub fn new_lit() -> Self {
        Self {
            lights: vec![vec![Light::On; Self::SIZE]; Self::SIZE],
        }
    }

    pub fn modify(&mut self, command: Command) -> Result<()> {
        for x in command.start.x..=command.end.x {
            for y in command.start.y..=command.end.y {
                let new_value = match command.action {
                    Action::Toggle => {
                        let mut old = self.lights[x][y];
                        old.toggle();

                        old
                    }
                    Action::TurnOn => Light::On,
                    Action::TurnOff => Light::Off,
                };

                let old_value = self
                    .lights
                    .get_mut(x)
                    .ok_or_else(|| anyhow!("Failed to get x coord (={x})"))?
                    .get_mut(y)
                    .ok_or_else(|| anyhow!("Failed to get y coord (={y})"))?;

                *old_value = new_value;
            }
        }

        Ok(())
    }

    pub fn lights_on_count(&self) -> usize {
        self.lights
            .iter()
            .flatten()
            .filter(|light| matches!(light, Light::On))
            .count()
    }
}

fn parse_commands(input_lines: InputLines) -> Result<Vec<Command>> {
    input_lines.map(|line| Command::parse(line?)).collect()
}

fn main() -> Result<()> {
    let input = read_input_lines(Year(2015), Day(6))?;
    let commands = parse_commands(input)?;

    let mut grid = Grid::new();

    for command in commands {
        grid.modify(command)?;
    }

    println!("Lights on: {}", grid.lights_on_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_on_all() {
        let mut grid = Grid::new();
        let command = Command {
            start: Point { x: 0, y: 0 },
            end: Point { x: 999, y: 999 },
            action: Action::TurnOn,
        };
        grid.modify(command).unwrap();

        assert_eq!(1_000_000, grid.lights_on_count());
    }

    #[test]
    fn toggle_first_line() {
        let mut grid = Grid::new();
        let command = Command {
            start: Point { x: 0, y: 0 },
            end: Point { x: 999, y: 0 },
            action: Action::Toggle,
        };
        grid.modify(command).unwrap();

        assert_eq!(1000, grid.lights_on_count());
    }

    #[test]
    fn middle_four_lights_turned_off() {
        let mut grid = Grid::new_lit();
        let command = Command {
            start: Point { x: 499, y: 499 },
            end: Point { x: 500, y: 500 },
            action: Action::TurnOff,
        };
        grid.modify(command).unwrap();

        assert_eq!(1_000_000 - 4, grid.lights_on_count());
    }
}
