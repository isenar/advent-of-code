use common::anyhow::{anyhow, bail, Error};
use common::{read_input_lines, Day, InputLines, Result, Year};
use std::ops::{Add, Sub};

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

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Brightness(usize);

impl Add for Brightness {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Brightness {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Light {
    status: LightStatus,    // for part 1
    brightness: Brightness, // for part 2
}

#[derive(Debug, Clone, Copy)]
enum LightStatus {
    On,
    Off,
}

impl Default for LightStatus {
    fn default() -> Self {
        Self::Off
    }
}

impl Light {
    pub fn toggle(&mut self) {
        self.status = match self.status {
            LightStatus::On => LightStatus::Off,
            LightStatus::Off => LightStatus::On,
        };
        self.brightness.0 += 2;
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
            lights: vec![vec![Light::default(); Self::SIZE]; Self::SIZE],
        }
    }

    #[cfg(test)]
    pub fn new_lit() -> Self {
        Self {
            lights: vec![
                vec![
                    Light {
                        status: LightStatus::On,
                        brightness: Brightness::default()
                    };
                    Self::SIZE
                ];
                Self::SIZE
            ],
        }
    }

    pub fn apply(&mut self, command: Command) -> Result<()> {
        for x in command.start.x..=command.end.x {
            for y in command.start.y..=command.end.y {
                let mut old_value = self.lights[x][y];
                let new_value = match command.action {
                    Action::Toggle => {
                        old_value.toggle();

                        old_value
                    }
                    Action::TurnOn => Light {
                        status: LightStatus::On,
                        brightness: old_value.brightness + Brightness(1),
                    },
                    Action::TurnOff => Light {
                        status: LightStatus::Off,
                        brightness: old_value.brightness - Brightness(1),
                    },
                };

                self.lights[x][y] = new_value;
            }
        }

        Ok(())
    }

    pub fn lights_on_count(&self) -> usize {
        self.lights
            .iter()
            .flatten()
            .filter(|light| matches!(light.status, LightStatus::On))
            .count()
    }

    pub fn total_brightness(&self) -> Brightness {
        self.lights
            .iter()
            .flatten()
            .fold(Brightness(0), |brightness, light| {
                brightness + light.brightness
            })
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
        grid.apply(command)?;
    }

    println!("Lights on: {}", grid.lights_on_count());
    println!("Total brightness: {:?}", grid.total_brightness().0);

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
        grid.apply(command).unwrap();

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
        grid.apply(command).unwrap();

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
        grid.apply(command).unwrap();

        assert_eq!(1_000_000 - 4, grid.lights_on_count());
    }

    #[test]
    fn increase_brightness_by_one() {
        let mut grid = Grid::new();
        let command = Command {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 0 },
            action: Action::TurnOn,
        };
        grid.apply(command).unwrap();

        assert_eq!(Brightness(1), grid.total_brightness());
    }

    #[test]
    fn toggling_all_lights_gives_2m_brigtness() {
        let mut grid = Grid::new();
        let command = Command {
            start: Point { x: 0, y: 0 },
            end: Point { x: 999, y: 999 },
            action: Action::Toggle,
        };
        grid.apply(command).unwrap();

        assert_eq!(Brightness(2_000_000), grid.total_brightness());
    }
}
