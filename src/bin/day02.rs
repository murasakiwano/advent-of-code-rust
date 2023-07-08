use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Play {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(char) = s.chars().next() {
            let variant = match char {
                'A' => Play::Rock,
                'B' => Play::Paper,
                'C' => Play::Scissors,
                _ => panic!("invalid play!"),
            };
            Ok(variant)
        } else {
            Err(anyhow::format_err!("couldn't parse two plays"))
        }
    }
}

#[derive(Debug)]
struct RPSLine(Play, Play);

impl FromStr for RPSLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((opponent, you)) = s.split_once(' ') {
            let opponent = opponent.parse::<Play>()?;
            let you = match (&opponent, you) {
                (Play::Rock, "X") => Play::Scissors,
                (Play::Rock, "Y") => Play::Rock,
                (Play::Rock, "Z") => Play::Paper,
                (Play::Paper, "X") => Play::Rock,
                (Play::Paper, "Y") => Play::Paper,
                (Play::Paper, "Z") => Play::Scissors,
                (Play::Scissors, "X") => Play::Paper,
                (Play::Scissors, "Y") => Play::Scissors,
                (Play::Scissors, "Z") => Play::Rock,
                (_, _) => panic!("Impossible round!"),
            };

            Ok(RPSLine(opponent, you))
        } else {
            Err(anyhow::format_err!("couldn't parse two plays"))
        }
    }
}

fn score_one_round(round: &RPSLine) -> u32 {
    use Play::*;

    match round {
        RPSLine(Rock, Rock) => 4,
        RPSLine(Rock, Paper) => 8,
        RPSLine(Rock, Scissors) => 3,
        RPSLine(Paper, Rock) => 1,
        RPSLine(Paper, Paper) => 5,
        RPSLine(Paper, Scissors) => 9,
        RPSLine(Scissors, Rock) => 7,
        RPSLine(Scissors, Paper) => 2,
        RPSLine(Scissors, Scissors) => 6,
    }
}

fn main() -> Result<()> {
    let input: Vec<RPSLine> = aoc::read_input("src/inputs/02.txt")?;

    let score = input.iter().fold(0, |acc, ele| acc + score_one_round(ele));

    println!("The total score is {score}");

    Ok(())
}
