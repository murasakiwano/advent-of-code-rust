use anyhow::Result;
use itertools::Itertools;

type ElvenCalories = Vec<Option<u64>>;

fn parse_input(filepath: &str) -> Result<ElvenCalories> {
    let input_string = std::fs::read_to_string(filepath)?;
    Ok(input_string
        .lines()
        .map(|l| l.parse::<u64>().ok())
        .collect::<ElvenCalories>())
}

fn get_max_group(groups: Vec<Option<u64>>) -> u64 {
    groups
        .into_iter()
        .coalesce(|x, y| match (x, y) {
            (None, None) => Err((x, y)),
            (None, Some(y)) => Ok(Some(y)),
            (Some(x), None) => Err((Some(x), None)),
            (Some(x), Some(y)) => Ok(Some(x + y)),
        })
        .flatten()
        .max()
        .unwrap_or(0)
}

fn part1() -> Result<u64> {
    let groups = parse_input("../inputs/01.txt")?;

    Ok(get_max_group(groups))
}

fn part2() -> Result<u64> {
    Ok(0)
}

fn main() -> Result<()> {
    let max_elven_food = part1()?;
    println!("Max = {max_elven_food:?}");

    Ok(())
}
