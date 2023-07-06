use anyhow::Result;
use itertools::Itertools;

fn part1() -> Result<u64> {
    let groups = include_str!("../inputs/01.txt")
        .lines()
        .map(|l| l.parse::<u64>().ok())
        .collect::<Vec<_>>();

    Ok(groups
        .into_iter()
        .coalesce(|x, y| match (x, y) {
            (None, None) => Err((x, y)),
            (None, Some(y)) => Ok(Some(y)),
            (Some(x), None) => Err((Some(x), None)),
            (Some(x), Some(y)) => Ok(Some(x + y)),
        })
        .flatten()
        .max()
        .unwrap_or(0))
}

fn main() -> Result<()> {
    let max_elven_food = part1()?;
    println!("Max = {max_elven_food:?}");

    Ok(())
}
