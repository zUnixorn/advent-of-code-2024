use nom::bytes::complete::tag;
use nom::{IResult, Parser};
use nom::multi::separated_list0;
use advent_of_code::{number_u32};

advent_of_code::solution!(2);

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list0(tag("\n"), separated_list0(tag(" "), number_u32.map(|n| n as i32)))(input.trim())
}

fn is_safe(line: &[i32]) -> bool {
    let mut decreasing = None;

    for pair in line.windows(2) {
        let difference = pair[1] - pair[0];
        let is_decreasing = difference.is_negative();
        let should_decrease = decreasing.get_or_insert(is_decreasing);
        let in_range = (1..=3).contains(&difference.abs());

        if is_decreasing != *should_decrease || !in_range {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, parsed) = parse(input).ok()?;

    Some(parsed.iter().filter_map(|i| is_safe(i).then_some(())).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, parsed) = parse(input).ok()?;
    let mut sum = 0;

    for line in parsed {
        for i in 0..line.len() {
            let mut dampened_line = line.clone();
            dampened_line.remove(i);

            if is_safe(&dampened_line) {
                sum += 1;
                break;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
