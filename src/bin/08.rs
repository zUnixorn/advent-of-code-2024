use geo::{coord, Contains, Point, Rect};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::multi::{many0, separated_list0};
use nom::IResult;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list0(tag("\n"), many0(none_of("\n")))(input.trim())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, parsed) = parse(input).ok()?;
    let height = parsed.len();
    let width = parsed[0].len();
    let bound = Rect::new(
        coord! { x: -1, y: -1},
        coord! { x: width as i32, y: height as i32 }
    );

    let mut frequencies = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            if parsed[y][x] != '.' {
                frequencies
                    .entry(parsed[y][x])
                    .or_insert_with(|| HashSet::new())
                    .insert(Point::new(x as i32, y as i32));
            }
        }
    }

    let mut antinodes = HashSet::new();

    for antennas in frequencies.values() {
        for comb in antennas.iter().combinations(2) {
            antinodes.extend(
                [*comb[0] * 2 - *comb[1], *comb[1] * 2 - *comb[0]]
                    .into_iter()
                    .filter(|a| bound.contains(a))
            );
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
