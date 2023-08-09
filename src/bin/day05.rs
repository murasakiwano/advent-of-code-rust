use anyhow::Result;
use aoc::parsers::*;

type WrappedLine = Vec<Option<&'static str>>;
type Line = Vec<&'static str>;
type Stacks = Vec<Line>;

fn main() {
    // let example = include_str!("../examples/05.txt");
    // println!("{:?}", arrange_stacks(example));

    let input = include_str!("../inputs/05.txt");
    let result = arrange_stacks(input);
    println!("{:?}", result);
}

/// Does the logic of arranging the stacks
fn arrange_stacks(input: &'static str) -> Result<String> {
    let input = input.splitn(2, "\n\n").collect::<Vec<&str>>();
    let moves = input[1].split('\n').collect::<Vec<&str>>();
    let stacks = input[0];
    let (_, stacks) = parse_stacks(stacks)?;
    let stacks = revert_and_transpose(&stacks);
    let mut processed_stacks: Vec<Vec<&str>> = vec![];

    for stack in &stacks {
        let stack: Vec<_> = stack.iter().filter_map(|e| *e).collect();
        processed_stacks.push(stack);
    }

    for motion in &moves {
        let motion = parse_move(motion).unwrap().1;
        move_crates(&mut processed_stacks, &motion);
    }

    let top_crates = processed_stacks.iter().map(|e| e[e.len() - 1]);
    let result = top_crates.fold("".to_string(), |acc, ite| format!("{}{}", acc, ite));

    Ok(result)
}

fn move_crates(stacks: &mut Stacks, motion: &[usize]) {
    let quantity = motion[0];
    let takeoff_stack = motion[1];
    let landing_stack = motion[2];

    dbg!(&stacks);
    let mut i = 0;
    let mut moving_stack = vec![];
    while i < quantity {
        let Some(ele) = stacks[takeoff_stack].pop() else { todo!() };
        moving_stack.push(ele);

        i += 1;
    }

    i = 0;

    while i < quantity {
        let Some(ele) = moving_stack.pop() else { todo!() };
        stacks[landing_stack].push(ele);

        i += 1;
    }

    dbg!(quantity);
    dbg!(moving_stack);
    dbg!(stacks);
}

fn revert_and_transpose(input: &[WrappedLine]) -> Vec<WrappedLine> {
    let mut transposed_stacks = vec![];
    for i in 0..input[0].len() {
        let line: &WrappedLine = &input.iter().map(|l| l[i]).rev().collect();
        transposed_stacks.push(line.to_vec());
    }
    transposed_stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn part_one_example_works() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3";
        let result = arrange_stacks(input);
        assert_eq!(&result.unwrap(), "CMZ");
    }

    #[test]
    fn test_parsing_moves() {
        let input = "move 1 from 2 to 1";
        let (_input, result) = parse_move(input).unwrap();

        assert_eq!(result, [1, 1, 0]);
    }

    #[test]
    fn move_a_crate_from_string_input() {
        let mut stacks = vec![vec!["D", "A"], vec!["E", "B", "C"], vec![]];
        let expected = vec![vec!["D", "A"], vec!["E", "B"], vec!["C"]];
        let motion = "move 1 from 2 to 3";
        let motion = parse_move(motion).unwrap().1;
        move_crates(&mut stacks, &motion);

        assert_eq!(stacks, expected);
    }

    #[test]
    fn move_more_than_one_crate() {
        let mut stacks = vec![vec!["D", "A"], vec!["E", "B", "C"], vec![]];
        let expected = vec![vec![], vec!["E", "B", "C"], vec!["A", "D"]];

        let motion = "move 2 from 1 to 3";
        let motion = parse_move(motion).unwrap().1;
        move_crates(&mut stacks, &motion);

        assert_eq!(stacks, expected);
    }

    #[test]
    fn transposes_stacks() {
        let stacks = vec![
            vec![None, Some("C"), None],
            vec![Some("A"), Some("B"), None],
            vec![Some("D"), Some("E"), None],
        ];
        let expected = vec![
            vec![Some("D"), Some("A"), None],
            vec![Some("E"), Some("B"), Some("C")],
            vec![None, None, None],
        ];

        let stacks = revert_and_transpose(&stacks);

        assert_eq!(stacks, expected);
    }
}
