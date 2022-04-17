use common::{read_input_lines, Day, InputLines, Result, Year};
use itertools::Itertools;

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
const NAUGHTY_WORDS: &[&str] = &["ab", "cd", "pq", "xy"];

fn words(input_lines: InputLines) -> Result<Vec<String>> {
    input_lines.map(|line| Ok(line?)).collect()
}

fn count_nice_words(words: &[String]) -> usize {
    words.iter().filter(|word| is_nice(word)).count()
}

fn is_nice(word: &str) -> bool {
    let has_enough_vowels = word.chars().filter(|c| VOWELS.contains(c)).count() >= 3;
    if !has_enough_vowels {
        return false;
    }

    let has_naughty_words = NAUGHTY_WORDS
        .iter()
        .any(|naughty_word| word.contains(naughty_word));
    if has_naughty_words {
        return false;
    }

    word.chars().tuple_windows::<(_, _)>().any(|(a, b)| a == b)
}

fn main() -> Result<()> {
    let input = read_input_lines(Year(2015), Day(5))?;
    let words = words(input)?;
    let nice_words = count_nice_words(&words);

    println!("Number of nice words: {nice_words}");

    Ok(())
}
