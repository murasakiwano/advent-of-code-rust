use anyhow::Result;

fn parse_text_to_range(input: &str) -> (u32, u32) {
    let split_input = input.split('-');
    let split_input: Vec<u32> = split_input
        .into_iter()
        .filter_map(|i| i.parse::<u32>().ok())
        .collect();

    let (x, y) = (split_input[0], split_input[1]);

    (x, y)
}

fn parse_input(input: &[String]) -> Vec<Vec<(u32, u32)>> {
    let parsed_input = input.iter().map(|x| x.split(',').collect::<Vec<&str>>());

    parsed_input
        .map(|r| {
            r.iter()
                .map(|rr| parse_text_to_range(rr))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_one(input: &[String]) -> usize {
    let parsed_input = parse_input(input);

    parsed_input
        .iter()
        .filter(|pair| {
            let r1 = pair[0];
            let r2 = pair[1];

            r1.0 <= r2.0 && r1.1 >= r2.1 || r2.0 <= r1.0 && r2.1 >= r1.1
        })
        .count()
}

fn part_two(input: &[String]) -> usize {
    let parsed_input = parse_input(input);

    parsed_input
        .iter()
        .filter(|pair| {
            let (a, b) = pair[0];
            let (c, d) = pair[1];

            a <= c && c <= b || a <= d && d <= b || c <= a && a <= d || c <= b && b <= d
        })
        .count()
}

fn main() -> Result<()> {
    let input: Vec<String> = aoc::read_input("inputs", 4)?;
    let part1 = part_one(&input);
    let part2 = part_two(&input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
