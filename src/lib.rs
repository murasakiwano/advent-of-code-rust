use anyhow::Result;
use std::str::FromStr;

pub fn read_input<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|x| x.parse::<T>().ok())
        .collect())
}
