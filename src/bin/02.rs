use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::IResult;
use nom::multi::separated_list0;

advent_of_code::solution!(2);

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    fn number_i32(input : &str) -> IResult<&str, i32> {
        map_res(recognize(digit1), str::parse)(input)
    }

    separated_list0(tag("\n"), separated_list0(tag(" "), number_i32))(input.trim())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, parsed) = parse(input).ok()?;
    let mut sum = parsed.len() as u32;

    for line in parsed {
        let mut decreasing = None;

        for pair in line.windows(2) {
            let difference = pair[1] - pair[0];
            let is_decreasing = difference.is_negative();
            let should_decrease = decreasing.get_or_insert(is_decreasing);
            let in_range = (1..=3).contains(&difference.abs());

            if is_decreasing != *should_decrease || !in_range {
                sum -= 1;
                break;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, parsed) = parse(input).ok()?;
    let mut sum = 0;

    for line in parsed {
        'outer: for i in 0..line.len() {
            let mut decreasing = None;
            let mut dampened_line = line.clone();
            dampened_line.remove(i);

            for pair in dampened_line.windows(2) {
                let difference = pair[1] - pair[0];
                let is_decreasing = difference.is_negative();
                let should_decrease = decreasing.get_or_insert(is_decreasing);
                let in_range = (1..=3).contains(&difference.abs());

                if is_decreasing != *should_decrease || !in_range {
                    continue 'outer;
                }
            }

            sum += 1;
            break;
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
