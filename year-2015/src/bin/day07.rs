use common::anyhow::{anyhow, bail, Error};
use common::{read_input_lines, Day, Result, Year};
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

const BASE_10: u32 = 10;

type Signal = u16;

#[derive(Debug)]
enum Gate {
    Value(Signal),
    Connected(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LeftShift(String, Signal),
    RightShift(String, Signal),
}

impl From<Wire> for Gate {
    fn from(wire: Wire) -> Self {
        match wire {
            Wire::Value(value) => Self::Value(value),
            Wire::Connected(wire) => Self::Connected(wire),
        }
    }
}

#[derive(Debug)]
struct Circuit {
    gates: HashMap<String, Gate>,
}

impl Circuit {
    pub fn parse(input: impl IntoIterator<Item = String>) -> Result<Self> {
        let mut gates = HashMap::new();
        for line in input {
            let split = line.split(' ').collect_vec();

            match split.as_slice() {
                [signal, "->", wire] => {
                    let signal = Wire::from_str(signal)?;

                    gates
                        .entry(wire.to_string())
                        .or_insert_with(|| Gate::from(signal));
                }
                ["NOT", wire, "->", output] => {
                    gates
                        .entry(output.to_string())
                        .or_insert_with(|| Gate::Not(wire.to_string()));
                }
                [wire1, "OR", wire2, "->", output] => {
                    gates
                        .entry(output.to_string())
                        .or_insert_with(|| Gate::Or(wire1.to_string(), wire2.to_string()));
                }
                [wire1, "AND", wire2, "->", output] => {
                    gates
                        .entry(output.to_string())
                        .or_insert_with(|| Gate::And(wire1.to_string(), wire2.to_string()));
                }
                [wire, "RSHIFT", value, "->", output] => {
                    let value = Signal::from_str(value)?;

                    gates
                        .entry(output.to_string())
                        .or_insert_with(|| Gate::RightShift(wire.to_string(), value));
                }
                [wire, "LSHIFT", value, "->", output] => {
                    let value = Signal::from_str(value)?;

                    gates
                        .entry(output.to_string())
                        .or_insert_with(|| Gate::LeftShift(wire.to_string(), value));
                }

                _ => {
                    bail!("Unrecognized pattern: {line}");
                }
            }
        }

        Ok(Self { gates })
    }

    pub fn signal_value(&self, wire_name: &str) -> Option<Signal> {
        let mut cache = HashMap::default();

        self.signal_value_cached(wire_name, &mut cache)
    }

    fn signal_value_cached(
        &self,
        wire_name: &str,
        cache: &mut HashMap<String, Signal>,
    ) -> Option<Signal> {
        let gate = self.gates.get(wire_name)?;

        let result = match gate {
            Gate::Value(value) => *value,
            Gate::Connected(connected) => {
                let result = self.try_fetch_from_cache(connected, cache)?;
                cache.insert(wire_name.to_string(), result);

                result
            }
            Gate::Not(wire) => {
                let result = self.try_fetch_from_cache(wire, cache)?;

                !result
            }
            Gate::And(wire1, wire2) => {
                let wire1 = Wire::from_str(wire1).ok().map(|wire| match wire {
                    Wire::Value(value) => Some(value),
                    Wire::Connected(connected) => self.try_fetch_from_cache(&connected, cache),
                })??;
                let wire2 = self.try_fetch_from_cache(wire2, cache)?;

                wire1 & wire2
            }
            Gate::Or(wire1, wire2) => {
                let wire1_val = self.try_fetch_from_cache(wire1, cache)?;
                cache.insert(wire1.to_string(), wire1_val);

                let wire2_val = self.try_fetch_from_cache(wire2, cache)?;
                cache.insert(wire2.to_string(), wire2_val);

                wire1_val | wire2_val
            }
            Gate::LeftShift(wire, value) => {
                let signal = self.try_fetch_from_cache(wire, cache)?;

                signal << value
            }
            Gate::RightShift(wire, value) => {
                let signal = self.try_fetch_from_cache(wire, cache)?;

                signal >> value
            }
        };

        cache.insert(wire_name.to_string(), result);

        Some(result)
    }

    fn try_fetch_from_cache(
        &self,
        wire_name: &str,
        cache: &mut HashMap<String, Signal>,
    ) -> Option<Signal> {
        cache
            .get(wire_name)
            .cloned()
            .or_else(|| self.signal_value_cached(wire_name, cache))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Wire {
    Value(Signal),
    Connected(String),
}

impl FromStr for Wire {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_numeric = s.chars().all(|c| c.is_digit(BASE_10));

        let wire = if is_numeric {
            Self::Value(s.parse()?)
        } else {
            Self::Connected(s.to_string())
        };

        Ok(wire)
    }
}

fn main() -> Result<()> {
    let input = read_input_lines(Year(2015), Day(7))?;
    let input: Vec<_> = input.into_iter().try_collect()?;
    let circuit = Circuit::parse(input)?;
    let a = circuit
        .signal_value("a")
        .ok_or_else(|| anyhow!("Failed to fetch value of a"))?;

    println!("Value of signal on wire 'a' is {a}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_from_task_description() {
        let input = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ]
        .into_iter()
        .map(ToOwned::to_owned);

        let circuit = Circuit::parse(input).unwrap();

        assert_eq!(Some(72), circuit.signal_value("d"));
        assert_eq!(Some(507), circuit.signal_value("e"));
        assert_eq!(Some(492), circuit.signal_value("f"));
        assert_eq!(Some(114), circuit.signal_value("g"));
        assert_eq!(Some(65412), circuit.signal_value("h"));
        assert_eq!(Some(65079), circuit.signal_value("i"));
        assert_eq!(Some(123), circuit.signal_value("x"));
        assert_eq!(Some(456), circuit.signal_value("y"));
    }
}
