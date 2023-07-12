use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
enum ProcessingError {
    #[error("the input has odd length")]
    SplittingError,
}

fn includes_char(candidate: &char, test_str: &str) -> bool {
    test_str.chars().any(|c| c == *candidate)
}

fn process_string<F>(input: &str, processor: F) -> Result<Vec<(char, u32)>>
where
    F: Fn(&char, &str, &str) -> bool,
{
    let split_input = input.split_at(input.len() / 2);
    if split_input.0.len() != split_input.1.len() {
        return Err(ProcessingError::SplittingError.into());
    }

    let zipped_lower = ('a'..='z').zip_eq(1..=26);
    let zipped_upper = ('A'..='Z').zip_eq(27..=52);
    let zipped_types = zipped_lower.merge(zipped_upper);

    let repeated_types = zipped_types
        .filter(|(c, _p)| processor(c, split_input.0, split_input.1))
        .collect();

    Ok(repeated_types)
}

fn part_one(input: &Vec<String>) -> Result<u32> {
    let mut repetitions = Vec::new();

    let processor = |candidate: &char, first_half: &str, second_half: &str| -> bool {
        includes_char(candidate, first_half) && includes_char(candidate, second_half)
    };

    for s in input {
        repetitions.push(process_string(s, processor)?[0]); // supposed to only be one
    }

    Ok(repetitions.iter().fold(0, |acc, el| acc + el.1))
}

fn main() -> Result<()> {
    let input: Vec<String> = aoc::read_input("inputs", 3)?;
    let part1 = part_one(&input)?;

    println!("part one: {part1:?}");

    Ok(())
}
