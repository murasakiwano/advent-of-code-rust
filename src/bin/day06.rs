use std::collections::HashMap;

pub fn count_till_marker(input: &str, marker_size: usize) -> usize {
    let windows = get_windows_by_size(input.chars().collect(), marker_size);
    let token_candidates = windows.iter().take_while(|&t| !unique_chars(t));

    let mut counter = 0;
    token_candidates.for_each(|_| {
        counter += 1;
    });

    counter + marker_size
}

fn get_windows_by_size(input: Vec<char>, marker_size: usize) -> Vec<Vec<char>> {
    let mut windows = vec![];

    for i in 0..input.len() - 1 - marker_size {
        windows.push(input[i..i + marker_size].to_vec())
    }

    windows
}

/// Returns true if every char is unique
pub fn unique_chars(t: &Vec<char>) -> bool {
    let mut map = HashMap::new();
    let mut res = true;

    for item in t {
        if map.insert(item, ()).is_some() {
            res = false;
            break;
        }
    }

    res
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_test_case(input: &str, expected: usize) {
        let count = count_till_marker(input, 14);

        assert_eq!(count, expected)
    }

    #[test]
    fn first_example_passes() {
        let example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_test_case(example, 19)
    }

    #[test]
    fn second_example_passes() {
        let example = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_test_case(example, 23)
    }

    #[test]
    fn third_example_passes() {
        let example = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_test_case(example, 23)
    }

    #[test]
    fn fourth_example_passes() {
        let example = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_test_case(example, 29)
    }

    #[test]
    fn fifth_example_passes() {
        let example = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_test_case(example, 26)
    }
}
