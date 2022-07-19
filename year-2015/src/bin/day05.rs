use common::{read_input_lines, Day, InputLines, Result, Year};
use itertools::Itertools;
use std::collections::HashMap;

trait NiceWordsSolver {
    fn is_nice(word: &str) -> bool;

    fn count_nice_words(words: &[String]) -> usize {
        words.iter().filter(|word| Self::is_nice(word)).count()
    }
}

struct NiceWordsSolverV1;

impl NiceWordsSolverV1 {
    const VOWELS: &'static [char] = &['a', 'e', 'i', 'o', 'u'];
    const NAUGHTY_WORDS: &'static [&'static str] = &["ab", "cd", "pq", "xy"];
}

impl NiceWordsSolver for NiceWordsSolverV1 {
    fn is_nice(word: &str) -> bool {
        let has_enough_vowels = word.chars().filter(|c| Self::VOWELS.contains(c)).count() >= 3;
        if !has_enough_vowels {
            return false;
        }

        let has_naughty_words = Self::NAUGHTY_WORDS
            .iter()
            .any(|naughty_word| word.contains(naughty_word));
        if has_naughty_words {
            return false;
        }

        word.chars().tuple_windows().any(|(a, b)| a == b)
    }
}

struct NiceWordsSolverV2;

impl NiceWordsSolver for NiceWordsSolverV2 {
    fn is_nice(word: &str) -> bool {
        let mut pairs = HashMap::new();
        let mut last_pair = None;
        for (a, b) in word.chars().tuple_windows() {
            if last_pair == Some((a, b)) {
                continue;
            }

            *pairs.entry((a, b)).or_insert(0) += 1;

            last_pair = Some((a, b));
        }

        if !pairs.values().any(|&num| num >= 2) {
            return false;
        }

        let has_repeating_letters_with_separator = word
            .chars()
            .tuple_windows()
            .any(|(first, _, second)| first == second);

        has_repeating_letters_with_separator
    }
}

fn words(input_lines: InputLines) -> Result<Vec<String>> {
    input_lines.map(|line| Ok(line?)).collect()
}

fn main() -> Result<()> {
    let input = read_input_lines(Year(2015), Day(5))?;
    let words = words(input)?;
    let nice_words = NiceWordsSolverV1::count_nice_words(&words);
    let nice_words_v2 = NiceWordsSolverV2::count_nice_words(&words);

    println!("Number of nice words: {nice_words}");
    println!("Number of nice words (v2): {nice_words_v2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_examples() {
        assert!(NiceWordsSolverV1::is_nice("ugknbfddgicrmopn"));
        assert!(NiceWordsSolverV1::is_nice("aaa"));
        assert!(!NiceWordsSolverV1::is_nice("jchzalrnumimnmhp"));
        assert!(!NiceWordsSolverV1::is_nice("haegwjzuvuyypxyu"));
        assert!(!NiceWordsSolverV1::is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn v2_examples() {
        assert!(NiceWordsSolverV2::is_nice("qjhvhtzxzqqjkmpb"));
        assert!(NiceWordsSolverV2::is_nice("xxyxx"));
        assert!(!NiceWordsSolverV2::is_nice("uurcxstgmygtbstg"));
        assert!(!NiceWordsSolverV2::is_nice("ieodomkazucvgmuy"));
        assert!(!NiceWordsSolverV2::is_nice("aaa"));
    }
}
