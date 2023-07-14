use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
enum ProcessingError {
    #[error("the input has odd length")]
    SplittingError,
    #[error("the length of the input is not divisible by 3")]
    GroupingError,
}

fn includes_char(candidate: &char, test_str: &str) -> bool {
    test_str.chars().any(|c| c == *candidate)
}

fn process_string<F>(input: &str, processor: F) -> Result<Vec<(char, u32)>>
where
    F: Fn(&char, &str) -> bool,
{
    let split_input = input.split_at(input.len() / 2);
    if split_input.0.len() != split_input.1.len() {
        return Err(ProcessingError::SplittingError.into());
    }

    let zipped_lower = ('a'..='z').zip_eq(1..=26);
    let zipped_upper = ('A'..='Z').zip_eq(27..=52);
    let zipped_types = zipped_lower.merge(zipped_upper);

    let repeated_types = zipped_types
        .filter(|(c, _p)| processor(c, split_input.0) && processor(c, split_input.1))
        .collect();

    Ok(repeated_types)
}

fn part_one(input: &Vec<String>) -> Result<u32> {
    let mut repetitions = Vec::new();

    for s in input {
        repetitions.push(process_string(s, includes_char)?[0]); // supposed to only be one
    }

    Ok(repetitions.iter().fold(0, |acc, el| acc + el.1))
}

fn part_two(input: &Vec<String>) -> Result<u32> {
    if input.len() % 3 != 0 {
        return Err(ProcessingError::GroupingError.into());
    }

    let zipped_lower = ('a'..='z').zip_eq(1..=26);
    let zipped_upper = ('A'..='Z').zip_eq(27..=52);
    let zipped_types = zipped_lower.merge(zipped_upper);
    let mut repetitions: Vec<u32> = vec![];

    for i in (0..input.len()).step_by(3) {
        let t1 = &input[i];
        let t2 = &input[i + 1];
        let t3 = &input[i + 2];

        let mut priorities = zipped_types
            .clone()
            .filter_map(|(c, p)| {
                if includes_char(&c, t1) && includes_char(&c, t2) && includes_char(&c, t3) {
                    return Some(p);
                }
                None
            })
            .collect::<Vec<u32>>();

        repetitions.append(&mut priorities);
    }

    Ok(repetitions.iter().sum())
}

fn main() -> Result<()> {
    let input: Vec<String> = aoc::read_input("inputs", 3)?;
    let part1 = part_one(&input)?;
    let part2 = part_two(&input)?;

    println!("part one: {part1:?}");
    println!("part two: {part2:?}");

    Ok(())
}
