use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;

advent_of_code::solution!(1);

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    fn number_u32(input : &str) -> IResult<&str, u32> {
        map_res(recognize(digit1), str::parse)(input)
    }

    separated_list0(tag("\n"), separated_pair(number_u32, tag("   "), number_u32))(input.trim())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, parsed) = parse(input).ok()?;
    let (mut left, mut right): (Vec<_>, Vec<_>) = parsed.into_iter().unzip();

    left.sort();
    right.sort();

    Some(left.into_iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, parsed) = parse(input).ok()?;
    let (left, right): (Vec<_>, Vec<_>) = parsed.into_iter().unzip();
    let mut right_frequencies = HashMap::new();

    for i in right {
        *right_frequencies.entry(i).or_insert(0u32) += 1;
    }

    Some(left.into_iter().map(| i| i * *right_frequencies.get(&i).unwrap_or(&0)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
