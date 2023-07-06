use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let groups = include_str!("../inputs/01.txt")
        .lines()
        .map(|l| l.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let max_elven_food = groups
        .into_iter()
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max();

    println!("Max = {max_elven_food:?}");
    Ok(())
}
