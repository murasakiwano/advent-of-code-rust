use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
enum ProcessingError {
    #[error("the input has odd length")]
    SplittingError,
}

fn is_in_both(candidate: &char, first_half: &str, second_half: &str) -> bool {
    first_half.chars().any(|c| c == *candidate) && second_half.chars().any(|c| c == *candidate)
}

fn process_string(input: &str) -> Result<Vec<(char, u32)>> {
    let split_input = input.split_at(input.len() / 2);
    if split_input.0.len() != split_input.1.len() {
        return Err(ProcessingError::SplittingError.into());
    }

    let zipped_lower = ('a'..='z').zip_eq(1..=26);
    let zipped_upper = ('A'..='Z').zip_eq(27..=52);
    let zipped_types = zipped_lower.merge(zipped_upper);

    let repeated_types = zipped_types
        .filter(|(c, _p)| is_in_both(c, split_input.0, split_input.1))
        .collect();

    Ok(repeated_types)
}

fn main() -> Result<()> {
    let input: Vec<String> = aoc::read_input("inputs", 3)?;
    let mut repetitions = Vec::new();

    for s in input {
        repetitions.push(process_string(&s)?[0]); // supposed to only be one
    }

    println!("{:?}", repetitions.iter().fold(0, |acc, el| acc + el.1));

    Ok(())
}
