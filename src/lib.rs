use anyhow::Result;
use std::str::FromStr;

pub mod parsers;

pub fn read_input<T>(kind: &str, day: usize) -> Result<Vec<T>>
where
    T: FromStr,
{
    let path = format!("src/{kind}/{day:02}.txt");

    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|x| x.parse::<T>().ok())
        .collect())
}
