use regex::{Captures, Regex};
use advent_of_code::parse;

advent_of_code::solution!(3);

fn mul_capture(capture: Captures) -> u32 {
    parse!(capture[1], u32) * parse!(capture[2], u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(re.captures_iter(input).map(mul_capture).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    // unfortunately does not work on real input
    // let re = Regex::new(r"(?:(?:^|do\(\))(?:(?!don't\(\)).)*mul\((\d+),(\d+)\))").unwrap();
    // Some(re.captures_iter(input).map(process_capture).sum())

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut do_mul = true;
    let mut sum = 0;

    for capture in re.captures_iter(input) {
        match &capture[0] {
            "do()" => do_mul = true,
            "don't()" => do_mul = false,
            _ if do_mul => sum += mul_capture(capture),
            _ => (),
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
