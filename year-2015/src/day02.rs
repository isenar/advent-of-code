use common::{anyhow, anyhow::Result, read_input_lines, Day, InputLines, Year};
use std::str::FromStr;

struct Gift {
    length: u64,
    width: u64,
    height: u64,
}

impl Gift {
    pub fn paper_needed(&self) -> u64 {
        let first_side = self.length * self.width;
        let second_side = self.width * self.height;
        let third_side = self.length * self.height;
        let extra = first_side.min(second_side).min(third_side);

        2 * (first_side + second_side + third_side) + extra
    }

    pub fn ribbon_needed(&self) -> u64 {
        let mut sides = [self.length, self.width, self.height];
        sides.sort_unstable();
        let sides_ribbon = 2 * (sides[0] + sides[1]);

        sides_ribbon + self.length * self.width * self.height
    }
}

impl FromStr for Gift {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = string.split('x').collect();
        match split.as_slice() {
            [length, width, height] => {
                let length = length.parse()?;
                let width = width.parse()?;
                let height = height.parse()?;

                Ok(Self {
                    length,
                    width,
                    height,
                })
            }
            _ => anyhow::bail!("Expected 'length x width x height' format, got {split:?}"),
        }
    }
}

fn gifts(input: InputLines) -> Result<Vec<Gift>> {
    input
        .map(|string| {
            let string = string?;
            Gift::from_str(&string)
        })
        .collect()
}

fn paper_needed(gifts: &[Gift]) -> u64 {
    gifts
        .iter()
        .fold(0, |paper_needed, gift| paper_needed + gift.paper_needed())
}

fn ribbon_needed(gifts: &[Gift]) -> u64 {
    gifts.iter().fold(0, |ribbon_needed, gift| {
        ribbon_needed + gift.ribbon_needed()
    })
}

fn main() -> Result<()> {
    let input = read_input_lines(Year(2015), Day(2))?;
    let gifts = gifts(input)?;
    let paper_needed = paper_needed(&gifts);
    let ribbon_needed = ribbon_needed(&gifts);

    println!("Paper needed: {paper_needed}");
    println!("Ribbon needed: {ribbon_needed}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_paper_and_ribbon_needed() {
        let gift = Gift {
            length: 1,
            width: 2,
            height: 3,
        };

        assert_eq!(24, gift.paper_needed());
        assert_eq!(12, gift.ribbon_needed());
    }
}
