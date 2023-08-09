use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1},
    combinator::map_res,
    error,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

/// Parses input in the form "move X from Y to Z" into a vector of usizes [X, Y, Z]
pub fn parse_move(input: &str) -> IResult<&str, Vec<usize>> {
    let mut first_parser = preceded(
        tag("move "),
        map_res(digit1::<&str, error::Error<&str>>, str::parse::<usize>),
    );
    let mut second_parser = preceded(
        tag(" from "),
        map_res(digit1::<&str, error::Error<&str>>, str::parse::<usize>),
    );
    let mut third_parser = preceded(
        tag(" to "),
        map_res(digit1::<&str, error::Error<&str>>, str::parse::<usize>),
    );
    let (input, quantity) = first_parser(input)?;
    let (input, from) = second_parser(input)?;
    let (input, to) = third_parser(input)?;
    Ok((input, vec![quantity, from - 1, to - 1])) // - 1 because we're using indexes
}

/// Parses a list of lines of crates. Each vector element is a line
pub fn parse_stacks(input: &str) -> IResult<&str, Vec<Vec<Option<&str>>>> {
    let (input, result) = separated_list1(complete::newline, parse_line)(input)?;
    Ok((input, result))
}

/// Parses a line of three crates
fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;

    Ok((input, result))
}

/// Parses "[A]" into Some("A"), "   " into None
fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), complete::alpha1, complete::char(']')),
    ))(input)?;

    let res = match c {
        "   " => None,
        crt => Some(crt),
    };

    Ok((input, res))
}
