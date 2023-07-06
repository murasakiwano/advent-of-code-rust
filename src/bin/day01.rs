use anyhow::{Context, Result};
use itertools::Itertools;

type ElvenCalories = Vec<Option<u64>>;

fn parse_input(filepath: &str) -> Result<ElvenCalories> {
    let input_string =
        std::fs::read_to_string(filepath).with_context(|| "Trying to read {filepath}")?;
    Ok(input_string
        .lines()
        .map(|l| l.parse::<u64>().ok())
        .collect::<ElvenCalories>())
}

fn form_groups(filepath: &str) -> Result<Vec<u64>> {
    let groups: ElvenCalories = parse_input(filepath)?;

    Ok(groups
        .into_iter()
        .coalesce(|x, y| match (x, y) {
            (None, None) => Err((x, y)),
            (None, Some(y)) => Ok(Some(y)),
            (Some(x), None) => Err((Some(x), None)),
            (Some(x), Some(y)) => Ok(Some(x + y)),
        })
        .flatten()
        .collect::<Vec<u64>>())
}

fn part1() -> Result<u64> {
    let groups = form_groups("src/inputs/01.txt")?;

    Ok(groups.into_iter().max().unwrap_or(0))
}

fn part2() -> Result<u64> {
    let groups = form_groups("src/inputs/01.txt")?;

    Ok(groups
        .into_iter()
        .sorted_by_key(|&k| k)
        .rev()
        .take(3)
        .sum::<u64>())
}

fn main() -> Result<()> {
    let max_elven_food = part1()?;
    println!("Max = {max_elven_food:?}");

    let top3_elves = part2()?;
    println!("Top 3 elves got {top3_elves:?} calories total");

    Ok(())
}
